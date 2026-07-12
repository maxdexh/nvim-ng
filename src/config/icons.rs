use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct MiniIcons {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        mock_nvim_web_devicons: LuaCallable<(), ()>,
    }
});

impl NvimConf<'_> {
    fn req_mini_icons(&self) -> Result<MiniIcons> {
        self.setup_plugin::<MiniIcons>("mini.icons", |plug| {
            plug.setup()?.call(tbl!(owned, {}))?;
            plug.mock_nvim_web_devicons()?.call(())
        })
    }
    pub fn load_icons(&self) {
        self.add_packs(["https://github.com/nvim-mini/mini.icons"]);
        self.on_very_lazy(|conf| {
            conf.req_mini_icons()?;
            Ok(())
        })
        .ok_or_notify(self);
    }
}
