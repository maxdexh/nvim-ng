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

        let Some(snacks) = env.req_snacks().ok_or_notify(env) else {
            return;
        };
        do_try(|| snacks.setup()?.call(self.snacks_opts())).ok_or_notify(env);
    }

    fn snacks_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        let env = self.env();
        let find_file =
            env.create_func(|env, ()| env.req_snacks()?.dashboard()?.pick()?.call("files"));
        let find_text =
            env.create_func(|env, ()| env.req_snacks()?.dashboard()?.pick()?.call("live_grep"));
        let load_session = env.create_func(|env, ()| {
            env.req_persistence()?.load()?.call(tbl!({
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
