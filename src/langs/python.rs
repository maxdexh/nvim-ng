use crate::{env::gvim::lsp::VimLspConfig, prelude::*};

impl NvimConf<'_> {
    pub fn load_python_lang(&self) {
        self.config_lsp(
            "basedpyright",
            mk_builder!(VimLspConfig, {
                cmd = self.nix_shell_cmd("basedpyright", ["basedpyright-langserver", "--stdio"]);
                settings = tbl!(owned, {
                    basedpyright = tbl!(owned, {
                        disableOrganizeImports = true; // Using Ruff
                        analysis = tbl!(owned, {
                            ignore = { "*" }; // Using Ruff
                            typeCheckingMode = "off"; // using mypy
                        });
                    });
                });
            }),
        );
        self.config_lsp(
            "ruff",
            mk_builder!(VimLspConfig, {
                cmd = self.nix_shell_cmd("ruff", ["ruff"]);
            }),
        );
    }
}

// TODO: mypy
//   {
//      "nvimtools/none-ls.nvim",
//      opts = function(_, opts)
//         local nls = require("null-ls")
//         opts.sources = {
//            nls.builtins.diagnostics.mypy,
//         }
//         opts.should_attach = function(bufnr)
//            return vim.api.nvim_buf_get_name(bufnr):match(".py$")
//         end
//      end,
//   }
