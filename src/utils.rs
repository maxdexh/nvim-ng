use crate::prelude::*;

macro_rules! tbl {
    (builder = $builder:ident, {$( $t:tt )*}) => { #[allow(clippy::redundant_closure_call)] {
        (|| -> mlua::Result<()> { crate::utils::tbl! { @visit $builder $($t)* } })()
    }};
    (|$builder:ident| {$( $t:tt )*}) => {{
        crate::utils::defer_lua_table(|$builder| {
            crate::utils::tbl! { builder = $builder, { $($t)* } }
        })
    }};
    ({$( $t:tt )*}) => {
        crate::utils::tbl! { |_builder| { $($t)* } }
    };
    (eval($lua:expr), {$( $t:tt )*}) => {
        crate::utils::tbl! { { $($t)* } }.eval($lua)
    };
    (@visit $builder:ident $key_or_field:tt = $val:expr; $($t:tt)*) => {{
        $builder.set(crate::utils::tbl!(@key_or_field $key_or_field), $val)?;
        crate::utils::tbl! { @visit $builder $($t)* }
    }};
    (@visit $builder:ident $key_or_field:tt$(.$field:ident)+ = $val:expr; $($t:tt)*) => {{
        $builder.set(crate::utils::tbl!(@key_or_field $key_or_field), crate::utils::tbl!({
            $($field).+ = $val;
        }))?;
        crate::utils::tbl! { @visit $builder $($t)* }
    }};
    (@visit $builder:ident) => { Ok(()) };
    (@visit $($t:tt)*) => {
        compile_error! {
            concat!(
                "Unexpected input:\n",
                stringify!($($t)*),
            )
        }
    };
    (@key_or_field $field:ident) => { stringify!($field) };
    (@key_or_field [$key:expr]) => { $key };
}
pub(crate) use tbl;

macro_rules! tbl_seq {
    [$($val:expr),* $(,)?] => {
        crate::utils::defer_lua_table(|builder| {
            $(builder.push($val)?;)*
            Ok(())
        })
    };
}
pub(crate) use tbl_seq;

pub struct DeferIntoLua<F>(pub F);
impl<T, F: FnOnce(&Lua) -> T> DeferIntoLua<F> {
    pub fn eval(self, lua: &Lua) -> T {
        self.0(lua)
    }
}
pub fn defer_lua_val<T, F: FnOnce(&Lua) -> Result<T>>(f: F) -> DeferIntoLua<F> {
    DeferIntoLua(f)
}
impl<T: IntoLua, F: FnOnce(&Lua) -> Result<T>> IntoLua for DeferIntoLua<F> {
    fn into_lua(self, lua: &Lua) -> Result<LuaValue> {
        self.0(lua)?.into_lua(lua)
    }
}
impl<T: IntoLua, F: FnOnce(&Lua) -> Result<T>> LuaSub<T> for DeferIntoLua<F> {}

pub struct LuaTableInit<const RAW: bool> {
    pub table: LuaTable,
}
impl<const RAW: bool> LuaTableInit<RAW> {
    pub fn new(table: LuaTable) -> Self {
        Self { table }
    }
    pub fn init(&mut self, init: impl FnOnce(&mut Self) -> Result<()>) -> Result<&mut Self> {
        init(self)?;
        Ok(self)
    }
    pub fn init_finish(mut self, init: impl FnOnce(&mut Self) -> Result<()>) -> Result<LuaTable> {
        self.init(init)?;
        Ok(self.table)
    }
    pub fn push(&mut self, val: impl IntoLua) -> Result<()> {
        if RAW {
            self.table.raw_push(val)
        } else {
            self.table.push(val)
        }
    }
    pub fn set(&mut self, key: impl IntoLua, val: impl IntoLua) -> Result<()> {
        if RAW {
            self.table.raw_set(key, val)
        } else {
            self.table.set(key, val)
        }
    }
}
pub fn defer_lua_table(
    init: impl FnOnce(&mut LuaTableInit<true>) -> Result<()>,
) -> DeferIntoLua<impl FnOnce(&Lua) -> Result<LuaTable>> {
    defer_lua_val(|lua| LuaTableInit::new(lua.create_table()?).init_finish(init))
}

pub struct LuaDeferErr<T>(pub Result<T>);
impl<T> IntoLua for LuaDeferErr<T>
where
    T: IntoLua,
{
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        self.0?.into_lua(lua)
    }
}
impl<U, T: LuaSub<U>> LuaSub<U> for LuaDeferErr<T> {}

pub trait ResultExt {
    type Ok;
    type Err;

    fn ok_or_notify(self, env: &Nvim) -> Option<Self::Ok>
    where
        Self::Err: Into<LuaError>;
}
impl<T, E> ResultExt for std::result::Result<T, E> {
    type Ok = T;
    type Err = E;

    fn ok_or_notify(self, env: &Nvim) -> Option<T>
    where
        E: Into<LuaError>,
    {
        let res = self.map_err(Into::into);
        match res {
            Ok(ok) => Some(ok),
            Err(err) => {
                env.notify_err(&err);
                None
            }
        }
    }
}

macro_rules! opts_struct {
    ($trait_name:ident, $(#[$meta:meta])* $gname:ident, [$(($field:ident, $gp:ident, $with:ident)),* $(,)?]) => {
        pub trait $trait_name: mlua::IntoLua {
            fn into_table(self, lua: &mlua::Lua) -> mlua::Result<mlua::Table>;
        }
        impl $trait_name for mlua::Table {
            fn into_table(self, _: &mlua::Lua) -> mlua::Result<mlua::Table> {
                Ok(self)
            }
        }

        #[derive(Default, Debug)]
        $(#[$meta])*
        pub struct $gname<$($gp = crate::lua::LuaNil),*> {
            $(pub $field: $gp),*
        }
        impl $gname {
            pub fn empty() -> Self {
                Self {
                    $($field: crate::lua::LuaNil::None),*
                }
            }
        }
        impl<$($gp),*> $gname<$($gp),*> {
            crate::utils::opts_struct! { @impl_generic [$($gp $field $with)*] $gname [] }
        }
        impl<$($gp: mlua::IntoLua),*> mlua::IntoLua for $gname<$($gp),*> {
            fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                self.into_table(lua).map(mlua::Value::Table)
            }
        }
        impl<$($gp: mlua::IntoLua),*> $trait_name for $gname<$($gp),*> {
            fn into_table(self, lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
                let Self { $($field),* } = self;
                let tbl = lua.create_table_with_capacity(0, 0usize $(+ {let $field=1; $field})*)?;
                $(tbl.raw_set(stringify!($field), $field)?;)*
                Ok(tbl)
            }
        }
    };
    ( @impl_generic [] $($rest:tt)* ) => {};
    (
        @impl_generic
        [$gp:ident $field:ident $with:ident $($rgp:ident $rfield:ident $rwith:ident)*]
        $struct:ident
        [$($lgp:ident $lfield:ident)*]
    ) => {
        pub fn $with<_Param: mlua::IntoLua>(self, $field: _Param) -> $struct<$($lgp,)* _Param, $($rgp,)*> {
            let Self { $($lfield,)* $field: _, $($rfield,)* } = self;
            $struct {
                $($lfield,)*
                $field,
                $($rfield,)*
            }
        }
        crate::utils::opts_struct! { @impl_generic [$($rgp $rfield $rwith)*] $struct [$($lgp $lfield)* $gp $field] }
    }
}
pub(crate) use opts_struct;

macro_rules! nvim_subproxy {
    ($ty:ident, $get:ident, $base:ident) => {
        #[derive(Debug)]
        pub struct $ty<'a>(&'a crate::prelude::Nvim);
        impl $base<'_> {
            pub fn $get(&self) -> $ty<'_> {
                $ty(self.env())
            }
        }
        crate::utils::_proxy_impl!($ty);
    };
}
pub(crate) use nvim_subproxy;
macro_rules! nvim_proxy {
    ($ty:ident, $get:ident) => {
        #[derive(Debug)]
        pub struct $ty<'a>(&'a crate::prelude::Nvim);
        impl crate::prelude::Nvim {
            pub fn $get(&self) -> $ty<'_> {
                $ty(self)
            }
        }
        crate::utils::_proxy_impl!($ty);
    };
}
pub(crate) use nvim_proxy;
macro_rules! _proxy_impl {
    ($ty:ident) => {
        impl $ty<'_> {
            pub fn env(&self) -> &crate::prelude::Nvim {
                self.0
            }
            #[allow(dead_code)]
            pub fn lua(&self) -> &Lua {
                &self.env().lua
            }
        }
    };
}
pub(crate) use _proxy_impl;

macro_rules! tbl_proxy {
    ({
        struct $name:ident {
            $($field:ident: $fieldty:ty),* $(,)?
        }
    }) => {
        #[derive(Clone, Debug)]
        pub struct $name { pub table: mlua::Table }
        impl mlua::FromLua for $name {
            fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
                mlua::FromLua::from_lua(value, lua).map(|table| Self { table })
            }
        }
        impl $name {$(
            pub fn $field(&self) -> mlua::Result<$fieldty> {
                self.table.get(stringify!($field))
            }
        )*}
    };
}
pub(crate) use tbl_proxy;
