use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_tex_lang(&self) {
        self.add_packs(["https://github.com/lervag/vimtex"]);

        tbl!(out(self.lua().globals()), {
            vim.g.vimtex_syntax_enabled = 0; // Use treesitter instead

            vim.g.tex_flavor = "latex";

            // vim.g.vimtex_view_method = "zathura"
            vim.g.vimtex_view_method = "general";
            vim.g.vimtex_view_general_viewer = "kitty";
            vim.g.vimtex_view_general_options = "tdf --reload-delay 2000 @pdf";
        })
        .ok_or_notify(self);

        self.ts_install_parser("latex");
        self.set_formatter("tex", ["latexindent"]);
    }
}
