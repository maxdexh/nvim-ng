use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_persistence(&self) {
        let env = self.env();
        env.vim()
            .pack()
            .add_one("https://github.com/folke/persistence.nvim");

        env.req_persistence()
            .and_then(|pers| pers.setup()?.call(tbl!({})))
            .ok_or_notify(env);
    }
}
