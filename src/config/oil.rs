use crate::{env::gvim::keymap::KeymapOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_oil(&self) {
        self.add_packs(["https://github.com/barrettruth/canola.nvim"]);

        self.setup_plugin("oil", self.oil_opts()).ok_or_notify(self);

        self.set_keymap(
            "n",
            "<leader>fe",
            self.create_cb(|conf, ()| {
                conf.run_cmd("Oil --float");
                Ok(())
            }),
            mk_builder!(KeymapOpts, {
                desc = "Oil (Float)";
            }),
        );
        self.set_keymap(
            "n",
            "<leader>fE",
            self.create_cb(|conf, ()| {
                conf.run_cmd("Oil");
                Ok(())
            }),
            mk_builder!(KeymapOpts, {
                desc = "Oil (Buffer)";
            }),
        );
    }

    fn oil_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        tbl!(owned, {
            default_file_explorer = true;
            buf_options.buflisted = false;
            float.border = "rounded";
            delete_to_trash = true;
            prompt_save_on_select_new_entry = true;
        })
    }
}
