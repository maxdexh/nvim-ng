use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_options(&self) {
        let env = self.env();

        do_try(|| {
            tbl!(out(env.globals.vim()?.opt()?), {
                shiftwidth = 0;
                tabstop = 2;
                expandtab = true;
                number = true;
                relativenumber = true;
                undofile = true;
            })
        })
        .ok_or_notify(env);

        do_try(|| {
            tbl!(out(env.globals.vim()?.g()?), {
                snacks_animate = false;
                mapleader = " ";
            })
        })
        .ok_or_notify(env);

        do_try(|| {
            env.globals.vim()?.diagnostic()?.config()?.call(tbl!({
                virtual_text.severity.min = 2; // 2 = warn
            }))
        })
        .ok_or_notify(env);
    }
}
