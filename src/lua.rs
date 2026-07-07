use std::marker::PhantomData;

use crate::typing::{FromLuaMultiTyped, FromLuaTyped, IntoLuaTyped, LuaSub, LuaSubMulti};
use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, Lua, ObjectLike};

pub type LuaError = mlua::Error;
pub type Result<T, E = LuaError> = std::result::Result<T, E>;

pub type LuaValue = mlua::Value;
pub type LuaString = mlua::String;
// FIXME: Replace with type-safe alternatives everywhere
pub type LuaTableAny = mlua::Table;
pub type LuaFuncAny = mlua::Function;
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
    Func(LuaFuncAny),
    Data(mlua::AnyUserData),
    Table(LuaTableAny),
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
                    to: std::any::type_name::<Self>().into(),
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
    pub fn from_any_func(func: LuaFuncAny) -> Self {
        Self::from_any(LuaMaybeCallable::Func(func))
    }
    pub fn into_any(self) -> LuaMaybeCallable {
        self.0
    }
    pub fn as_any(&self) -> &LuaMaybeCallable {
        &self.0
    }
}

pub struct LuaTableMap<K, V>(LuaTableAny, PhantomData<fn() -> (K, V)>);
pub struct LuaTableMapOwned<K, V>(LuaTableAny, PhantomData<fn() -> (K, V)>);
pub struct LuaTableMapMut<K, V>(LuaTableAny, PhantomData<fn(K, V) -> (K, V)>);
pub struct LuaTableSeq<T>(LuaTableAny, PhantomData<fn() -> T>);
pub struct LuaTableSeqMut<T>(LuaTableAny, PhantomData<fn(T) -> T>);
pub struct LuaTableSeqOwned<T>(LuaTableAny, PhantomData<fn() -> T>);
pub trait LuaMutTable {
    type Key: FromLuaTyped;
    type Val: FromLuaTyped;

    fn set(&self, key: impl LuaSub<Self::Key>, val: impl LuaSub<Self::Val>) -> Result<()>;
}
impl LuaMutTable for LuaTableAny {
    type Key = LuaValue;
    type Val = LuaValue;
    fn set(&self, key: impl LuaSub<Self::Key>, val: impl LuaSub<Self::Val>) -> Result<()> {
        ObjectLike::set(self, key, val)
    }
}
impl<T: LuaMutTable> LuaMutTable for &T {
    type Key = T::Key;
    type Val = T::Val;

    fn set(&self, key: impl LuaSub<Self::Key>, val: impl LuaSub<Self::Val>) -> Result<()> {
        T::set(self, key, val)
    }
}

const _: () = {
    macro_rules! g_tbl_prox_impl_read {
        ((gp![$($g:tt)*], $t:ty, ($k:ty, $v:ty $(,)?) $(,)?)) => {
            impl<$($g)*> $t {
                pub fn get(&self, k: impl LuaSub<$k>) -> Result<$v>
                where
                    $k: FromLuaTyped,
                    $v: FromLua,
                {
                    self.0.get(k)
                }
            }
        };
    }
    macro_rules! g_tbl_prox_impl_base {
        ((gp![$($g:tt)*], $t:ty, ($k:ty, $v:ty $(,)?) $(,)?)) => {
            impl<$($g)*> IntoLua for $t {
                fn into_lua(self, lua: &Lua) -> mlua::Result<mlua::Value> {
                    self.into_table_any().into_lua(lua)
                }
            }
            impl<$($g)*> $t {
                pub fn new(lua: &Lua) -> Result<Self> {
                    lua.create_table().map(Self::from_table_any)
                }
                pub fn from_table_any(table: LuaTableAny) -> Self {
                    Self(table, PhantomData)
                }
                pub fn into_table_any(self) -> LuaTableAny {
                    self.0
                }
            }
        };
    }
    macro_rules! g_tbl_prox_impl_const {
        ((gp![$($g:tt)*], $t:ty, ($k:ty, $v:ty $(,)?) $(,)?)) => {
            g_tbl_prox_impl_base![(gp![$($g)*], $t, ($k, $v))];
            g_tbl_prox_impl_read![(gp![$($g)*], $t, ($k, $v))];

            impl<$($g)*> FromLua for $t
            where
                $k: FromLuaTyped,
                $v: FromLuaTyped,
            {
                fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
                    // TODO: Optional Validation
                    Ok(Self::from_table_any(lua.unpack(value)?))
                }
            }
        };
    }
    macro_rules! g_tbl_prox_impl_mut {
        ((gp![$($g:tt)*], $t:ty, ($k:ty, $v:ty $(,)?) $(,)?)) => {
            g_tbl_prox_impl_base![(gp![$($g)*], $t, ($k, $v))];
            g_tbl_prox_impl_read![(gp![$($g)*], $t, ($k, $v))];

            impl<$($g)*> LuaMutTable for $t
            where
                $k: IntoLuaTyped + FromLuaTyped,
                $v: IntoLuaTyped + FromLuaTyped,
            {
                type Key = $k;
                type Val = $v;
                fn set(&self, k: impl LuaSub<$k>, v: impl LuaSub<$v>) -> Result<()> {
                    // TODO: Optional Validation
                    self.0.set(k, v)
                }
            }
            impl<$($g)*> FromLua for $t
            where
                $k: IntoLuaTyped + FromLuaTyped,
                $v: IntoLuaTyped + FromLuaTyped,
            {
                fn from_lua(value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
                    // TODO: Optional Validation
                    Ok(Self::from_table_any(lua.unpack(value)?))
                }
            }
        };
    }
    g_tbl_prox_impl_const![(gp![K, V], LuaTableMap<K, V>, (K, V))];
    g_tbl_prox_impl_mut![(gp![K, V], LuaTableMapMut<K, V>, (K, V))];
    g_tbl_prox_impl_base![(gp![K, V], LuaTableMapOwned<K, V>, (K, V))];
    g_tbl_prox_impl_const![(gp![T], LuaTableSeq<T>, (LuaInt, T))];
    g_tbl_prox_impl_mut![(gp![T], LuaTableSeqMut<T>, (LuaInt, T))];
    g_tbl_prox_impl_base![(gp![T], LuaTableSeqOwned<T>, (LuaInt, T))];
};
impl<T> LuaTableSeqMut<T> {
    pub fn push(&self, val: impl LuaSub<T>) -> Result<()>
    where
        T: FromLuaTyped,
    {
        self.0.push(val)
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
    fn into_lua(self, lua: &Lua) -> Result<LuaValue> {
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
