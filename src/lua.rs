use std::marker::PhantomData;

use crate::typing::{FromLuaMultiTyped, FromLuaTyped, LuaSub, LuaSubMulti};
use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, Lua, ObjectLike};

pub type LuaError = mlua::Error;
pub type Result<T, E = LuaError> = std::result::Result<T, E>;

pub type LuaTop = mlua::Value;
pub type LuaString = mlua::String;
pub type LuaTopTable = mlua::Table;
pub type LuaTopFunc = mlua::Function;
pub type LuaNum = mlua::Number;
pub type LuaInt = mlua::Integer;
pub type LuaEither<L, R> = mlua::Either<L, R>;

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
impl FromLua for LuaBottom {
    fn from_lua(value: mlua::Value, _: &Lua) -> mlua::Result<Self> {
        Err(LuaError::FromLuaConversionError {
            from: value.type_name(),
            to: "Bottom".into(),
            message: Some("Conversion to bottom type always fails".into()),
        })
    }
}

#[derive(Clone, Debug)]
pub enum LuaMaybeCallable {
    Func(LuaTopFunc),
    Data(mlua::AnyUserData),
    Table(LuaTopTable),
}
impl LuaMaybeCallable {
    pub fn call_any<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> Result<R> {
        match self {
            Self::Func(func) => func.call(args),
            Self::Data(data) => data.call(args),
            Self::Table(table) => table.call(args),
        }
    }
}
impl IntoLua for LuaMaybeCallable {
    fn into_lua(self, _: &Lua) -> mlua::Result<mlua::Value> {
        Ok(match self {
            Self::Func(func) => mlua::Value::Function(func),
            Self::Data(data) => mlua::Value::UserData(data),
            Self::Table(table) => mlua::Value::Table(table),
        })
    }
}
impl FromLua for LuaMaybeCallable {
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

pub struct LuaCallable<A, R>(LuaMaybeCallable, PhantomData<fn(A) -> R>);
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
    pub fn call_any_ret<U: FromLuaMulti>(self, args: impl LuaSubMulti<A>) -> Result<U>
    where
        A: FromLuaMultiTyped,
    {
        self.as_any().call_any(args)
    }

    pub fn call(self, args: impl LuaSubMulti<A>) -> Result<R>
    where
        A: FromLuaMultiTyped,
        R: FromLuaMulti,
    {
        self.as_any().call_any(args)
    }

    pub fn from_any(func: LuaMaybeCallable) -> Self {
        Self(func, PhantomData)
    }
    pub fn from_any_func(func: LuaTopFunc) -> Self {
        Self::from_any(LuaMaybeCallable::Func(func))
    }
    pub fn into_any(self) -> LuaMaybeCallable {
        self.0
    }
    pub fn as_any(&self) -> &LuaMaybeCallable {
        &self.0
    }
}

pub struct LuaTableMap<K, V>(LuaTopTable, PhantomData<fn() -> (K, V)>);
impl<K, V> LuaTableMap<K, V> {
    pub fn from_table(func: LuaTopTable) -> Self {
        Self(func, PhantomData)
    }
    pub fn into_table(self) -> LuaTopTable {
        self.0
    }
    pub fn as_table(&self) -> &LuaTopTable {
        &self.0
    }

    pub fn get(&self, k: impl LuaSub<K>) -> Result<V>
    where
        K: FromLuaTyped,
        V: FromLua,
    {
        self.as_table().get(k)
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

pub struct LuaTableSeq<T>(
    LuaTopTable,
    #[allow(clippy::complexity)] PhantomData<fn() -> T>,
);
impl<T> LuaTableSeq<T> {
    pub fn from_table(func: LuaTopTable) -> Self {
        Self(func, PhantomData)
    }
    pub fn into_table(self) -> LuaTopTable {
        self.0
    }
}
impl<T> FromLua for LuaTableSeq<T> {
    fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        Ok(Self::from_table(lua.unpack(value)?))
    }
}
impl<T> IntoLua for LuaTableSeq<T> {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        self.into_table().into_lua(lua)
    }
}

pub struct LuaDefer<F>(pub F);
impl<T, F: FnOnce(&Lua) -> T> LuaDefer<F> {
    pub fn eval(self, lua: &Lua) -> T {
        self.0(lua)
    }
}
pub fn defer_lua_val<T, F: FnOnce(&Lua) -> Result<T>>(f: F) -> LuaDefer<F> {
    LuaDefer(f)
}
impl<T: IntoLua, F: FnOnce(&Lua) -> Result<T>> IntoLua for LuaDefer<F> {
    fn into_lua(self, lua: &Lua) -> Result<LuaTop> {
        self.0(lua)?.into_lua(lua)
    }
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

pub struct LuaCastIntoAny<T>(pub T);
impl<T: IntoLua> IntoLua for LuaCastIntoAny<T> {
    fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
        self.0.into_lua(lua)
    }
}

pub fn lua_conv_sub<U: FromLuaTyped>(lua: &Lua, val: impl LuaSub<U>) -> Result<U> {
    lua.convert(val)
}
