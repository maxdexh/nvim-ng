use crate::{env::gvim::api::AutoCmdOpts, prelude::*};

impl NvimConf<'_> {
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
                    conf.with_vim_opt_local(|opt_local| {
                        tbl!(out(opt_local), {
                            shiftwidth = 0;
                            tabstop = indent;
                            expandtab = true;
                        })
                    })
                    .map(|_| ())
                });
            }),
        );
    }

    pub fn version_range(&self, arg: impl LuaSub<LuaString>) -> Result<LuaVal> {
        self.env().globals.vim()?.version()?.range()?.call(arg)
    }
}
