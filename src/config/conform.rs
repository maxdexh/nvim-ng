use crate::prelude::*;

crate::utils::from_tbl_proxy!({
    struct Conform {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        formatters_by_ft: LuaDictMut<LuaSeq<LuaString>>,
        formatters: LuaDictMut<LuaDictMut<LuaVal>>,
    }
});

impl NvimConf<'_> {
    pub fn load_conform(&self) {
        self.add_packs(["https://github.com/stevearc/conform.nvim"]);
    }
    fn req_conform(&self) -> Result<Conform> {
        // FIXME: Timeout triggers when installing from nixpkgs
        // FIXME: Keybind to toggle formatting
        self.setup_plugin::<Conform>("conform", |conform| {
            conform.setup()?.call(tbl!(owned, {
                format_on_save = tbl!(owned, {
                    timeout_ms = 500;
                    lsp_format = "fallback";
                });
            }))
        })
    }

    // FIXME: table arg can be more than sequence, e.g. fallback = ...
    pub fn set_formatter(&self, ft: impl LuaSub<LuaString>, table: impl LuaSub<LuaSeq<LuaString>>) {
        do_try(|| {
            let conform = self.req_conform()?;
            conform.formatters_by_ft()?.set(ft, table)?;

            Ok(())
        })
        .ok_or_notify(self);
    }
    pub fn formatter_use_nix(&self, formatter: &str, package: &str, cmd: &str) {
        do_try(|| {
            let lua = self.lua();

            let formatter: LuaString = lua_conv_sub(lua, formatter)?;

            let fmts = self.req_conform()?.formatters()?.into_table_any();
            let settings = match fmts.get_any(formatter.clone())? {
                Some(s) => s,
                None => {
                    let s = lua.create_table()?;
                    fmts.set_any(formatter, s.clone())?;
                    s
                }
            };
            settings.set_any("command", "nix")?;
            let prepend_args = self.lua().create_sequence_from([
                "shell",
                format!("nixpkgs#{package}").as_str(),
                "--command",
                cmd,
            ])?;
            let key = lua.create_string("prepend_args")?;
            if let Some(t) = settings.get_any::<Option<LuaTableAny>>(key.clone())? {
                for v in t.sequence_values::<mlua::Value>() {
                    prepend_args.raw_push_any(v?)?;
                }
            }
            settings.set_any(key, prepend_args)
        })
        .ok_or_notify(self);
    }
}
