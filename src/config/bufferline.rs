use crate::{env::vim::keymap::KeymapOpts, plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_bufferline(&self) {
        let env = self.env();

        do_try(|| {
            env.globals.vim()?.pack()?.add()?.call([
                "https://github.com/nvim-tree/nvim-web-devicons",
                "https://github.com/akinsho/bufferline.nvim",
            ])
        })
        .ok_or_notify(env);

        do_try(|| {
            env.globals
                .require()?
                .call_any_ret::<GenericPlugin>("bufferline")
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
            KeymapOpts::empty().with_desc("Next Buffer").finish(),
        );

        self.set_keymap(
            "n",
            "H",
            self.create_cb(|conf, ()| {
                conf.run_cmd("BufferLineCyclePrev");
                Ok(())
            }),
            KeymapOpts::empty().with_desc("Prev Buffer").finish(),
        );
    }

    fn bufferline_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        tbl!({})
    }
}
