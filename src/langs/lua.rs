use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_lua_lang(&self) {
        self.ft_set_indent("lua", 2);
        self.ts_install_lang("lua");
        self.set_formatter("lua", ["stylua"]);
    }
}
