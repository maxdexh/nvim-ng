use crate::prelude::*;

impl Nvim {
    pub fn load_init(&self) {
        let conf = self.config();
        conf.load_options();
        conf.load_keybinds();

        conf.load_snacks();
        conf.load_persistence();
        conf.load_colorscheme();

        conf.load_completions();
        conf.load_bufferline();
        conf.load_conform();
        conf.load_oil();

        conf.load_nix_lang();
        conf.load_rust_lang();
        conf.load_lua_lang();
    }
}
