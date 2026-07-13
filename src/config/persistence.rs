use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct Persistence {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        load: LuaCallable<Option<LuaDict<LuaVal>>, ()>,
    }
});

impl NvimConf<'_> {
    pub fn req_persistence(&self) -> Result<Persistence> {
        self.setup_plugin::<Persistence>("persistence", |pers| pers.setup()?.call(tbl!(owned, {})))
    }

    pub fn load_persistence(&self) {
        self.add_packs(["https://github.com/folke/persistence.nvim"]);

        self.req_persistence().ok_or_notify(self);
    }
}
