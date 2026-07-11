use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_nix_lang(&self) {
        self.ft_set_indent("nix", 2);
        self.ts_install_parser("nix");
    }
}
