use crate::{env::vim::api::AutoCmdOpts, prelude::*};

impl NvimConf<'_> {
    pub fn ts_install_lang(&self, s: &str) {
        self.req_treesitter()
            .and_then(|ts| ts.install()?.call([s]))
            .ok_or_notify(self.env());
    }
    pub fn ft_set_indent(&self, ft: &str, indent: u8) {
        let cb = self.create_cb(move |conf, ()| {
            tbl!(out(&conf.env().globals.vim()?.opt_local()?), {
                shiftwidth = 0;
                tabstop = indent;
                expandtab = true;
            })
        });
        self.add_autocmd(
            "FileType",
            AutoCmdOpts::empty()
                .with_pattern(ft)
                .with_callback(cb)
                .finish(),
        );
    }
    pub fn set_formatter(&self, ft: impl LuaSub<LuaString>, table: impl LuaSub<LuaTableAny>) {
        do_try(|| {
            let table = lua_conv_sub(self.lua(), table)?;
            let ft = lua_conv_sub(self.lua(), ft)?;

            self.add_autocmd(
                "FileType",
                AutoCmdOpts::empty()
                    .with_once(true)
                    .with_pattern(ft.clone())
                    .with_callback(self.create_cb_once(move |env, ()| {
                        env.req_conform()
                            .and_then(|conform| conform.formatters_by_ft()?.set(ft, table))
                    }))
                    .finish(),
            );

            Ok(())
        })
        .ok_or_notify(self.env());
    }
}
