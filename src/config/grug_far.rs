use crate::{env::gvim::keymap::KeymapOpts, prelude::*};

crate::utils::from_tbl_proxy!({
    struct GrugFar {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        open: LuaCallable<LuaDict<LuaVal>, ()>,
    }
});

impl NvimConf<'_> {
    fn req_grug_far(&self) -> Result<GrugFar> {
        self.setup_plugin::<GrugFar>("grug-far", |gf| gf.setup()?.call(tbl!(owned, {})))
    }
    pub fn load_grug_far(&self) {
        self.add_packs(["https://github.com/MagicDuck/grug-far.nvim"]);

        self.set_keymap(
            ["n", "x"],
            "<leader>sr",
            self.create_cb(|conf, ()| {
                // TODO: prefill selected text
                conf.req_grug_far()?.open()?.call(tbl!(owned, {
                    transient = true;
                }))
            }),
            mk_builder!(KeymapOpts, {
                desc = "Search and Replace";
            }),
        );
    }
}
