use crate::{
    env::{
        NvimConf,
        vim::{api::AutoCmdOpts, keymap::KeymapOpts, pack::PackOpts},
    },
    prelude::*,
};

impl NvimConf<'_> {
    pub fn add_autocmd(
        &self,
        event: impl LuaSub<LuaString>,
        opts: impl LuaSub<LuaStruct<AutoCmdOpts>>,
    ) -> bool {
        do_try(|| {
            self.env()
                .globals
                .vim()?
                .api()?
                .nvim_create_autocmd()?
                .call((event, opts))
        })
        .ok_or_notify(self.env())
        .is_some()
    }
    pub fn run_cmd(&self, cmd: impl LuaSub<LuaString>) -> bool {
        do_try(|| self.env().globals.vim()?.cmd()?.call(cmd))
            .ok_or_notify(self.env())
            .is_some()
    }
    pub fn set_keymap(
        &self,
        modes: impl LuaSub<LuaUnion<LuaString, LuaSeq<LuaString>>>,
        sequence: impl LuaSub<LuaString>,
        action: impl LuaSub<LuaUnion<LuaString, LuaCallable<(), ()>>>,
        opts: impl LuaSub<LuaStruct<KeymapOpts>>,
    ) -> bool {
        do_try(|| {
            self.env()
                .globals
                .vim()?
                .keymap()?
                .set()?
                .call((modes, sequence, action, opts))
        })
        .ok_or_notify(self.env())
        .is_some()
    }

    pub fn create_cb<A: FromLuaMultiTyped>(
        &self,
        f: impl Fn(NvimConf, A) -> Result<()> + 'static,
    ) -> LuaDeferErr<LuaCallable<A, ()>> {
        self.env().create_func(move |env, args| {
            f(env.conf(), args).ok_or_notify(env);
            Ok(())
        })
    }
    pub fn create_cb_once<A: FromLuaMultiTyped>(
        &self,
        f: impl FnOnce(NvimConf, A) -> Result<()> + 'static,
    ) -> LuaDeferErr<LuaCallable<A, ()>> {
        let func = std::sync::Mutex::new(Some(f));
        self.create_cb(move |env, args| {
            func.try_lock()
                .or_else(|err| match err {
                    std::sync::TryLockError::Poisoned(pe) => Ok(pe.into_inner()),
                    std::sync::TryLockError::WouldBlock => Err(()),
                })
                .ok()
                .and_then(|mut it| it.take())
                .ok_or_else(|| LuaError::runtime("callback can only be called once"))
                .and_then(|f| f(env, args))
        })
    }
    pub fn call_require<T: mlua::FromLua>(&self, s: &str) -> Result<T> {
        self.env().globals.require()?.call_any_ret(s)
    }
    pub fn add_packs(
        &self,
        packs: impl LuaSub<LuaSeq<LuaUnion<LuaStruct<PackOpts>, LuaString>>>,
    ) -> bool {
        do_try(|| self.env().globals.vim()?.pack()?.add()?.call(packs))
            .ok_or_notify(self)
            .is_some()
    }
}
