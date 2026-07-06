use crate::{plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_oil(&self) {
        self.env().vim().pack().add(
            "https://github.com/barrettruth/canola.nvim",
            PackOpts::empty(),
        );

        self.env()
            .call_require::<GenericPlugin>("oil")
            .and_then(|oil| oil.setup()?.call(self.oil_opts()))
            .ok_or_notify(self.env());

        let keymap = self.keymap();
        keymap.set(["n"], "<leader>fe", "Oil (Float)", |env| {
            env.vim().run_cmd("Oil --float");
            Ok(())
        });
        keymap.set(["n"], "<leader>fE", "Oil (Buffer)", |env| {
            env.vim().run_cmd("Oil");
            Ok(())
        });
    }

    fn oil_opts(&self) -> impl LuaSub<LuaTopTable> {
        tbl!({
            default_file_explorer = true;
            buf_options.buflisted = false;
            float.border = "rounded";
            delete_to_trash = true;
            prompt_save_on_select_new_entry = true;
        })
    }
}
