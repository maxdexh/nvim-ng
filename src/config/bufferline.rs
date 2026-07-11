use crate::{env::gvim::keymap::KeymapOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_bufferline(&self) {
        self.add_packs([
            "https://github.com/nvim-tree/nvim-web-devicons",
            "https://github.com/akinsho/bufferline.nvim",
        ]);

        self.on_very_lazy(|conf| conf.setup_plugin_now("bufferline", conf.bufferline_opts()))
            .ok_or_notify(self);

        self.set_keymap(
            "n",
            "L",
            "<CMD>BufferLineCycleNext<Enter>",
            mk_builder!(KeymapOpts, {
                desc = "Next Buffer";
            }),
        );

        self.set_keymap(
            "n",
            "H",
            "<CMD>BufferLineCyclePrev<Enter>",
            mk_builder!(KeymapOpts, {
                desc = "Prev Buffer";
            }),
        );
    }

    fn bufferline_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        tbl!(owned, {})
    }
}
