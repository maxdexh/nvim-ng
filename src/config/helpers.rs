use crate::{env::NvimConf, prelude::*, utils::nvim_subproxy};

nvim_subproxy!(NvimKeymap, keymap, NvimConf);
impl NvimKeymap<'_> {
    pub fn set_base(
        &self,
        modes: impl IntoIterator<Item: AsRef<[u8]>>,
        sequence: impl LuaSub<LuaString>,
        callback_or_action: impl LuaSub<LuaVal>,
        opts: impl LuaSub<LuaTableAny>,
    ) -> bool {
        let modes = LuaDeferErr(
            self.lua().create_sequence_from(
                modes
                    .into_iter()
                    .map(|seq| defer_lua_val(|lua| lua.create_string(seq))),
            ),
        );
        do_try(|| {
            self.env().globals.vim()?.keymap()?.set()?.call((
                modes,
                sequence,
                callback_or_action,
                opts,
            ))
        })
        .ok_or_notify(self.env())
        .is_some()
    }
    pub fn set(
        &self,
        modes: impl IntoIterator<Item: AsRef<[u8]>>,
        sequence: impl LuaSub<LuaString>,
        desc: impl LuaSub<LuaString>,
        callback: impl Fn(&Nvim) -> Result<()> + 'static + Send,
    ) -> bool {
        self.set_base(
            modes,
            sequence,
            LuaCastIntoAny(self.env().create_func(move |env, ()| callback(env))),
            tbl!({
                desc = desc;
            }),
        )
    }
}
