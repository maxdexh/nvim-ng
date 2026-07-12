use crate::{env::gvim::lsp::VimLspConfig, prelude::*};

impl NvimConf<'_> {
    pub fn load_nix_lang(&self) {
        self.ft_set_indent("nix", 2);

        let on_init = self.create_cb(|_, client: LuaDictMut<LuaVal>| {
            let client = client.into_table_any();

            // disable all capabilities except the ones not provided by nixd
            {
                let caps: mlua::Table = client.get("capabilities")?;
                let td: mlua::Table = caps.get("textDocument")?;

                client.set(
                    "capabilities",
                    tbl!(owned, {
                        workspace.semanticTokens = caps
                            .get::<mlua::Table>("workspace")?
                            .get::<LuaVal>("semanticTokens")?;

                        textDocument = tbl!(owned, {
                            documentHighlight = td.get::<LuaVal>("documentHighlight")?;
                            semanticTokens = td.get::<LuaVal>("semanticTokens")?;
                        });
                    }),
                )?;
            }
            {
                let caps: mlua::Table = client.get("server_capabilities")?;
                client.set(
                    "server_capabilities",
                    tbl!(owned, {
                        semanticTokensProvider = caps.get::<LuaVal>("semanticTokensProvider")?;
                        documentHighlightProvider =
                            caps.get::<LuaVal>("documentHighlightProvider")?;
                        textDocumentSync = caps.get::<LuaVal>("textDocumentSync")?;
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
            let flake_expr = "(builtins.getFlake (builtins.toString ./.))";

            let out = LuaDictMut::<LuaVal>::new(self.lua())?;

            if let Some(username) = std::env::var_os("USER") {
                let mut expr = flake_expr.as_bytes().to_vec();
                expr.extend_from_slice(b".homeConfigurations.\"");
                expr.extend_from_slice(username.as_encoded_bytes());
                expr.extend_from_slice(b"@");
                expr.extend_from_slice(config_name.as_encoded_bytes());
                expr.extend_from_slice(b"\".options");
                out.set(
                    "home-manager",
                    tbl!(owned, {
                        expr = self.lua().create_string(expr)?;
                    }),
                )?;
            }

            if std::env::var_os("NVIM_NIX_IS_NIXOS").is_some_and(|it| it == "1") {
                let mut expr = flake_expr.as_bytes().to_vec();
                expr.extend_from_slice(b".nixosConfigurations.\"");
                expr.extend_from_slice(config_name.as_encoded_bytes());
                expr.extend_from_slice(b"\".options");
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
