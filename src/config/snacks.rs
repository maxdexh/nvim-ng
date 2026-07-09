use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_snacks(&self) {
        let env = self.env();
        do_try(|| {
            env.globals
                .vim()?
                .pack()?
                .add()?
                .call(["https://github.com/folke/snacks.nvim"])
        })
        .ok_or_notify(env);

        let Some(snacks) = self.req_snacks().ok_or_notify(env) else {
            return;
        };
        do_try(|| snacks.setup()?.call(self.snacks_opts())).ok_or_notify(env);
    }

    fn snacks_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        let find_file =
            self.create_cb(|conf, ()| conf.req_snacks()?.dashboard()?.pick()?.call("files"));
        let find_text =
            self.create_cb(|conf, ()| conf.req_snacks()?.dashboard()?.pick()?.call("live_grep"));
        let load_session = self.create_cb(|conf, ()| {
            conf.req_persistence()?.load()?.call(tbl!({
                last = true;
            }))
        });
        tbl!({
            bigfile.enabled = true;
            indent.enabled = true;
            input.enabled = true;
            notifier.enabled = true;
            quickfile.enabled = true;
            scope.enabled = true;
            scroll.enabled = true;
            statuscolumn.enabled = false;
            words.enabled = true;
            picker.ui_select = true;
            dashboard = tbl!({
                enabled = true;
                preset.keys = tbl_seq![
                    tbl!({
                        icon = " ";
                        key = "f";
                        desc = "Find File";
                        action = find_file;
                    }),
                    tbl!({
                        icon = " ";
                        key = "n";
                        desc = "New File";
                        action = ":ene | startinsert";
                    }),
                    tbl!({
                        icon = " ";
                        key = "g";
                        desc = "Find Text";
                        action = find_text;
                    }),
                    tbl!({
                        icon = " ";
                        key = "s";
                        desc = "Restore Session";
                        action = load_session;
                    }),
                    tbl!({
                        icon = " ";
                        key = "q";
                        desc = "Quit";
                        action = ":qa";
                    }),
                ];
                sections = tbl_seq![
                    tbl!({
                        section = "header";
                    }),
                    tbl!({
                        section = "keys";
                        gap = 1;
                        padding = 1;
                    }),
                ];
            });
        })
    }
}
