use crate::{prelude::*, utils::from_tbl_proxy};

from_tbl_proxy!({
    struct SnacksDash {
        pick: LuaCallable<LuaString, ()>,
    }
});
from_tbl_proxy!({
    struct SnacksGit {
        get_root: LuaCallable<(), Option<LuaString>>,
    }
});
from_tbl_proxy!({
    struct Snacks {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        git: SnacksGit,
        dashboard: SnacksDash,
        picker: LuaDict<LuaCallable<Option<LuaDict<LuaVal>>, ()>>,
    }
});
impl NvimConf<'_> {
    pub fn req_snacks(&self) -> Result<Snacks> {
        self.setup_plugin::<Snacks>("snacks", |snacks| snacks.setup()?.call(self.snacks_opts()))
    }

    pub fn load_snacks(&self) {
        self.add_packs(["https://github.com/folke/snacks.nvim"]);

        self.req_snacks().ok_or_notify(self);
    }

    fn snacks_opts(&self) -> impl LuaSub<LuaDict<LuaVal>> {
        let find_file =
            self.create_cb(|conf, ()| conf.req_snacks()?.dashboard()?.pick()?.call("files"));
        let find_text =
            self.create_cb(|conf, ()| conf.req_snacks()?.dashboard()?.pick()?.call("live_grep"));
        let load_session = self.create_cb(|conf, ()| conf.req_persistence()?.load()?.call(()));
        tbl!(owned, {
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
            dashboard = tbl!(owned, {
                enabled = true;
                preset.keys = tbl_seq![
                    tbl!(owned, {
                        icon = " ";
                        key = "f";
                        desc = "Find File";
                        action = find_file;
                    }),
                    tbl!(owned, {
                        icon = " ";
                        key = "n";
                        desc = "New File";
                        action = ":ene | startinsert";
                    }),
                    tbl!(owned, {
                        icon = " ";
                        key = "g";
                        desc = "Find Text";
                        action = find_text;
                    }),
                    tbl!(owned, {
                        icon = " ";
                        key = "s";
                        desc = "Restore Session";
                        action = load_session;
                    }),
                    tbl!(owned, {
                        icon = " ";
                        key = "q";
                        desc = "Quit";
                        action = ":qa";
                    }),
                ];
                sections = tbl_seq![
                    tbl!(owned, {
                        section = "header";
                    }),
                    tbl!(owned, {
                        section = "keys";
                        gap = 1;
                        padding = 1;
                    }),
                ];
            });
        })
    }
}
