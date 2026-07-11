use crate::{env::gvim::keymap::KeymapOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_bufferline(&self) {
        self.add_packs([
            "https://github.com/nvim-tree/nvim-web-devicons",
            "https://github.com/akinsho/bufferline.nvim",
        ]);

        self.setup_plugin("bufferline", self.bufferline_opts())
            .ok_or_notify(self);

        self.set_keymap(
            "n",
            "L",
            self.create_cb(|conf, ()| {
                conf.run_cmd("BufferLineCycleNext");
                Ok(())
            }),
            mk_builder!(KeymapOpts, {
                desc = "Next Buffer";
            }),
        );

        self.set_keymap(
            "n",
            "H",
            self.create_cb(|conf, ()| {
                conf.run_cmd("BufferLineCyclePrev");
                Ok(())
            }),
            mk_builder!(KeymapOpts, {
                desc = "Prev Buffer";
            }),
        );
    }

    fn bufferline_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        tbl!(owned, {})
    }
}
