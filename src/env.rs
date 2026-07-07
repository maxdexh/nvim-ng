mod plugin;

use crate::{
    prelude::*,
    utils::{nvim_proxy, nvim_subproxy, opts_struct},
};

mod proxy {
    use crate::prelude::*;
    use crate::utils::tbl_proxy;

    tbl_proxy!({
        struct VimDiagnostic {
            config: LuaCallable<LuaTableAny, ()>,
        }
    });
    tbl_proxy!({
        struct VimPack {
            add: LuaCallable<LuaTableAny, ()>,
        }
    });
    tbl_proxy!({
        struct VimKeymap {
            set: LuaCallable<(LuaTableAny, LuaString, LuaValue, LuaTableAny), ()>,
        }
    });
    tbl_proxy!({
        struct VimUV {
            cwd: LuaCallable<(), LuaString>,
        }
    });
    tbl_proxy!({
        struct VimApi {
            nvim_create_autocmd: LuaCallable<(LuaString, LuaTableAny), ()>,
        }
    });
    tbl_proxy!({
        struct VimVersion {
            range: LuaCallable<LuaString, LuaValue>,
        }
    });
    tbl_proxy!({
        struct Vim {
            opt: LuaTableMapMut<LuaString, LuaValue>,
            opt_local: LuaTableMapMut<LuaString, LuaValue>,
            g: LuaTableMapMut<LuaString, LuaValue>,
            uv: VimUV,
            pack: VimPack,
            keymap: VimKeymap,
            diagnostic: VimDiagnostic,
            notify: LuaCallable<(LuaString, LuaInt), ()>,
            cmd: LuaCallable<LuaString, ()>,
            version: VimVersion,
            api: VimApi,
        }
    });
    tbl_proxy!({
        struct Globals {
            vim: Vim,
            require: LuaCallable<LuaString, LuaValue>,
        }
    });
}

#[derive(Clone, Debug)]
pub struct Nvim {
    pub lua: Lua,
    pub globals: proxy::Globals,
    pub req_cache: crate::plugins::ReqCache,
}
impl Nvim {
    #[cold]
    pub fn notify_err(&self, err: &LuaError) {
        self.lua
            .traceback(Some(&format!("{err}\n\n")), 0)
            .or_else(|tb_err| {
                self.lua.create_string(format!(
                    "{err}\n\nfailed to create trackback: {tb_err}\n{}",
                    std::backtrace::Backtrace::force_capture()
                ))
            })
            .and_then(|err| {
                // 4 = error
                self.globals.vim()?.notify()?.call((err, 4))
            })
            .expect("failed to notify");
    }
    pub fn create_func<A: FromLuaMultiTyped, R: IntoLuaMultiTyped>(
        &self,
        f: impl Fn(&Nvim, A) -> Result<R> + 'static,
    ) -> LuaDeferErr<LuaCallable<A, R>> {
        let env = self.clone();
        LuaDeferErr(
            self.lua
                .create_function(move |_, args| f(&env, args))
                .map(LuaCallable::from_any_func),
        )
    }
    pub fn create_autocmd_cb<A: FromLuaMultiTyped>(
        &self,
        f: impl Fn(&Nvim, A) -> Result<()> + 'static,
    ) -> LuaDeferErr<LuaCallable<A, ()>> {
        self.create_func(move |env, args| {
            f(env, args).ok_or_notify(env);
            Ok(())
        })
    }
    pub fn create_autocmd_cb_once<A: FromLuaMultiTyped>(
        &self,
        f: impl FnOnce(&Nvim, A) -> Result<()> + 'static,
    ) -> LuaDeferErr<LuaCallable<A, ()>> {
        let func = std::sync::Mutex::new(Some(f));
        self.create_autocmd_cb(move |env, args| {
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
        self.globals.require()?.call_any_ret(s)
    }
}

opts_struct!(
    AutoCmdOptsAny,
    AutoCmdOpts,
    [
        (once, O, bool, with_once),
        (pattern, P, LuaString, with_pattern)
    ]
);

nvim_proxy!(VimProxy, vim);
impl VimProxy<'_> {
    pub fn add_autocmd(
        &self,
        event: &str,
        opts: impl AutoCmdOptsAny,
        callback: impl LuaSub<LuaCallable<(), ()>>,
    ) -> bool {
        do_try(|| {
            let opts = opts.into_table(self.lua())?;

            opts.set("callback", callback)?;
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
    pub fn init_g(
        &self,
        init: impl FnOnce(&LuaTableMapMut<LuaString, LuaValue>) -> Result<()>,
    ) -> bool {
        do_try(|| init(&self.env().globals.vim()?.g()?))
            .ok_or_notify(self.env())
            .is_some()
    }
    pub fn init_opt(
        &self,
        init: impl FnOnce(&LuaTableMapMut<LuaString, LuaValue>) -> Result<()>,
    ) -> bool {
        do_try(|| init(&self.env().globals.vim()?.opt()?))
            .ok_or_notify(self.env())
            .is_some()
    }
    pub fn init_opt_local(
        &self,
        init: impl FnOnce(&LuaTableMapMut<LuaString, LuaValue>) -> Result<()>,
    ) -> bool {
        do_try(|| init(&self.env().globals.vim()?.opt_local()?))
            .ok_or_notify(self.env())
            .is_some()
    }
    pub fn run_cmd(&self, cmd: impl LuaSub<LuaString>) -> bool {
        do_try(|| self.env().globals.vim()?.cmd()?.call(cmd))
            .ok_or_notify(self.env())
            .is_some()
    }
}

nvim_subproxy!(VimVersionProxy, version, VimProxy);
impl VimVersionProxy<'_> {
    pub fn range(&self, spec: &str) -> LuaDeferErr<LuaValue> {
        LuaDeferErr(do_try(|| {
            self.env().globals.vim()?.version()?.range()?.call(spec)
        }))
    }
}

nvim_subproxy!(VimPackProxy, pack, VimProxy);
opts_struct!(
    PackOptsAny,
    PackOpts,
    [(version, V, LuaValue, with_version)]
);
impl VimPackProxy<'_> {
    pub fn add(&self, url: &str, opts: impl PackOptsAny) -> bool {
        let env = self.env();
        opts.into_table(self.lua())
            .and_then(|opts| {
                opts.set("src", url)?;
                env.globals.vim()?.pack()?.add()?.call([opts])
            })
            .ok_or_notify(env)
            .is_some()
    }
}

nvim_proxy!(NvimConf, config);
