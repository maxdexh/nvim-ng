use crate::{env::gvim::lsp::VimLspConfig, prelude::*};

impl NvimConf<'_> {
    pub fn load_nix_lang(&self) {
        self.ft_set_indent("nix", 2);

        let on_init = self.create_cb(|conf, client: LuaDictMut<LuaVal>| {
            let client = client.into_table_any();

            // disable all capabilities except the ones not provided by nixd
            let lua = conf.lua();
            {
                let caps_key = lua.create_string("capabilities")?;
                let td_key = lua.create_string("textDocument")?;
                let sem_tok_key = lua.create_string("semanticTokens")?;
                let doc_hl_key = lua.create_string("documentHighlight")?;
                let ws_key = lua.create_string("workspace")?;

                let caps: LuaTableAny = client.get_any(caps_key.clone())?;
                let td: LuaTableAny = caps.get_any(td_key.clone())?;

                client.set_any(
                    caps_key,
                    tbl!(owned, {
                        [ws_key.clone()] = tbl!(owned, {
                            [sem_tok_key.clone()] = caps
                                .get_any::<LuaTableAny>(ws_key)?
                                .get_any::<LuaVal>(sem_tok_key.clone())?;
                        });

                        [td_key] = tbl!(owned, {
                            [doc_hl_key.clone()] = td.get_any::<LuaVal>(doc_hl_key)?;
                            [sem_tok_key.clone()] = td.get_any::<LuaVal>(sem_tok_key)?;
                        });
                    }),
                )?;
            }
            {
                let scap_key = lua.create_string("server_capabilities")?;
                let caps: LuaTableAny = client.get_any(scap_key.clone())?;
                client.set_any(
                    scap_key,
                    lua.create_table_from(
                        [
                            lua.create_string("semanticTokensProvider")?,
                            lua.create_string("documentHighlightProvider")?,
                            lua.create_string("textDocumentSync")?,
                        ]
                        .map(|key| (key.clone(), caps.get_any::<LuaVal>(key))),
                    ),
                )?;
            }

            Ok(())
        });
        self.config_lsp(
            "nil_ls",
            mk_builder!(VimLspConfig, {
                cmd = self.nix_shell_cmd("nil", ["nil"]);
                settings = tbl!(owned, {
                    nil = tbl!(owned, {
                        diagnostics = tbl!(owned, {
                            ignored = ["let_attrset"];
                        });
                        nix = tbl!(owned, {
                            flake = tbl!(owned, {
                                autoArchive = false;
                            });
                        });
                    });
                });
                on_init = on_init;
            }),
        );

        let nixd_opts = do_try(|| {
            if !std::env::current_dir().is_ok_and(|cwd| {
                std::env::var_os("NIXOS_FLAKE").is_some_and(|fl| cwd.starts_with(fl))
            }) {
                return Ok(None);
            }
            let Some(config_name) = std::env::var_os("NVIM_NIX_HOST_NAME") else {
                return Ok(None);
            };
            let flake_expr = b"(builtins.getFlake (builtins.toString ./.))" as &[_];

            let out = LuaDictMut::<LuaVal>::new(self.lua())?;

            if let Some(username) = std::env::var_os("USER") {
                let expr = [
                    flake_expr,
                    b".homeConfigurations.\"",
                    username.as_encoded_bytes(),
                    b"@",
                    config_name.as_encoded_bytes(),
                    b"\".options",
                ]
                .concat();
                out.set(
                    "home-manager",
                    tbl!(owned, {
                        expr = self.lua().create_string(expr)?;
                    }),
                )?;
            }

            if std::env::var_os("NVIM_NIX_IS_NIXOS").is_some_and(|it| it == "1") {
                let expr = [
                    flake_expr,
                    b".nixosConfigurations.\"",
                    config_name.as_encoded_bytes(),
                    b"\".options",
                ]
                .concat();
                out.set(
                    "nixos",
                    tbl!(owned, {
                        expr = self.lua().create_string(expr)?;
                    }),
                )?;
            }

            Ok(Some(out))
        })
        .ok_or_notify(self)
        .flatten();

        self.config_lsp(
            "nixd",
            mk_builder!(VimLspConfig, {
                cmd = self.nix_shell_cmd("nixd", ["nixd"]);
                settings = tbl!(owned, {
                    nixd.options = nixd_opts;
                });
            }),
        );

        self.set_formatter("nix", ["alejandra"]);
        self.formatter_use_nix("alejandra", "alejandra", "alejandra");
    }
}
