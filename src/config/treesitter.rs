use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct TSPlugin {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        //install: LuaCallable<LuaSeq<LuaString>, ()>,
    }
});

impl NvimConf<'_> {
    fn req_ts(&self) -> Result<TSPlugin> {
        self.setup_plugin::<TSPlugin>("nvim-treesitter", |ts| {
            ts.setup()?.call(tbl!(owned, {
                auto_install = true;
                highlight.enable = true;
                indent = tbl!(owned, {
                    enable = true;
                    disable = ["python", "css", "rust"];
                });
            }))
        })
    }
    pub fn ts_install_parser(&self, _: impl LuaSub<LuaString>) {
        //self.req_treesitter()
        //    .and_then(|ts| ts.install()?.call([s]))
        //    .ok_or_notify(self.env());
    }
    pub fn load_treesitter(&self) {
        self.add_packs(["https://github.com/nvim-treesitter/nvim-treesitter"]);
        // TODO: Lazy
        self.req_ts().ok_or_notify(self);
    }
}
