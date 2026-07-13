use std::{collections::HashSet, sync::RwLock};

use crate::{
    env::{
        NvimConf,
        gvim::{api::AutoCmdOpts, keymap::KeymapOpts, pack::PackOpts},
    },
    prelude::*,
};

impl NvimConf<'_> {
    pub fn with_vim_opt<T>(
        &self,
        f: impl FnOnce(LuaMapMut<LuaString, LuaVal>) -> Result<T>,
    ) -> Result<T> {
        f(self.env().globals.vim()?.opt()?)
    }
    pub fn with_vim_opt_local<T>(
        &self,
        f: impl FnOnce(LuaMapMut<LuaString, LuaVal>) -> Result<T>,
    ) -> Result<T> {
        f(self.env().globals.vim()?.opt_local()?)
    }
    pub fn with_vim_g<T>(
        &self,
        f: impl FnOnce(LuaMapMut<LuaString, LuaVal>) -> Result<T>,
    ) -> Result<T> {
        f(self.env().globals.vim()?.g()?)
    }
    pub fn add_autocmd(
        &self,
        event: impl LuaSub<LuaString>,
        opts: impl LuaSub<LuaStruct<AutoCmdOpts>>,
    ) {
        do_try(|| {
            self.env()
                .globals
                .vim()?
                .api()?
                .nvim_create_autocmd()?
                .call((event, opts))
        })
        .ok_or_notify(self.env());
    }
    pub fn schedule(&self, f: impl FnOnce(NvimConf) -> Result<()> + 'static) -> Result<()> {
        let cb = self.create_cb_once(|conf, ()| f(conf)).into_result()?;
        self.env().globals.vim()?.schedule()?.call(cb)
    }
    pub fn on_very_lazy(&self, f: impl FnOnce(NvimConf) -> Result<()> + 'static) -> Result<()> {
        let cb = self
            .create_cb_once(|conf, ()| conf.schedule(f))
            .into_result()?;

        let opts = mk_builder!(AutoCmdOpts, {
            callback = cb;
            once = true;
        });

        self.env()
            .globals
            .vim()?
            .api()?
            .nvim_create_autocmd()?
            .call(("UIEnter", opts))
    }
    pub fn run_cmd(&self, cmd: impl LuaSub<LuaString>) {
        do_try(|| self.env().globals.vim()?.cmd()?.call(cmd)).ok_or_notify(self.env());
    }
    pub fn set_keymap(
        &self,
        modes: impl LuaSub<LuaUnion<LuaString, LuaSeq<LuaString>>>,
        sequence: impl LuaSub<LuaString>,
        action: impl LuaSub<LuaUnion<LuaString, LuaCallable<(), ()>>>,
        opts: impl LuaSub<LuaStruct<KeymapOpts>>,
    ) {
        do_try(|| {
            self.env()
                .globals
                .vim()?
                .keymap()?
                .set()?
                .call((modes, sequence, action, opts))
        })
        .ok_or_notify(self.env());
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
                .map_or_else(
                    |err| match err {
                        std::sync::TryLockError::Poisoned(pe) => Some(pe.into_inner()),
                        std::sync::TryLockError::WouldBlock => None,
                    },
                    Some,
                )
                .and_then(|mut it| it.take())
                .ok_or_else(|| anyhow::anyhow!("callback can only be called once"))
                .and_then(|f| f(env, args))
        })
    }
    pub fn setup_plugin_now(&self, name: &str, opts: impl LuaSub<LuaDict<LuaVal>>) -> Result<()> {
        if !self.get_setup_state().lock_setup(name) {
            panic!("setup_plugin_now called twice on {name:?}");
        }
        let plugin = self.env().require::<mlua::Table>(name)?;
        let setup: LuaCallable<LuaDict<LuaVal>, ()> = plugin.get("setup")?;
        setup.call(opts)
    }
    pub fn setup_plugin<T: mlua::FromLua>(
        &self,
        name: &str,
        setup: impl FnOnce(&mut T) -> Result<()>,
    ) -> Result<T> {
        let mut plugin = self.env().require::<T>(name)?;
        if self.get_setup_state().lock_setup(name) {
            setup(&mut plugin)?;
        }
        Ok(plugin)
    }
    pub fn add_packs(&self, packs: impl LuaSub<LuaSeq<LuaUnion<LuaStruct<PackOpts>, LuaString>>>) {
        do_try(|| self.env().globals.vim()?.pack()?.add()?.call(packs)).ok_or_notify(self);
    }
}

#[derive(Default)]
struct SetupState {
    plugins: RwLock<HashSet<String>>,
}
impl SetupState {
    fn is_setup(&self, name: &str) -> bool {
        self.plugins
            .read()
            .unwrap_or_else(|pe| pe.into_inner())
            .contains(name)
    }
    fn lock_setup(&self, name: &str) -> bool {
        if self.is_setup(name) {
            return false;
        }

        std::hint::cold_path();

        let mut plugins = self.plugins.write().unwrap_or_else(|pe| pe.into_inner());
        if plugins.contains(name) {
            return false;
        }
        plugins.insert(name.into());
        true
    }
}
impl NvimConf<'_> {
    fn get_setup_state(&self) -> std::sync::Arc<SetupState> {
        self.env().registry.get_or_default()
    }
}
