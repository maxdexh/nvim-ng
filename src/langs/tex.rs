use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_tex_lang(&self) {
        self.add_packs(["https://github.com/lervag/vimtex"]);

        self.with_vim_g(|g| {
            tbl!(out(g), {
                vimtex_syntax_enabled = 0; // Use treesitter instead

                tex_flavor = "latex";

                // vimtex_view_method = "zathura"
                vimtex_view_method = "general";
                vimtex_view_general_viewer = "kitty";
                vimtex_view_general_options = "tdf --reload-delay 2000 @pdf";
            })
        })
        .ok_or_notify(self);

        self.set_formatter("tex", ["latexindent"]);
    }
}
