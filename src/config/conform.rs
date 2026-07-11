use crate::{env::gvim::api::AutoCmdOpts, prelude::*};

crate::utils::from_tbl_proxy!({
    struct Conform {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        formatters_by_ft: LuaDictMut<LuaSeq<LuaString>>,
    }
});

impl NvimConf<'_> {
    pub fn load_conform(&self) {
        self.add_packs(["https://github.com/stevearc/conform.nvim"]);
    }
    fn req_conform(&self) -> Result<Conform> {
        self.setup_plugin::<Conform>("conform", |conform| {
            conform.setup()?.call(tbl!(owned, {
                format_on_save = tbl!(owned, {
                    timeout_ms = 500;
                    lsp_format = "fallback";
                });
            }))
        })
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
                    callback = self.create_cb_once(move |conf, ()| {
                        let conform = conf.req_conform()?;
                        conform.formatters_by_ft()?.set(ft, table)
                    });
                }),
            );

            Ok(())
        })
        .ok_or_notify(self);
    }
}
