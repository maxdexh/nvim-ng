use crate::{env::vim::keymap::KeymapOpts, plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_bufferline(&self) {
        let env = self.env();

        self.add_packs([
            "https://github.com/nvim-tree/nvim-web-devicons",
            "https://github.com/akinsho/bufferline.nvim",
        ]);

        do_try(|| {
            self.call_require::<GenericPlugin>("bufferline")
                .and_then(|bl| bl.setup()?.call(self.bufferline_opts()))
        })
        .ok_or_notify(env);

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
        tbl!({})
    }
}
