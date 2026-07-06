use crate::prelude::*;

impl NvimConf<'_> {
    pub fn load_snacks(&self) {
        let env = self.env();
        env.vim()
            .pack()
            .add("https://github.com/folke/snacks.nvim", PackOpts::empty());

        let Some(snacks) = env.req_snacks().ok_or_notify(env) else {
            return;
        };
        self.snacks_opts()
            .and_then(|opts| snacks.setup()?.call(opts))
            .ok_or_notify(env);
    }
    fn snacks_opts(&self) -> Result<LuaTopTable> {
        let env = self.env();
        tbl!(eval(self.lua()), {
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
                        action = env.create_func(|env, ()| {
                            env.req_snacks()?.dashboard()?.pick()?.call("files")
                        });
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
                        action = env.create_func(|env, ()| {
                            env.req_snacks()?.dashboard()?.pick()?.call("live_grep")
                        });
                    }),
                    tbl!({
                        icon = " ";
                        key = "s";
                        desc = "Restore Session";
                        action = env.create_func(|env, ()| {
                            env.req_persistence()?.load()?.call(tbl!({
                                last = true;
                            }))
                        });
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
