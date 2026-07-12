use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct GitSigns {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
    }
});

fn gitsigns_opts(_: &NvimConf) -> impl LuaSub<LuaDict<LuaVal>> {
    tbl!(owned, {
        signs = tbl!(owned, {
            add.text = "▎";
            change.text = "▎";
            delete.text = "";
            topdelete.text = "";
            changedelete.text = "▎";
            untracked.text = "▎";
        });
        signs_staged = tbl!(owned, {
            add.text = "▎";
            change.text = "▎";
            delete.text = "";
            topdelete.text = "";
            changedelete.text = "▎";
        });
    })
}

impl NvimConf<'_> {
    fn req_gitsigns(&self) -> Result<GitSigns> {
        self.setup_plugin::<GitSigns>("gitsigns", |gs| gs.setup()?.call(gitsigns_opts(self)))
    }
    pub fn load_gitsigns(&self) {
        self.add_packs(["https://github.com/lewis6991/gitsigns.nvim"]);
        self.on_very_lazy(|conf| {
            conf.req_gitsigns()?;
            Ok(())
        })
        .ok_or_notify(self);
    }
}
