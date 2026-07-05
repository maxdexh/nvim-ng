use std::marker::PhantomData;

use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, Lua, ObjectLike};

pub type LuaError = mlua::Error;
pub type Result<T, E = LuaError> = std::result::Result<T, E>;

pub type LuaFunc = mlua::Function;
pub type LuaString = mlua::String;
pub type LuaTable = mlua::Table;
pub type LuaValue = mlua::Value;
pub type LuaNum = mlua::Number;
pub type LuaInt = mlua::Integer;

pub type LuaNil = Option<LuaBottom>;
#[expect(non_upper_case_globals)]
pub const LuaNil: LuaNil = LuaNil::None;

#[derive(Debug)]
pub enum LuaBottom {}

impl IntoLua for LuaBottom {
    fn into_lua(self, _: &Lua) -> mlua::Result<mlua::Value> {
        match self {}
    }
}

pub trait LuaSub<Sub>: IntoLua {}

macro_rules! lua_sub {
    ($dst:ty, [$($t:ty),* $(,)?]) => {
        $(
            impl LuaSub<$dst> for $t {}
        )*
    };
}

lua_sub!(LuaTable, [LuaTable]);
lua_sub!(LuaString, [LuaString, &str, String]);
lua_sub!(LuaInt, [i8, u8, i16, u16, i32, u32, i64, u64, isize, usize]);
lua_sub!(
    LuaNum,
    [f64, f32, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize]
);
lua_sub!(
    LuaValue,
    [
        LuaTable, LuaString, &str, String, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize,
        LuaValue, LuaFunc
    ]
);
impl<T: LuaSub<U>, U> LuaSub<Option<U>> for Option<T> {}
impl<T> LuaSub<T> for LuaBottom {}
impl<T: IntoLua, const N: usize> LuaSub<LuaTable> for [T; N] {}

#[derive(Clone, Debug)]
pub enum LuaAnyCallable {
    Func(LuaFunc),
    Data(mlua::AnyUserData),
    Table(LuaTable),
}
impl LuaAnyCallable {
    pub fn call_any<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> Result<R> {
        match self {
            Self::Func(func) => func.call(args),
            Self::Data(data) => data.call(args),
            Self::Table(table) => table.call(args),
        }
    }
}
impl IntoLua for LuaAnyCallable {
    fn into_lua(self, _: &Lua) -> mlua::Result<mlua::Value> {
        Ok(match self {
            Self::Func(func) => mlua::Value::Function(func),
            Self::Data(data) => mlua::Value::UserData(data),
            Self::Table(table) => mlua::Value::Table(table),
        })
    }
}
impl FromLua for LuaAnyCallable {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        Ok(match value {
            mlua::Value::Table(table) => Self::Table(table),
            mlua::Value::Function(func) => Self::Func(func),
            mlua::Value::UserData(data) => Self::Data(data),
            _ => {
                return Err(LuaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: "LuaAnyCallable".into(),
                    message: Some("expected callable value type".into()),
                });
            }
        })
    }
}

pub trait LuaSubMulti<Sub>: IntoLuaMulti {}
impl<U, T: LuaSub<U>> LuaSubMulti<U> for T {}
impl LuaSubMulti<()> for () {}
macro_rules! impl_tuple {
    () => {};
    (($TL:ident, $UL:ident) $(,($T:ident, $U:ident))* $(,)?) => {
        impl_tuple!($(($T, $U)),*);
        impl<$($U, $T: LuaSub<$U>,)* $UL, $TL: LuaSubMulti<$UL>>
            LuaSubMulti<($($U,)* $UL,)>
        for ($($T,)* $TL,) {}
        impl<$($T: mlua::IntoLua,)* $TL: mlua::IntoLuaMulti>
            LuaSubMulti<mlua::MultiValue>
        for ($($T,)* $TL,) {}
    };
}
impl_tuple!(
    (TP, UP),
    (TO, UO),
    (TN, UN),
    (TM, UM),
    (TL, UL),
    (TK, UK),
    (TJ, UJ),
    (TI, UI),
    (TH, UH),
    (TG, UG),
    (TF, UF),
    (TE, UE),
    (TD, UD),
    (TC, UC),
    (TB, UB),
    (TA, UA),
);

pub struct LuaCallable<A, R>(LuaAnyCallable, PhantomData<fn(A) -> R>);
impl<A, R> std::fmt::Debug for LuaCallable<A, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl<A, R> Clone for LuaCallable<A, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}
impl<A, R> FromLua for LuaCallable<A, R> {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        Ok(Self::from_any(lua.unpack(value)?))
    }
}
impl<A, R> IntoLua for LuaCallable<A, R> {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        lua.pack(self.into_any())
    }
}
impl<A, R> LuaCallable<A, R> {
    pub fn call_any_ret<U: FromLuaMulti>(self, args: impl LuaSubMulti<A>) -> Result<U> {
        self.as_any().call_any(args)
    }

    pub fn call(self, args: impl LuaSubMulti<A>) -> Result<R>
    where
        R: FromLuaMulti,
    {
        self.as_any().call_any(args)
    }

    pub fn from_any(func: LuaAnyCallable) -> Self {
        Self(func, PhantomData)
    }
    pub fn into_any(self) -> LuaAnyCallable {
        self.0
    }
    pub fn as_any(&self) -> &LuaAnyCallable {
        &self.0
    }
}

pub struct LuaTableMap<K, V>(
    LuaTable,
    #[allow(clippy::complexity)] PhantomData<fn((K, V)) -> (K, V)>,
);
impl<K, V> LuaTableMap<K, V> {
    pub fn from_table(func: LuaTable) -> Self {
        Self(func, PhantomData)
    }
    pub fn into_table(self) -> LuaTable {
        self.0
    }
    pub fn as_table(&self) -> &LuaTable {
        &self.0
    }

    pub fn get(&self, k: impl LuaSub<K>) -> Result<V>
    where
        V: FromLua,
    {
        self.as_table().get(k)
    }
    #[expect(unused)]
    pub fn set(&self, k: impl LuaSub<K>, v: impl LuaSub<V>) -> Result<()> {
        self.as_table().set(k, v)
    }
}
impl<K, V> FromLua for LuaTableMap<K, V> {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        Ok(Self::from_table(lua.unpack(value)?))
    }
}
impl<K, V> IntoLua for LuaTableMap<K, V> {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        self.into_table().into_lua(lua)
    }
}
