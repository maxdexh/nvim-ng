use crate::prelude::*;

pub fn do_try<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    f()
}

#[doc(hidden)]
pub mod __tbl {
    use crate::lua::{LuaTableMapOwned, LuaTableSeqOwned};
    use crate::prelude::*;
    use mlua::IntoLua;

    pub struct TableBuilder<K, V>(LuaTableMapOwned<K, V>);
    pub fn builder_new(lua: &Lua) -> Result<TableBuilder<LuaBottom, LuaBottom>> {
        lua.create_table()
            .map(LuaTableMapOwned::from_table_any)
            .map(TableBuilder)
    }
    pub trait TableWith<K, V> {
        type Out;
        fn __with(self, key: K, val: V) -> Result<Self::Out>;
    }
    pub trait TableBuildFinish {
        type Finish;
        fn __finish(self) -> Self::Finish;
    }
    impl<Key, Val, K: IntoLua, V: IntoLua> TableWith<K, V> for TableBuilder<Key, Val> {
        type Out = TableBuilder<LuaEither<Key, K>, LuaEither<Val, V>>;
        fn __with(self, key: K, val: V) -> Result<Self::Out> {
            let table = self.0.into_table_any();
            table.set(key, val)?;
            Ok(TableBuilder(LuaTableMapOwned::from_table_any(table)))
        }
    }
    impl<K, V> TableBuildFinish for TableBuilder<K, V> {
        type Finish = LuaTableMapOwned<K, V>;
        fn __finish(self) -> Self::Finish {
            self.0
        }
    }
    impl<T: LuaMutTable, K: LuaSub<T::Key>, V: LuaSub<T::Val>> TableWith<K, V> for &T {
        type Out = Self;
        fn __with(self, key: K, val: V) -> Result<Self::Out> {
            self.set(key, val)?;
            Ok(self)
        }
    }
    impl<T: LuaMutTable> TableBuildFinish for &T {
        type Finish = ();
        fn __finish(self) -> Self::Finish {}
    }

    pub fn tbl_seq_new(lua: &Lua) -> Result<LuaTableSeqOwned<LuaBottom>> {
        LuaTableSeqOwned::new(lua)
    }
    pub fn tbl_seq_append<T: IntoLua, V>(
        seq: LuaTableSeqOwned<V>,
        item: T,
    ) -> Result<LuaTableSeqOwned<LuaEither<T, V>>> {
        let seq = seq.into_table_any();
        seq.push(item)?;
        Ok(LuaTableSeqOwned::from_table_any(seq))
    }
}

macro_rules! tbl {
    (eval($lua:expr), $($rest:tt)*) => {
        crate::utils::tbl! { $($rest)* }.eval($lua)
    };
    ({$( $t:tt )*}) => {
        crate::lua::defer_lua_val(|lua| {
            let out = crate::utils::__tbl::builder_new(lua)?;
            crate::utils::tbl! { out(out), { $($t)* } }
        })
    };
    (out($table:expr), {$( $t:tt )*}) => {{
        let out = $table;
        crate::utils::do_try(move || {
            #[allow(unused_imports)]
            use crate::utils::__tbl::TableWith as _;
            use crate::utils::__tbl::TableBuildFinish as _;
            crate::utils::tbl! { @visit out $($t)* }
        })
    }};
    (@visit $builder:ident $key_or_field:tt = $val:expr; $($t:tt)*) => {{
        let $builder = $builder.__with(
            crate::utils::tbl!(@key_or_field $key_or_field),
            $val,
        )?;
        crate::utils::tbl! { @visit $builder $($t)* }
    }};
    (@visit $builder:ident $key_or_field:tt$(.$field:ident)+ = $val:expr; $($t:tt)*) => {{
        let $builder = $builder.__with(
            crate::utils::tbl!(@key_or_field $key_or_field),
            crate::utils::tbl!({
                $($field).+ = $val;
            }),
        )?;
        crate::utils::tbl! { @visit $builder $($t)* }
    }};
    (@visit $builder:ident) => { Ok($builder.__finish()) };
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
        crate::lua::defer_lua_val(|lua| {
            let seq = crate::utils::__tbl::tbl_seq_new(lua)?;
            $(let seq = crate::utils::__tbl::tbl_seq_append(seq, $val)?;)*
            Ok(seq)
        })
    };
}
pub(crate) use tbl_seq;

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
    ($trait_name:ident, $(#[$meta:meta])* $gname:ident, [$(($field:ident, $gp:ident, $fty:ty, $with:ident)),* $(,)?]) => {
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
            crate::utils::opts_struct! { @impl_generic [$($gp $field ($fty) $with)*] $gname [] }
        }
        impl<$($gp: crate::typing::LuaSub<Option<$fty>>),*> mlua::IntoLua for $gname<$($gp),*> {
            fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                self.into_table(lua).map(mlua::Value::Table)
            }
        }
        impl<$($gp: crate::typing::LuaSub<Option<$fty>>),*> $trait_name for $gname<$($gp),*> {
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
        [$gp:ident $field:ident ($fty:ty) $with:ident $($rgp:ident $rfield:ident $rfty:tt $rwith:ident)*]
        $struct:ident
        [$($lgp:ident $lfield:ident)*]
    ) => {
        pub fn $with<_Param: crate::typing::LuaSub<$fty>>(self, $field: _Param) -> $struct<$($lgp,)* Option<_Param>, $($rgp,)*> {
            let Self { $($lfield,)* $field: _, $($rfield,)* } = self;
            $struct {
                $($lfield,)*
                $field: Some($field),
                $($rfield,)*
            }
        }
        crate::utils::opts_struct! { @impl_generic [$($rgp $rfield $rfty $rwith)*] $struct [$($lgp $lfield)* $gp $field] }
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
