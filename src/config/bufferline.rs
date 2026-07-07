use crate::{plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_bufferline(&self) {
        let env = self.env();
        env.vim().pack().add(
            "https://github.com/nvim-tree/nvim-web-devicons",
            PackOpts::empty(),
        );

        env.vim().pack().add(
            "https://github.com/akinsho/bufferline.nvim",
            PackOpts::empty(),
        );

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

    fn bufferline_opts(&self) -> impl LuaSub<LuaTableAny> {
        tbl!({})
    }
}
