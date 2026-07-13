use crate::{env::gvim::lsp::VimLspConfig, prelude::*};

impl NvimConf<'_> {
    pub fn load_nix_lang(&self) {
        self.ft_set_indent("nix", 2);

        let on_init = self.create_cb(|_, client: LuaDictMut<LuaVal>| {
            let client = client.into_table_any();

            // disable all capabilities except the ones not provided by nixd
            {
                let caps: LuaTableAny = client.get_any("capabilities")?;
                let td: LuaTableAny = caps.get_any("textDocument")?;

                client.set(
                    "capabilities",
                    tbl!(owned, {
                        workspace.semanticTokens = caps
                            .get_any::<LuaTableAny>("workspace")?
                            .get_any::<LuaVal>("semanticTokens")?;

                        textDocument = tbl!(owned, {
                            documentHighlight = td.get_any::<LuaVal>("documentHighlight")?;
                            semanticTokens = td.get_any::<LuaVal>("semanticTokens")?;
                        });
                    }),
                )?;
            }
            {
                let caps: LuaTableAny = client.get_any("server_capabilities")?;
                client.set(
                    "server_capabilities",
                    tbl!(owned, {
                        semanticTokensProvider =
                            caps.get_any::<LuaVal>("semanticTokensProvider")?;
                        documentHighlightProvider =
                            caps.get_any::<LuaVal>("documentHighlightProvider")?;
                        textDocumentSync = caps.get_any::<LuaVal>("textDocumentSync")?;
                    }),
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
