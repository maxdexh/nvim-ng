use crate::{env::gvim::api::AutoCmdOpts, prelude::*};

impl NvimConf<'_> {
    pub fn ts_install_parser(&self, s: impl LuaSub<LuaString>) {
        self.req_treesitter()
            .and_then(|ts| ts.install()?.call([s]))
            .ok_or_notify(self.env());
    }
    pub fn ft_set_indent(
        &self,
        ft: impl LuaSub<LuaString>,
        indent: impl LuaSub<LuaInt> + 'static + Send + Copy,
    ) {
        self.add_autocmd(
            "FileType",
            mk_builder!(AutoCmdOpts, {
                pattern = Some(ft);
                callback = self.create_cb(move |conf, ()| {
                    tbl!(out(conf.lua().globals()), {
                        vim.opt_local.shiftwidth = 0;
                        vim.opt_local.tabstop = indent;
                        vim.opt_local.expandtab = true;
                    })
                    .map(|_| ())
                });
            }),
        );
    }
    pub fn set_formatter(&self, ft: impl LuaSub<LuaString>, table: impl LuaSub<LuaSeq<LuaString>>) {
        do_try(|| {
            let table = lua_conv_sub(self.lua(), table)?;
            let ft = lua_conv_sub(self.lua(), ft)?;

            self.add_autocmd(
                "FileType",
                mk_builder!(AutoCmdOpts, {
                    once = true;
                    pattern = ft.clone();
                    callback = self.create_cb_once(move |env, ()| {
                        env.req_conform()
                            .and_then(|conform| conform.formatters_by_ft()?.set(ft, table))
                    });
                }),
            );

            Ok(())
        })
        .ok_or_notify(self);
    }

    pub fn version_range(&self, arg: impl LuaSub<LuaString>) -> Result<LuaVal> {
        self.env().globals.vim()?.version()?.range()?.call(arg)
    }
}
