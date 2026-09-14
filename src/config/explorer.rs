use crate::{env::gvim::keymap::KeymapOpts, prelude::*};

impl NvimConf<'_> {
    pub fn load_oil(&self) {
        if !self.is_vscode() {
            self.add_packs(["https://github.com/barrettruth/canola.nvim"]);

            self.setup_plugin_now("oil", self.oil_opts())
                .ok_or_notify(self);
        }

        self.set_keymap(
            "n",
            "<leader>fe",
            if self.is_vscode() {
                LuaUnion::Left("<CMD>call VSCodeNotify('workbench.view.explorer')<CR>")
            } else {
                LuaUnion::Right(self.mk_callback(|conf, ()| {
                    conf.run_cmd("Oil --float");
                    Ok(())
                }))
            },
            mk_builder!(KeymapOpts, {
                desc = "Oil (Float)";
            }),
        );
        if !self.is_vscode() {
            self.set_keymap(
                "n",
                "<leader>fE",
                self.mk_callback(|conf, ()| {
                    conf.run_cmd("Oil");
                    Ok(())
                }),
                mk_builder!(KeymapOpts, {
                    desc = "Oil (Buffer)";
                }),
            );
        }
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
