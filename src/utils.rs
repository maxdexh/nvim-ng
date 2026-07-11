use crate::{lua::AsLua, prelude::*};

pub fn do_try<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    f()
}

pub type LuaDict<V> = LuaMap<LuaString, V>;
pub type LuaDictMut<V> = LuaMapMut<LuaString, V>;

#[doc(hidden)]
pub mod __tbl {
    use crate::lua::LuaDeferImpl;
    use crate::prelude::*;
    use mlua::IntoLua;

    pub fn single_key_val<K: IntoLua, V: IntoLua>(
        key: K,
        val: V,
    ) -> LuaDeferImpl!(LuaMapOwned<K, V>) {
        lua_defer_val(|lua| {
            lua.create_table_from([(key, val)])
                .map(LuaMapOwned::cast_mlua_table)
        })
    }

    pub fn tbl_seq_new(lua: &Lua) -> Result<LuaSeqOwned<LuaBottom>> {
        LuaSeqOwned::new(lua)
    }
    pub fn tbl_seq_append<T: IntoLua, V>(
        seq: LuaSeqOwned<V>,
        item: T,
    ) -> Result<LuaSeqOwned<LuaUnion<T, V>>> {
        let seq = seq.into_table_any();
        seq.push(item)?;
        Ok(LuaSeqOwned::cast_mlua_table(seq))
    }
}

macro_rules! tbl {
    (eval($lua:expr), $($rest:tt)*) => {
        crate::utils::tbl! { $($rest)* }.eval($lua)
    };
    ({$( $t:tt )*}) => {
        crate::lua::lua_defer_val(|lua| {
            #[deprecated]
            let out = crate::utils::__tbl::builder_new(lua)?;
            crate::utils::tbl! { out(out), { $($t)* } }
        })
    };
    (owned, {$( $t:tt )*}) => {
        crate::utils::tbl! { owned!(crate::lua::LuaString, crate::lua::LuaVal), { $($t)* } }
    };
    (owned!($vt:ty), {$( $t:tt )*}) => {
        crate::utils::tbl! { owned!(crate::lua::LuaString, $vt), { $($t)* } }
    };
    (owned!($kt:ty, $vt:ty), {$( $t:tt )*}) => {
        crate::lua::lua_defer_val(|lua| {
            let out = crate::lua::LuaMapOwned::<$kt, $vt>::new(lua)?;
            crate::utils::tbl! { out(out), { $($t)* } }
        })
    };
    (out($table:expr), {$( $t:tt )*}) => {{
        let out = $table;
        crate::utils::do_try(move || {
            #[allow(unused_imports)]
            use crate::lua::LuaTableSet as _;
            crate::utils::tbl! { @visit out $($t)* }
            Ok(out)
        })
    }};
    (@visit $out:ident $key_or_field:tt = $val:expr; $($t:tt)*) => {
        $out.set(
            crate::utils::tbl!(@key_or_field $key_or_field),
            $val,
        )?;
        crate::utils::tbl! { @visit $out $($t)* }
    };
    (@visit $out:ident $key_or_field:tt$(.$field:ident)+ = $val:expr; $($t:tt)*) => {{
        $out.set(
            crate::utils::tbl!(@key_or_field $key_or_field),
            crate::utils::tbl!(@nest_single_field [$(stringify!($field)),*], $val),
        )?;
        crate::utils::tbl! { @visit $out $($t)* }
    }};
    (@visit $out:ident) => { () };
    (@visit $($t:tt)*) => {
        compile_error! {
            concat!(
                "Unexpected input:\n",
                stringify!($($t)*),
            )
        }
    };
    (@key_or_field $field:ident) => { stringify!($field) };
    (@key_or_field $field:literal) => { $field };
    (@key_or_field [$key:expr]) => { $key };
    (@nest_single_field [$first_field:expr $(,$field:expr)*], $val:expr) => {
        crate::utils::__tbl::single_key_val(
            $first_field,
            crate::utils::tbl!(@nest_single_field [$($field),*], $val),
        )
    };
    (@nest_single_field [], $val:expr) => { $val };
}
pub(crate) use tbl;

macro_rules! tbl_seq {
    [$($val:expr),* $(,)?] => {
        crate::lua::lua_defer_val(|lua| {
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

    fn ok_or_notify(self, lua: impl AsLua) -> Option<Self::Ok>
    where
        Self::Err: Into<LuaError>;
}
impl<T, E> ResultExt for std::result::Result<T, E> {
    type Ok = T;
    type Err = E;

    fn ok_or_notify(self, lua: impl AsLua) -> Option<T>
    where
        E: Into<LuaError>,
    {
        let res = self.map_err(Into::into);
        match res {
            Ok(ok) => Some(ok),
            Err(err) => {
                crate::env::lua_notify_err(Some(lua.as_lua()), err);
                None
            }
        }
    }
}

macro_rules! builder_struct {
    ({
        $(#[$meta:meta])*
        struct $gname:ident {$(
            $field:ident: $fty:ty
        ),* $(,)? }
    }) => {
        $(#[$meta])*
        #[derive(Default, Debug)]
        #[allow(non_camel_case_types)]
        pub struct $gname<$($field = $fty),*> {
            $(pub $field: $field),*
        }
        #[allow(non_camel_case_types)]
        const _: () = {
            $(type $field = crate::lua::LuaNil;)*
            impl $gname<$($field),*> {
                pub fn _new() -> Self {
                    Self {
                        $($field: crate::lua::LuaNil),*
                    }
                }
            }
        };

        #[allow(non_camel_case_types)]
        const _: () = {
            impl<$($field),*> $gname<$($field),*> {
                crate::utils::builder_struct! { @impl_generic [$($field ($fty))*] $gname [] }
            }
            impl<$($field: mlua::IntoLua),*> mlua::IntoLua for $gname<$($field),*> {
                fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                    let table = lua.create_table()?;
                    let Self { $($field),* } = self;
                    $(table.raw_set(stringify!($field), $field)?;)*
                    mlua::Result::Ok(mlua::Value::Table(table))
                }
            }
            impl crate::lua::LuaStructInner for $gname {
                const FIELD_NAMES: &[&[u8]] = &[
                    $(stringify!($field).as_bytes()),*
                ];
                type Fields = ($($fty,)*);
                type Repr = mlua::Table;
            }
            impl<$($field: crate::typing::LuaSub<$fty>),*> $gname<$($field),*> {
                pub fn _finish(self) -> crate::lua::LuaDeferImpl!(crate::lua::LuaStruct::<$gname>) {
                    crate::lua::lua_defer_val(|lua| {
                        let table = lua.create_table()?;
                        let Self { $($field),* } = self;
                        $(table.raw_set(stringify!($field), $field)?;)*
                        mlua::Result::Ok(crate::lua::LuaStruct::<$gname>::from_repr_unchecked(table))
                    })
                }
            }
        };
    };
    ( @impl_generic [] $($rest:tt)* ) => {};
    (
        @impl_generic
        [$field:ident ($fty:ty) $($rfield:ident $rfty:tt)*]
        $struct:ident
        [$($lfield:ident)*]
    ) => {
        pub fn $field<_Param>(self, $field: _Param) -> $struct<$($lfield,)* _Param, $($rfield,)*> {
            let Self { $($lfield,)* $field: _, $($rfield,)* } = self;
            $struct {
                $($lfield,)*
                $field,
                $($rfield,)*
            }
        }
        crate::utils::builder_struct! { @impl_generic [$($rfield $rfty)*] $struct [$($lfield)* $field] }
    }
}
pub(crate) use builder_struct;

macro_rules! mk_builder {
    ($($name:ident)::+, {
        $($field:ident = $val:expr;)*
    }) => {{
        $($name)::+::_new()
            $(.$field($val))*
            ._finish()
    }};
}
pub(crate) use mk_builder;

macro_rules! from_tbl_proxy {
    ({
        $(#[$meta:meta])*
        struct $name:ident {$(
            $(#[$fmeta:meta])*
            $field:ident: $fieldty:ty
        ),* $(,)?}
    }) => {
        #[derive(Clone, Debug)]
        $(#[$meta])*
        pub struct $name { pub table: mlua::Table }
        impl mlua::FromLua for $name {
            fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                mlua::FromLua::from_lua(value, lua).map(|table| Self { table })
            }
        }
        #[allow(non_snake_case)]
        impl $name {$(
            $(#[$fmeta])*
            pub fn $field(&self) -> mlua::Result<$fieldty> {
                self.table.get(stringify!($field))
            }
        )*}
    };
}
pub(crate) use from_tbl_proxy;
