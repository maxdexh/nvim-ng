use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_options(&self) {
        let env = self.env();

        tbl!(out(self.lua().globals()), {
            vim.opt.shiftwidth = 0;
            vim.opt.tabstop = 2;
            vim.opt.expandtab = true;
            vim.opt.number = true;
            vim.opt.relativenumber = true;
            vim.opt.undofile = true;
        })
        .ok_or_notify(env);

        tbl!(out(self.lua().globals()), {
            snacks_animate = false;
            mapleader = " ";
        })
        .ok_or_notify(env);

        do_try(|| {
            env.globals
                .vim()?
                .diagnostic()?
                .config()?
                .call(tbl!(owned, {
                    virtual_text.severity.min = 2; // 2 = warn
                }))
        })
        .ok_or_notify(env);
    }
}
