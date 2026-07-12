use crate::{env::gvim::lsp::VimLspConfig, prelude::*};

impl NvimConf<'_> {
    pub fn load_lua_lang(&self) {
        self.ft_set_indent("lua", 2);

        self.set_formatter("lua", ["stylua"]);
        self.formatter_use_nix("stylua", "stylua", "stylua");

        self.config_lsp(
            "emmylua_ls",
            mk_builder!(VimLspConfig, {
                cmd = self.nix_shell_cmd("emmylua-ls", ["emmylua_ls"]);
            }),
        );
    }
}
