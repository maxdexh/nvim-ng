use crate::prelude::*;
use std::marker::PhantomData;

// FIXME: Migrate to anyhow for backtrace support
pub type MluaError = mlua::Error;
pub type Error = anyhow::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn error_into_mlua(err: Error) -> MluaError {
    // TODO: Add backtrace
    err.into()
}

pub type LuaVal = mlua::Value;
pub type LuaString = mlua::LuaString;
pub type LuaNum = mlua::Number;
pub type LuaInt = mlua::Integer;
pub type LuaUnion<L, R> = mlua::Either<L, R>;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct Lua(mlua::Lua);
impl Lua {
    pub fn as_mlua(&self) -> &mlua::Lua {
        &self.0
    }
    pub fn by_mlua(lua: &mlua::Lua) -> &Self {
        // SAFETY: repr(transparent)
        unsafe { &*std::ptr::from_ref(lua).cast() }
    }
    pub fn from_mlua(lua: mlua::Lua) -> Self {
        Self(lua)
    }
    pub fn create_string(&self, s: impl AsRef<[u8]>) -> Result<LuaString> {
        self.as_mlua().create_string(s).map_err(Into::into)
    }
    pub fn create_table(&self) -> Result<LuaTableAny> {
        self.as_mlua()
            .create_table()
            .map(LuaTableAny)
            .map_err(Into::into)
    }
    pub fn create_sequence_from<T: mlua::IntoLua>(
        &self,
        iter: impl IntoIterator<Item = T>,
    ) -> Result<LuaTableAny> {
        self.as_mlua()
            .create_sequence_from(iter)
            .map(LuaTableAny)
            .map_err(Into::into)
    }
    pub fn create_table_from<K: mlua::IntoLua, V: mlua::IntoLua>(
        &self,
        iter: impl IntoIterator<Item = (K, V)>,
    ) -> Result<LuaTableAny> {
        self.as_mlua()
            .create_table_from(iter)
            .map(LuaTableAny)
            .map_err(Into::into)
    }
    pub fn convert<R: mlua::FromLua>(&self, val: impl mlua::IntoLua) -> Result<R> {
        self.as_mlua().convert(val).map_err(Into::into)
    }
    pub fn globals(&self) -> LuaTableAny {
        LuaTableAny(self.as_mlua().globals())
    }
}

#[derive(Clone, Copy, Default)]
pub struct LuaNil;
impl mlua::IntoLua for LuaNil {
    fn into_lua(self, _: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(mlua::Value::Nil)
    }
}
impl mlua::FromLua for LuaNil {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Nil => Ok(Self),
            _ => Err(MluaError::FromLuaConversionError {
                from: value.type_name(),
                to: std::any::type_name::<Self>().into(),
                message: None,
            }),
        }
    }
}

#[derive(Debug)]
pub enum LuaBottom {}

#[derive(Clone, Debug)]
pub struct LuaTableAny(mlua::Table);
impl LuaTableAny {
    pub fn get_any<R: mlua::FromLua>(&self, key: impl mlua::IntoLua) -> Result<R> {
        self.0.get(key).map_err(Into::into)
    }
    #[expect(dead_code)]
    pub fn raw_set_any(&self, key: impl mlua::IntoLua, val: impl mlua::IntoLua) -> Result<()> {
        self.0.raw_set(key, val).map_err(Into::into)
    }
    pub fn set_any(&self, key: impl mlua::IntoLua, val: impl mlua::IntoLua) -> Result<()> {
        self.0.set(key, val).map_err(Into::into)
    }
    pub fn push_any(&self, val: impl mlua::IntoLua) -> Result<()> {
        self.0.push(val).map_err(Into::into)
    }
    pub fn raw_push_any(&self, val: impl mlua::IntoLua) -> Result<()> {
        self.0.raw_push(val).map_err(Into::into)
    }
}
impl mlua::FromLua for LuaTableAny {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        mlua::FromLua::from_lua(value, lua).map(Self)
    }
}
impl mlua::IntoLua for LuaTableAny {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        mlua::IntoLua::into_lua(self.0, lua)
    }
}

impl mlua::IntoLua for LuaBottom {
    fn into_lua(self, _: &mlua::Lua) -> mlua::Result<mlua::Value> {
        match self {}
    }
}
impl mlua::FromLua for LuaBottom {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        Err(MluaError::FromLuaConversionError {
            from: value.type_name(),
            to: std::any::type_name::<Self>().into(),
            message: None,
        })
    }
}

#[derive(Clone, Debug)]
pub enum LuaCallableAny {
    Func(mlua::Function),
    Data(mlua::AnyUserData),
    Table(mlua::Table),
}
impl LuaCallableAny {
    pub fn call_any<R: mlua::FromLuaMulti>(&self, args: impl mlua::IntoLuaMulti) -> Result<R> {
        use mlua::ObjectLike as _;
        match self {
            Self::Func(func) => func.call(args),
            Self::Data(data) => data.call(args),
            Self::Table(table) => table.call(args),
        }
        .map_err(Into::into)
    }
}
impl mlua::IntoLua for LuaCallableAny {
    fn into_lua(self, _: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(match self {
            Self::Func(func) => mlua::Value::Function(func),
            Self::Data(data) => mlua::Value::UserData(data),
            Self::Table(table) => mlua::Value::Table(table),
        })
    }
}
impl mlua::FromLua for LuaCallableAny {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        Ok(match value {
            mlua::Value::Table(table) => Self::Table(table),
            mlua::Value::Function(func) => Self::Func(func),
            mlua::Value::UserData(data) => Self::Data(data),
            _ => {
                return Err(MluaError::FromLuaConversionError {
                    from: value.type_name(),
                    to: std::any::type_name::<Self>().into(),
                    message: Some("expected callable value type".into()),
                });
            }
        })
    }
}

pub struct LuaCallable<A, R>(LuaCallableAny, PhantomData<fn(A) -> R>);
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
impl<A, R> mlua::FromLua for LuaCallable<A, R> {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        Ok(Self::from_any(lua.unpack(value)?))
    }
}
impl<A, R> mlua::IntoLua for LuaCallable<A, R> {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.pack(self.into_any())
    }
}
impl<A, R> LuaCallable<A, R> {
    pub fn call_any_ret<U: mlua::FromLuaMulti>(self, args: impl LuaSubMulti<A>) -> Result<U>
    where
        A: FromLuaMultiTyped,
    {
        self.as_any().call_any(args)
    }

    pub fn call(&self, args: impl LuaSubMulti<A>) -> Result<R>
    where
        A: FromLuaMultiTyped,
        R: mlua::FromLuaMulti,
    {
        // TODO: Optional validation
        self.as_any().call_any(args)
    }

    pub fn from_any(func: LuaCallableAny) -> Self {
        Self(func, PhantomData)
    }
    pub fn from_mlua_func(func: mlua::Function) -> Self {
        Self::from_any(LuaCallableAny::Func(func))
    }
    pub fn into_any(self) -> LuaCallableAny {
        self.0
    }
    pub fn as_any(&self) -> &LuaCallableAny {
        &self.0
    }
}

pub struct LuaMap<K, V>(LuaTableAny, PhantomData<fn() -> (K, V)>);
pub struct LuaMapOwned<K, V>(LuaTableAny, PhantomData<fn() -> (K, V)>);
pub struct LuaMapMut<K, V>(
    LuaTableAny,
    #[allow(clippy::complexity)] PhantomData<fn(K, V) -> (K, V)>,
);
pub struct LuaSeq<T>(LuaTableAny, PhantomData<fn() -> T>);
pub struct LuaSeqMut<T>(LuaTableAny, PhantomData<fn(T) -> T>);
pub struct LuaSeqOwned<T>(LuaTableAny, PhantomData<fn() -> T>);
pub trait LuaTableGet {
    type Key;
    type Val;

    fn get(&self, key: impl LuaSub<Self::Key>) -> Result<Self::Val>;
}
pub trait LuaTableSet: LuaTableGet {
    fn set(&self, key: impl LuaSub<Self::Key>, val: impl LuaSub<Self::Val>) -> Result<()>;
}
impl<T: LuaTableGet> LuaTableGet for &T {
    type Key = T::Key;
    type Val = T::Val;

    fn get(&self, key: impl LuaSub<Self::Key>) -> Result<Self::Val> {
        T::get(self, key)
    }
}
impl<T: LuaTableSet> LuaTableSet for &T {
    fn set(&self, key: impl LuaSub<Self::Key>, val: impl LuaSub<Self::Val>) -> Result<()> {
        T::set(self, key, val)
    }
}
// TODO: Remove?
impl LuaTableGet for LuaTableAny {
    type Key = mlua::Value;
    type Val = mlua::Value;
    fn get(&self, key: impl LuaSub<Self::Key>) -> Result<Self::Val> {
        self.get_any(key)
    }
}
impl LuaTableSet for LuaTableAny {
    fn set(&self, key: impl LuaSub<Self::Key>, val: impl LuaSub<Self::Val>) -> Result<()> {
        self.set_any(key, val)
    }
}

const _: () = {
    macro_rules! g_tbl_prox_impl_base {
        ((gp![$($g:tt)*], $t:ty, ($k:ty, $v:ty $(,)?) $(,)?)) => {
            impl<$($g)*> mlua::IntoLua for $t {
                fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
                    self.into_table_any().into_lua(lua)
                }
            }
            #[allow(dead_code)]
            impl<$($g)*> $t {
                pub fn new(lua: &Lua) -> crate::lua::Result<Self> {
                    lua.create_table().map(Self::cast_table_any)
                }
                pub fn cast_table_any(table: LuaTableAny) -> Self {
                    Self(table, Default::default())
                }
                pub fn as_table_any(&self) -> &LuaTableAny {
                    &self.0
                }
                pub fn into_table_any(self) -> LuaTableAny {
                    self.0
                }
            }
            impl<$($g)*> LuaTableGet for $t
                where
                    $k: mlua::FromLua,
                    $v: mlua::FromLua,
            {
                type Key = $k;
                type Val = $v;

                fn get(&self, k: impl LuaSub<$k>) -> Result<$v> {
                    self.0.get_any(k)
                }
            }
        };
    }
    macro_rules! g_tbl_prox_impl_const {
        ((gp![$($g:tt)*], $t:ty, ($k:ty, $v:ty $(,)?) $(,)?)) => {
            g_tbl_prox_impl_base![(gp![$($g)*], $t, ($k, $v))];

            impl<$($g)*> mlua::FromLua for $t
            where
                $k: FromLuaTyped,
                $v: FromLuaTyped,
            {
                fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                    // TODO: Optional Validation
                    Ok(Self::cast_table_any(lua.unpack(value)?))
                }
            }
        };
    }
    macro_rules! g_tbl_prox_impl_mut {
        ((gp![$($g:tt)*], $t:ty, ($k:ty, $v:ty $(,)?) $(,)?)) => {
            g_tbl_prox_impl_base![(gp![$($g)*], $t, ($k, $v))];

            impl<$($g)*> LuaTableSet for $t
            where
                $k: IntoLuaTyped + FromLuaTyped,
                $v: IntoLuaTyped + FromLuaTyped,
            {
                fn set(&self, k: impl LuaSub<$k>, v: impl LuaSub<$v>) -> Result<()> {
                    // TODO: Optional Validation
                    self.0.set(k, v)
                }
            }
            impl<$($g)*> mlua::FromLua for $t
            where
                $k: IntoLuaTyped + FromLuaTyped,
                $v: IntoLuaTyped + FromLuaTyped,
            {
                fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
                    // TODO: Optional Validation
                    Ok(Self::cast_table_any(lua.unpack(value)?))
                }
            }
        };
    }
    macro_rules! g_tbl_prox_impl_owned {
        ((gp![$($g:tt)*], $t:ty, ($k:ty, $v:ty $(,)?) $(,)?)) => {
            g_tbl_prox_impl_base![(gp![$($g)*], $t, ($k, $v))];

            impl<$($g)*> LuaTableSet for $t
            where
                $k: FromLuaTyped,
                $v: FromLuaTyped,
            {
                fn set(&self, k: impl LuaSub<$k>, v: impl LuaSub<$v>) -> Result<()> {
                    // TODO: Optional Validation
                    self.0.set(k, v)
                }
            }
        };
    }
    g_tbl_prox_impl_const![(gp![K, V], LuaMap<K, V>, (K, V))];
    g_tbl_prox_impl_mut![(gp![K, V], LuaMapMut<K, V>, (K, V))];
    g_tbl_prox_impl_owned![(gp![K, V], LuaMapOwned<K, V>, (K, V))];
    g_tbl_prox_impl_const![(gp![T], LuaSeq<T>, (LuaInt, T))];
    g_tbl_prox_impl_mut![(gp![T], LuaSeqMut<T>, (LuaInt, T))];
    g_tbl_prox_impl_owned![(gp![T], LuaSeqOwned<T>, (LuaInt, T))];
};

pub struct LuaDefer<F>(pub F);
impl<T, F: FnOnce(&Lua) -> T> LuaDefer<F> {
    pub fn eval(self, lua: impl AsLua) -> T {
        self.0(lua.as_lua())
    }
}
pub fn lua_defer_val<T, F: FnOnce(&Lua) -> Result<T>>(f: F) -> LuaDefer<F> {
    LuaDefer(f)
}
macro_rules! LuaDeferImpl {
    ($res:ty) => {
        crate::lua::LuaDefer<impl FnOnce(&crate::lua::Lua) -> crate::lua::Result<$res>>
    };
}
pub(crate) use LuaDeferImpl;
impl<T: mlua::IntoLua, F: FnOnce(&Lua) -> Result<T>> mlua::IntoLua for LuaDefer<F> {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<LuaVal> {
        self.eval(lua)?.into_lua(lua)
    }
}

pub struct LuaDeferErr<T>(pub Result<T>);
impl<T> LuaDeferErr<T> {
    pub fn into_result(self) -> Result<T> {
        self.0
    }
}
impl<T> mlua::IntoLua for LuaDeferErr<T>
where
    T: mlua::IntoLua,
{
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.0?.into_lua(lua)
    }
}

#[allow(dead_code)]
pub struct LuaCastIntoAny<T>(pub T);
impl<T: mlua::IntoLua> mlua::IntoLua for LuaCastIntoAny<T> {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.0.into_lua(lua)
    }
}

pub fn lua_conv_sub<U: FromLuaTyped>(lua: &Lua, val: impl LuaSub<U>) -> Result<U> {
    lua.convert(val)
}

pub trait LuaStructInner: Sized {
    // FIXME: Add mandatory validation for field names, since we do not have nominals
    #[expect(unused)]
    const FIELD_NAMES: &[&[u8]];

    type Fields: FromLuaMultiTyped + IntoLuaMultiTyped;
}
pub struct LuaStruct<T: LuaStructInner>(pub T);

impl<T: LuaStructInner + mlua::IntoLua> mlua::IntoLua for LuaStruct<T> {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        self.0.into_lua(lua)
    }
}
impl<T: LuaStructInner + mlua::FromLua> mlua::FromLua for LuaStruct<T> {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        mlua::FromLua::from_lua(value, lua).map(LuaStruct)
    }
}

pub trait AsLua {
    fn as_lua(&self) -> &Lua;
}
impl<T: AsLua> AsLua for &T {
    fn as_lua(&self) -> &Lua {
        T::as_lua(self)
    }
}
impl AsLua for Lua {
    fn as_lua(&self) -> &Lua {
        self
    }
}
impl AsLua for mlua::Lua {
    fn as_lua(&self) -> &Lua {
        Lua::by_mlua(self)
    }
}
