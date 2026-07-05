use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_options(&self) {
        let env = self.env();

        env.vim().init_opt(|mut builder| {
            tbl!(builder = builder, {
                shiftwidth = 0;
                tabstop = 2;
                expandtab = true;
                number = true;
                relativenumber = true;
                undofile = true;
            })
        });

        env.vim().init_g(|mut builder| {
            tbl!(builder = builder, {
                snacks_animate = false;
                // NOTE: Must be set before any leader binds
                mapleader = " ";
            })
        });

        do_try(|| {
            env.globals.vim()?.diagnostic()?.config()?.call(tbl!({
                virtual_text.severity.min = 2; // 2 = warn
            }))
        })
        .ok_or_notify(env);
    }
}
