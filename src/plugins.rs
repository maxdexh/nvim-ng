use std::sync::{Arc, Mutex, OnceLock};

use crate::{prelude::*, utils::from_tbl_proxy};

#[derive(Debug, Clone)]
pub struct CachedReq<T>(Arc<OnceLock<T>>);
impl<T> Default for CachedReq<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T: mlua::FromLua> CachedReq<T> {
    pub fn require(&self, name: &str, env: &Nvim) -> Result<&T> {
        if let Some(it) = self.0.get() {
            return Ok(it);
        }
        std::hint::cold_path();

        let it = env.globals.require()?.call_any_ret(name)?;
        Ok(self.0.get_or_init(|| it))
    }
}

#[allow(clippy::complexity)]
struct SetupFunc<T>(Box<dyn FnOnce(&Nvim, &T) -> Result<()> + 'static + Send>);
impl<T> std::fmt::Debug for SetupFunc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(std::any::type_name::<Self>())
            .finish_non_exhaustive()
    }
}
#[derive(Debug, Default)]
enum SetupState<T> {
    #[default]
    NotReady,
    Ready(SetupFunc<T>),
    Done,
}

#[derive(Debug, Clone)]
pub struct CachedSetup<T>(Arc<(OnceLock<T>, Mutex<SetupState<T>>)>);
impl<T> Default for CachedSetup<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T: mlua::FromLua> CachedSetup<T> {
    pub fn register_setup(
        &self,
        callback: impl FnOnce(&Nvim, &T) -> Result<()> + 'static + Send,
    ) -> Result<()> {
        match &mut *self.0.1.lock().unwrap_or_else(|pe| pe.into_inner()) {
            state @ SetupState::NotReady => {
                *state = SetupState::Ready(SetupFunc(Box::new(callback)));
                Ok(())
            }
            SetupState::Ready(_) | SetupState::Done => {
                Err(LuaError::runtime("setup callback was already registered"))
            }
        }
    }
    pub fn require(&self, name: &str, env: &Nvim) -> Result<&T> {
        if let Some(it) = self.0.0.get() {
            return Ok(it);
        }
        std::hint::cold_path();

        let (cache, state) = &*self.0;
        Ok(
            match &mut *state.lock().unwrap_or_else(|pe| pe.into_inner()) {
                state @ SetupState::Ready(_) => {
                    let module: T = env.globals.require()?.call_any_ret(name)?;
                    let SetupState::Ready(cb) = std::mem::replace(state, SetupState::Done) else {
                        unreachable!()
                    };
                    cb.0(env, &module)?;
                    cache.set(module).unwrap_or_else(|_| unreachable!());
                    cache.get().unwrap()
                }
                SetupState::NotReady => {
                    return Err(LuaError::runtime("setup callback is not ready"));
                }
                SetupState::Done => cache.get().unwrap(),
            },
        )
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReqCache {
    pub snacks: CachedReq<Snacks>,
    pub persistence: CachedReq<Persistence>,
    pub conform: CachedSetup<Conform>,
    pub treesitter: CachedReq<Treesitter>,
}

impl NvimConf<'_> {
    pub fn req_snacks(&self) -> Result<&Snacks> {
        self.env().req_cache.snacks.require("snacks", self.env())
    }
    pub fn req_persistence(&self) -> Result<&Persistence> {
        self.env()
            .req_cache
            .persistence
            .require("persistence", self.env())
    }
    pub fn req_conform(&self) -> Result<&Conform> {
        self.env().req_cache.conform.require("conform", self.env())
    }
    pub fn req_treesitter(&self) -> Result<&Treesitter> {
        self.env()
            .req_cache
            .treesitter
            .require("nvim-treesitter", self.env())
    }
}

from_tbl_proxy!({
    struct Snacks {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        git: SnacksGit,
        dashboard: SnacksDash,
        picker: LuaDict<LuaCallable<Option<LuaDict<LuaVal>>, ()>>,
    }
});
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
    struct Persistence {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        load: LuaCallable<LuaDict<LuaVal>, ()>,
    }
});

from_tbl_proxy!({
    struct Conform {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        formatters_by_ft: LuaDictMut<LuaSeq<LuaString>>,
    }
});

from_tbl_proxy!({
    struct Treesitter {
        setup: LuaCallable<LuaDict<LuaVal>, ()>,
        install: LuaCallable<LuaSeq<LuaString>, ()>,
    }
});
