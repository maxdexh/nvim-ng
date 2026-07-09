mod plugin;

use crate::{
    prelude::*,
    utils::{nvim_proxy, opts_struct},
};

pub mod vim;

crate::utils::from_tbl_proxy!({
    struct Globals {
        vim: vim::Vim,
        require: LuaCallable<LuaString, LuaVal>,
    }
});

#[derive(Clone, Debug)]
pub struct Nvim {
    pub lua: Lua,
    pub globals: Globals,
    pub req_cache: crate::plugins::ReqCache,
}

#[cold]
pub fn lua_notify_err(lua: Option<&Lua>, err: impl std::fmt::Display) {
    let mut msg_begin = format!(
        "{err}\n\n{}\n{}\n\n",
        if cfg!(debug_assertions) {
            ""
        } else {
            "WARN: backtrace may not include all info in release mode"
        },
        std::backtrace::Backtrace::force_capture()
    );
    let Some(lua) = lua else {
        msg_begin.push_str("No lua env was available");
        eprintln!("{msg_begin}");
        return;
    };
    let msg = lua
        .traceback(Some(&msg_begin), 0)
        .map_err(|tb_err| {
            std::fmt::write(
                &mut msg_begin,
                format_args!("failed to create lua trackback: {tb_err}",),
            )
            .unwrap_or_else(|_| unreachable!("error in infallible write"));
            msg_begin
        })
        .map_or_else(mlua::Either::Left, mlua::Either::Right);

    () = do_try(|| {
        lua.convert::<Globals>(lua.globals())?
            .vim()?
            .notify()?
            .call((msg.clone(), 4i64)) // 4 = error
    })
    .unwrap_or_else(|notify_err| {
        eprintln!("Failed to notify: {notify_err}\n");
        match msg {
            mlua::Either::Left(l) => eprintln!("{l}"),
            mlua::Either::Right(r) => eprintln!("{}", String::from_utf8_lossy(&r.as_bytes())),
        }
    });
}

impl Nvim {
    #[cold]
    pub fn notify_err(&self, err: impl std::fmt::Display) {
        lua_notify_err(Some(&self.lua), err)
    }
    pub fn create_func<A: FromLuaMultiTyped, R: IntoLuaMultiTyped>(
        &self,
        f: impl Fn(&Nvim, A) -> Result<R> + 'static,
    ) -> LuaDeferErr<LuaCallable<A, R>> {
        let env = self.clone();
        LuaDeferErr(
            self.lua
                .create_function(move |_, args| f(&env, args))
                .map(LuaCallable::from_mlua_func),
        )
    }
    pub fn create_cb<A: FromLuaMultiTyped>(
        &self,
        f: impl Fn(&Nvim, A) -> Result<()> + 'static,
    ) -> LuaDeferErr<LuaCallable<A, ()>> {
        self.create_func(move |env, args| {
            f(env, args).ok_or_notify(env);
            Ok(())
        })
    }
    pub fn create_cb_once<A: FromLuaMultiTyped>(
        &self,
        f: impl FnOnce(&Nvim, A) -> Result<()> + 'static,
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
        self.globals.require()?.call_any_ret(s)
    }
}

// TODO: Mandatory opts
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
                .call((event, crate::utils::downcast_mlua_map(opts)))
        })
        .ok_or_notify(self.env())
        .is_some()
    }
    pub fn run_cmd(&self, cmd: impl LuaSub<LuaString>) -> bool {
        do_try(|| self.env().globals.vim()?.cmd()?.call(cmd))
            .ok_or_notify(self.env())
            .is_some()
    }
}

nvim_proxy!(NvimConf, config);
