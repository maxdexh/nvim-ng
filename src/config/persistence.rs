use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_persistence(&self) {
        let env = self.env();
        do_try(|| {
            env.globals
                .vim()?
                .pack()?
                .add()?
                .call(["https://github.com/folke/persistence.nvim"])
        })
        .ok_or_notify(env);

        self.req_persistence()
            .and_then(|pers| pers.setup()?.call(tbl!({})))
            .ok_or_notify(env);
    }
}
