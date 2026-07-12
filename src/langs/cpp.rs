use crate::{env::gvim::lsp::VimLspConfig, prelude::*};

impl NvimConf<'_> {
    pub fn load_cpp_lang(&self) {
        for ft in ["cpp", "c", "h"] {
            self.ft_set_indent(ft, 4);
        }

        self.config_lsp(
            "clangd",
            mk_builder!(VimLspConfig, {
                cmd = self.nix_shell_cmd(
                    "clang-tools",
                    ["clangd", "--query-driver=/nix/store/*/bin/*"],
                );
            }),
        );
    }
}
