use crate::prelude::*;

pub fn do_try<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    f()
}

pub type LuaDict<V> = LuaMap<LuaString, V>;
pub type LuaDictMut<V> = LuaMapMut<LuaString, V>;

#[doc(hidden)]
pub mod __tbl {
    use crate::lua::{LuaMapOwned, LuaSeqOwned};
    use crate::prelude::*;
    use mlua::IntoLua;

    pub struct TableBuilder<K, V>(LuaMapOwned<K, V>);
    pub fn builder_new(lua: &Lua) -> Result<TableBuilder<LuaBottom, LuaBottom>> {
        lua.create_table()
            .map(LuaMapOwned::cast_mlua_table)
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
        type Out = TableBuilder<LuaUnion<Key, K>, LuaUnion<Val, V>>;
        fn __with(self, key: K, val: V) -> Result<Self::Out> {
            let table = self.0.into_table_any();
            table.set(key, val)?;
            Ok(TableBuilder(LuaMapOwned::cast_mlua_table(table)))
        }
    }
    impl<K, V> TableBuildFinish for TableBuilder<K, V> {
        type Finish = LuaMapOwned<K, V>;
        fn __finish(self) -> Self::Finish {
            self.0
        }
    }
    impl<T: LuaTableSet, K: LuaSub<T::Key>, V: LuaSub<T::Val>> TableWith<K, V> for &T {
        type Out = Self;
        fn __with(self, key: K, val: V) -> Result<Self::Out> {
            self.set(key, val)?;
            Ok(self)
        }
    }
    impl<T: LuaTableSet> TableBuildFinish for T {
        type Finish = ();
        fn __finish(self) -> Self::Finish {}
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
    (@key_or_field $field:literal) => { $field };
    (@key_or_field [$key:expr]) => { $key };
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

    fn ok_or_notify(self, env: impl AsRef<Nvim>) -> Option<Self::Ok>
    where
        Self::Err: Into<LuaError>;
}
impl<T, E> ResultExt for std::result::Result<T, E> {
    type Ok = T;
    type Err = E;

    fn ok_or_notify(self, env: impl AsRef<Nvim>) -> Option<T>
    where
        E: Into<LuaError>,
    {
        let res = self.map_err(Into::into);
        match res {
            Ok(ok) => Some(ok),
            Err(err) => {
                env.as_ref().notify_err(&err);
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
                pub fn _finish(self) -> crate::lua::lua_defer_impl!(crate::lua::LuaStruct::<$gname>) {
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
