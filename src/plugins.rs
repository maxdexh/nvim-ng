use std::sync::{Arc, Mutex, OnceLock};

use crate::{prelude::*, utils::tbl_proxy};

#[derive(Debug, Clone)]
pub struct CachedReq<T>(Arc<OnceLock<T>>);
impl<T> Default for CachedReq<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T: FromLua> CachedReq<T> {
    pub fn require(&self, name: &str, env: &Nvim) -> Result<&T> {
        if let Some(it) = self.0.get() {
            return Ok(it);
        }
        std::hint::cold_path();

        let it = env.call_require(name)?;
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
impl<T: FromLua> CachedSetup<T> {
    pub fn register(
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

        match &mut *self.0.1.lock().unwrap_or_else(|pe| pe.into_inner()) {
            state @ SetupState::Ready(_) => {
                let module: T = env.call_require(name)?;
                let SetupState::Ready(cb) = std::mem::replace(state, SetupState::Done) else {
                    unreachable!()
                };
                cb.0(env, &module)?;
            }
            SetupState::NotReady => {
                return Err(LuaError::runtime("setup callback is not ready"));
            }
            SetupState::Done => (),
        }

        Ok(self.0.0.get().unwrap())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReqCache {
    pub snacks: CachedReq<Snacks>,
    pub persistence: CachedReq<Persistence>,
    pub conform: CachedSetup<Conform>,
    pub treesitter: CachedReq<Treesitter>,
}

impl Nvim {
    pub fn req_snacks(&self) -> Result<&Snacks> {
        self.req_cache.snacks.require("snacks", self)
    }
    pub fn req_persistence(&self) -> Result<&Persistence> {
        self.req_cache.persistence.require("persistence", self)
    }
    pub fn req_conform(&self) -> Result<&Conform> {
        self.req_cache.conform.require("conform", self)
    }
    pub fn req_treesitter(&self) -> Result<&Treesitter> {
        self.req_cache.treesitter.require("nvim-treesitter", self)
    }
}

tbl_proxy!({
    struct GenericPlugin {
        setup: LuaCallable<LuaTable, ()>,
    }
});

tbl_proxy!({
    struct Snacks {
        setup: LuaCallable<LuaTable, ()>,
        git: SnacksGit,
        dashboard: SnacksDash,
        picker: LuaTableMap<LuaString, LuaCallable<Option<LuaTable>, ()>>,
    }
});
tbl_proxy!({
    struct SnacksDash {
        pick: LuaCallable<LuaString, ()>,
    }
});
tbl_proxy!({
    struct SnacksGit {
        get_root: LuaCallable<(), Option<LuaString>>,
    }
});

tbl_proxy!({
    struct Persistence {
        setup: LuaCallable<LuaTable, ()>,
        load: LuaCallable<LuaTable, ()>,
    }
});

tbl_proxy!({
    struct Conform {
        setup: LuaCallable<LuaTable, ()>,
        formatters_by_ft: LuaTable,
    }
});

tbl_proxy!({
    struct Treesitter {
        setup: LuaCallable<LuaTable, ()>,
        install: LuaCallable<LuaTable, ()>,
    }
});
