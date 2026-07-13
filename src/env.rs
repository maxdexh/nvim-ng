mod plugin;

use crate::{lua::AsLua, prelude::*};

pub mod gvim;

crate::utils::from_tbl_proxy!({
    struct Globals {
        vim: gvim::Vim,
        require: LuaCallable<LuaString, LuaVal>,
    }
});

#[derive(Clone, Debug)]
pub struct Nvim {
    pub lua: Lua,
    pub globals: Globals,
    pub registry: crate::registry::Registry,
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
        .as_mlua()
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
        let vim = lua.convert::<Globals>(lua.globals())?.vim()?;
        let msg = msg.as_ref().map_left(std::ops::Deref::deref);
        vim.notify()?.call((msg, vim.log()?.levels()?.ERROR()?))
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
    pub fn create_func<A: FromLuaMultiTyped, R: IntoLuaMultiTyped>(
        &self,
        f: impl Fn(&Nvim, A) -> Result<R> + 'static,
        err_handler: impl Fn(&Lua, Error) -> mlua::Result<R::IntoReprMulti> + 'static,
    ) -> Result<LuaCallable<A, R>> {
        let env = self.clone();
        self.lua
            .as_mlua()
            .create_function(move |lua, args| {
                A::from_mlua_multi(args)
                    .and_then(|args| f(&env, args).and_then(|it| it.into_mlua_multi()))
                    .or_else(|err| err_handler(Lua::by_mlua(lua), err))
            })
            .map(LuaCallable::from_mlua_func)
            .map_err(crate::lua::mlua_into_error)
    }

    pub fn require<T: PopLua>(&self, name: impl LuaSub<LuaString>) -> Result<T> {
        self.globals.require()?.call_any_ret(name)
    }
}
impl AsLua for Nvim {
    fn as_lua(&self) -> &Lua {
        &self.lua
    }
}
impl AsLua for NvimConf<'_> {
    fn as_lua(&self) -> &Lua {
        self.lua()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NvimConf<'a>(&'a crate::prelude::Nvim);

impl crate::prelude::Nvim {
    pub fn conf(&self) -> NvimConf<'_> {
        NvimConf(self)
    }
}
impl NvimConf<'_> {
    pub fn env(&self) -> &crate::prelude::Nvim {
        self.0
    }
    #[allow(dead_code)]
    pub fn lua(&self) -> &Lua {
        &self.env().lua
    }
}
