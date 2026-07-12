use crate::prelude::*;

// TODO:
// - Git diff on lhs
impl Nvim {
    pub fn load_init(&self) {
        let conf = self.conf();

        conf.load_options();
        conf.load_keybinds();

        conf.load_snacks();
        conf.load_persistence();

        conf.load_icons();

        conf.load_treesitter();
        conf.load_colorscheme();
        conf.load_noice();

        conf.load_delims();
        conf.load_multicursor();

        conf.load_whichkey();
        conf.load_completions();
        conf.load_bufferline();
        conf.load_conform();
        conf.load_oil();

        conf.load_nix_lang();
        conf.load_rust_lang();
        conf.load_lua_lang();
        conf.load_tex_lang();
    }
}
