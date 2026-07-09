use crate::{plugins::GenericPlugin, prelude::*};

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

        self.keymap().set(["n"], "L", "Next Buffer", |env| {
            env.vim().run_cmd("BufferLineCycleNext");
            Ok(())
        });

        self.keymap().set(["n"], "H", "Prev Buffer", |env| {
            env.vim().run_cmd("BufferLineCyclePrev");
            Ok(())
        });
    }

    fn bufferline_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        tbl!({})
    }
}
