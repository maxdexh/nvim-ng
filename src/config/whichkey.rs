use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct WhichKey {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
    }
});

impl NvimConf<'_> {
    fn req_whichkey(&self) -> Result<WhichKey> {
        self.setup_plugin::<WhichKey>("which-key", |wk| {
            wk.setup()?.call(tbl!(owned, {
                preset = "helix";
            }))
        })
    }

    pub fn load_whichkey(&self) {
        self.add_packs(["https://github.com/folke/which-key.nvim"]);
        self.on_very_lazy(|conf| {
            conf.req_whichkey()?;
            Ok(())
        })
        .ok_or_notify(self);
    }
}
