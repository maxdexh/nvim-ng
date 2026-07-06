use crate::prelude::*;

impl NvimConf<'_> {
    pub fn ts_install_lang(&self, s: &str) {
        self.env()
            .req_treesitter()
            .and_then(|ts| ts.install()?.call([s]))
            .ok_or_notify(self.env());
    }
    pub fn ft_set_indent(&self, ft: &str, indent: u8) {
        let env = self.env();
        let cb = env.create_autocmd_cb(move |env, ()| {
            env.vim().init_opt_local(|mut builder| {
                tbl!(builder = builder, {
                    shiftwidth = 0;
                    tabstop = indent;
                    expandtab = true;
                })
            });
            Ok(())
        });
        env.vim()
            .add_autocmd("FileType", AutoCmdOpts::empty().with_pattern(ft), cb);
    }

    pub fn set_formatter(&self, ft: impl LuaSub<LuaString>, table: impl LuaSub<LuaTopTable>) {
        let env = self.env();
        mlua::Result::Ok(())
            .and_then(|()| {
                let table = lua_conv_sub(self.lua(), table)?;
                let ft = lua_conv_sub(self.lua(), ft)?;

                env.vim().add_autocmd(
                    "FileType",
                    AutoCmdOpts::empty()
                        .with_once(true)
                        .with_pattern(ft.clone()),
                    env.create_autocmd_cb_once(move |env, ()| {
                        env.req_conform()
                            .and_then(|conform| conform.formatters_by_ft()?.set(ft, table))
                            .ok_or_notify(env);
                        Ok(())
                    }),
                );

                Ok(())
            })
            .ok_or_notify(env);
    }
}
