use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_options(&self) {
        self.with_vim_opt(|opt| {
            tbl!(out(opt), {
                shiftwidth = 0;
                tabstop = 2;
                expandtab = true;
                number = true;
                relativenumber = true;
                undofile = true;
            })
        })
        .ok_or_notify(self);

        self.with_vim_g(|g| {
            tbl!(out(g), {
                snacks_animate = false;
                mapleader = " ";
            })
        })
        .ok_or_notify(self);

        do_try(|| {
            self.env()
                .globals
                .vim()?
                .diagnostic()?
                .config()?
                .call(tbl!(owned, {
                    virtual_text.severity.min = 2; // 2 = warn
                }))
        })
        .ok_or_notify(self);
    }
}
