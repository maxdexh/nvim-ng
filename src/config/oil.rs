use crate::{env::vim::keymap::KeymapOpts, plugins::GenericPlugin, prelude::*};

impl NvimConf<'_> {
    pub fn load_oil(&self) {
        let env = self.env();
        do_try(|| {
            env.globals
                .vim()?
                .pack()?
                .add()?
                .call(["https://github.com/barrettruth/canola.nvim"])
        })
        .ok_or_notify(env);

        self.call_require::<GenericPlugin>("oil")
            .and_then(|oil| oil.setup()?.call(self.oil_opts()))
            .ok_or_notify(self.env());

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
        tbl!({
            default_file_explorer = true;
            buf_options.buflisted = false;
            float.border = "rounded";
            delete_to_trash = true;
            prompt_save_on_select_new_entry = true;
        })
    }
}
