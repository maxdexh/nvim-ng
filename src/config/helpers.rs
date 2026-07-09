use crate::{env::NvimConf, prelude::*, utils::nvim_subproxy};

nvim_subproxy!(NvimKeymap, keymap, NvimConf);
impl NvimKeymap<'_> {
    pub fn set_base(
        &self,
        modes: impl LuaSub<LuaUnion<LuaString, LuaSeq<LuaString>>>,
        sequence: impl LuaSub<LuaString>,
        callback_or_action: impl LuaSub<LuaVal>,
        opts: impl LuaSub<LuaDict<LuaVal>>, // TODO: opts struct
    ) -> bool {
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
        modes: impl LuaSub<LuaUnion<LuaString, LuaSeq<LuaString>>>,
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
