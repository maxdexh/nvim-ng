pub mod logic {
    pub trait Bool {
        type And<R: Bool>: Bool;
        type Or<R: Bool>: Bool;
        type Not: Bool;
    }
    pub struct True;
    pub struct False;
    impl Bool for True {
        type And<R: Bool> = R;
        type Or<R: Bool> = True;
        type Not = False;
    }
    impl Bool for False {
        type And<R: Bool> = False;
        type Or<R: Bool> = R;
        type Not = True;
    }
    pub type And<L, R> = <L as Bool>::And<R>;
    pub type Or<L, R> = <L as Bool>::Or<R>;
}
use logic::*;

pub trait FromLuaTyped: mlua::FromLua {
    type IsFrom<Src: IntoLuaTyped>: Bool;
}
pub trait IntoLuaTyped: mlua::IntoLua {
    type IsInto<Dst: FromLuaTyped>: Bool;

    type StripNil: IntoLuaTyped;

    type IsIntoTable: Bool;
    type IsIntoNum: Bool;
    type IsIntoInt: Bool;
    type IsIntoStr: Bool;
    type IsIntoFunc: Bool;
    type IsIntoBool: Bool;

    type IsIntoTableSeqConst<I: FromLuaTyped>: Bool;
    type IsIntoTableSeqMut<I: IntoLuaTyped + FromLuaTyped>: Bool;
    type IsIntoTableMapConst<K: FromLuaTyped, V: FromLuaTyped>: Bool;
    type IsIntoTableMapMut<K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped>: Bool;
    type IsIntoCallWith<A: IntoLuaMultiTyped, R: FromLuaMultiTyped>: Bool;
}
pub type IsInto<Src, Dst> = <Src as IntoLuaTyped>::IsInto<Dst>;
pub type IsEquiv<T, U> = And<IsInto<T, U>, IsInto<U, T>>;
macro_rules! default_item {
    (IsInto) => {
        type IsInto<Dst: FromLuaTyped> = Dst::IsFrom<Self>;
    };
    (StripNil) => {
        type StripNil = Self;
    };
    (IsIntoTable) => {
        type IsIntoTable = crate::typing::False;
    };
    (IsIntoNum) => {
        type IsIntoNum = crate::typing::False;
    };
    (IsIntoInt) => {
        type IsIntoInt = crate::typing::False;
    };
    (IsIntoStr) => {
        type IsIntoStr = crate::typing::False;
    };
    (IsIntoFunc) => {
        type IsIntoFunc = crate::typing::False;
    };
    (IsIntoBool) => {
        type IsIntoBool = crate::typing::False;
    };
    (IsIntoTableSeqConst) => {
        type IsIntoTableSeqConst<_I: crate::typing::FromLuaTyped> = crate::typing::False;
    };
    (IsIntoTableSeqMut) => {
        type IsIntoTableSeqMut<_I: crate::typing::IntoLuaTyped + crate::typing::FromLuaTyped> =
            crate::typing::False;
    };
    (IsIntoTableMapConst) => {
        type IsIntoTableMapConst<_K: crate::typing::FromLuaTyped, _V: crate::typing::FromLuaTyped> =
            crate::typing::False;
    };
    (IsIntoTableMapMut) => {
        type IsIntoTableMapMut<
            _K: crate::typing::IntoLuaTyped + crate::typing::FromLuaTyped,
            _V: crate::typing::IntoLuaTyped + crate::typing::FromLuaTyped,
        > = crate::typing::False;
    };
    (IsIntoCallWith) => {
        type IsIntoCallWith<
            _A: crate::typing::IntoLuaMultiTyped,
            _R: crate::typing::FromLuaMultiTyped,
        > = crate::typing::False;
    };
    ($t:ident) => {
        compile_error! {concat!("Unknown item ", stringify!($t)) }
    };
}
macro_rules! mk_defaults_except {
    ($mac:ident, [$($item:ident),* $(,)?]) => {
        macro_rules! $mac {
            () => {
                $mac!(@sel IsInto);
                $mac!(@sel IsIntoTable);
                $mac!(@sel IsIntoNum);
                $mac!(@sel IsIntoInt);
                $mac!(@sel IsIntoStr);
                $mac!(@sel IsIntoFunc);
                $mac!(@sel IsIntoBool);
                $mac!(@sel IsIntoTableMapConst);
                $mac!(@sel IsIntoTableMapMut);
                $mac!(@sel IsIntoTableSeqConst);
                $mac!(@sel IsIntoTableSeqMut);
                $mac!(@sel IsIntoCallWith);
                $mac!(@sel StripNil);
            };
            $( (@sel $item) => {}; )*
            (@sel $other:ident) => {
                default_item!($other);
            }
        }
    };
}

mod from_impls {
    use super::*;

    impl FromLuaTyped for crate::lua::LuaValue {
        type IsFrom<Src: IntoLuaTyped> = True;
    }
    impl FromLuaTyped for crate::lua::LuaString {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoStr;
    }
    impl FromLuaTyped for crate::lua::LuaTableAny {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoTable;
    }
    impl FromLuaTyped for crate::lua::LuaFuncAny {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoFunc;
    }
    impl FromLuaTyped for bool {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoBool;
    }
    impl FromLuaTyped for crate::lua::LuaNum {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoNum;
    }
    impl FromLuaTyped for crate::lua::LuaInt {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoInt;
    }
    impl<A: IntoLuaMultiTyped, R: FromLuaMultiTyped> FromLuaTyped for crate::lua::LuaCallable<A, R> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoCallWith<A, R>;
    }
    impl<T: FromLuaTyped> FromLuaTyped for Option<T> {
        type IsFrom<Src: IntoLuaTyped> = IsInto<Src::StripNil, T>;
    }
    impl FromLuaTyped for crate::lua::LuaBottom {
        type IsFrom<Src: IntoLuaTyped> = False;
    }
    impl<T: FromLuaTyped> FromLuaTyped for crate::lua::LuaTableSeq<T> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoTableSeqConst<T>;
    }
    impl<T: IntoLuaTyped + FromLuaTyped> FromLuaTyped for crate::lua::LuaTableSeqMut<T> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoTableSeqMut<T>;
    }
    impl<K: FromLuaTyped, V: FromLuaTyped> FromLuaTyped for crate::lua::LuaTableMap<K, V> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoTableMapConst<K, V>;
    }
    impl<K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped> FromLuaTyped
        for crate::lua::LuaTableMapMut<K, V>
    {
        type IsFrom<Src: IntoLuaTyped> = Src::IsIntoTableMapMut<K, V>;
    }
    impl<L: FromLuaTyped, R: FromLuaTyped> FromLuaTyped for crate::lua::LuaEither<L, R> {
        type IsFrom<Src: IntoLuaTyped> = Or<Src::IsInto<L>, Src::IsInto<R>>;
    }
}

mod into_impls {
    use super::*;
    macro_rules! impl_into {
        (
            $([$($g:tt)*])?
            impl $t:ty;
            $( type $item:ident$([$($ig:tt)*])? = $ity:ty; )*
        ) => {
            const _: () = {
                mk_defaults_except!(default_items, [$($item),*]);
                impl$(<$($g)*>)? IntoLuaTyped for $t {
                    default_items!();
                    $(type $item$(<$($ig)*>)? = $ity;)*
                }
            };
        };
    }

    impl_into! {
        impl bool;
        type IsIntoBool = True;
    }
    impl_into! {
        impl crate::lua::LuaValue;
    }
    impl_into! {
        impl crate::lua::LuaString;
        type IsIntoStr = True;
    }
    impl_into! {
        impl &str;
        type IsIntoStr = True;
    }
    impl_into! {
        impl String;
        type IsIntoStr = True;
    }
    impl_into! {
        impl crate::lua::LuaTableAny;
        type IsIntoTable = True;
    }
    impl_into! {
        impl crate::lua::LuaFuncAny;
        type IsIntoFunc = True;
    }
    impl_into! {
        impl f32;
        type IsIntoNum = True;
    }
    impl_into! {
        impl f64;
        type IsIntoNum = True;
    }
    macro_rules! ints {
        ($($int:ty),* $(,)?) => {$(
            impl_into! {
                impl $int;
                type IsIntoNum = True;
                type IsIntoInt = True;
            }
        )*};
    }
    ints![i8, u8, i16, u16, i32, u32, i64, u64, isize, usize];
    impl_into! {
        [T: IntoLuaTyped, const N: usize]
        impl [T; N];
        type IsIntoTable = True;
        type IsIntoTableSeqConst[U: FromLuaTyped] = T::IsInto<U>;
        type IsIntoTableSeqMut[U: IntoLuaTyped + FromLuaTyped] = Self::IsIntoTableSeqConst<U>;
        type IsIntoTableMapConst[K: FromLuaTyped, V: FromLuaTyped] = And<IsInto<crate::lua::LuaInt, K>, T::IsInto<V>>;
        type IsIntoTableMapMut[K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped] = Self::IsIntoTableMapConst<K, V>;
    }
    impl_into! {
        [T: IntoLuaTyped]
        impl Option<T>;
        type StripNil = T::StripNil;
    }
    impl_into! {
        [T: FromLuaTyped + IntoLuaTyped]
        impl crate::lua::LuaTableSeq<T>;
        type IsIntoTable = True;
        type IsIntoTableSeqConst[U: FromLuaTyped] = T::IsInto<U>;
        type IsIntoTableMapConst[K: FromLuaTyped, V: FromLuaTyped] = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
    }
    impl_into! {
        [T: FromLuaTyped + IntoLuaTyped]
        impl crate::lua::LuaTableSeqMut<T>;
        type IsIntoTable = True;
        type IsIntoTableSeqConst[U: FromLuaTyped] = T::IsInto<U>;
        type IsIntoTableSeqMut[U: IntoLuaTyped + FromLuaTyped] = IsEquiv<T, U>;
        type IsIntoTableMapConst[K: FromLuaTyped, V: FromLuaTyped] = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type IsIntoTableMapMut[K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped] = And<IsEquiv<crate::lua::LuaInt, K>, IsEquiv<T, V>>;
    }
    impl_into! {
        [T: IntoLuaTyped]
        impl crate::lua::LuaTableSeqOwned<T>;
        type IsIntoTable = True;
        type IsIntoTableSeqConst[U: FromLuaTyped] = T::IsInto<U>;
        type IsIntoTableSeqMut[U: IntoLuaTyped + FromLuaTyped] = T::IsInto<U>;
        type IsIntoTableMapConst[K: FromLuaTyped, V: FromLuaTyped] = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type IsIntoTableMapMut[K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped] = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
    }
    impl_into! {
        [K: IntoLuaTyped, V: IntoLuaTyped]
        impl crate::lua::LuaTableMap<K, V>;
        type IsIntoTable = True;
        type IsIntoTableMapConst[DK: FromLuaTyped, DV: FromLuaTyped] = And<IsInto<K, DK>, IsInto<V, DV>>;
    }
    impl_into! {
        [K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped]
        impl crate::lua::LuaTableMapMut<K, V>;
        type IsIntoTable = True;
        type IsIntoTableMapConst[DK: FromLuaTyped, DV: FromLuaTyped] = And<IsInto<K, DK>, IsInto<V, DV>>;
        type IsIntoTableMapMut[DK: IntoLuaTyped + FromLuaTyped, DV: IntoLuaTyped + FromLuaTyped] = And<IsEquiv<K, DK>, IsEquiv<V, DV>>;
    }
    impl_into! {
        [K: IntoLuaTyped, V: IntoLuaTyped]
        impl crate::lua::LuaTableMapOwned<K, V>;
        type IsIntoTable = True;
        type IsIntoTableMapConst[DK: FromLuaTyped, DV: FromLuaTyped] = And<IsInto<K, DK>, IsInto<V, DV>>;
        type IsIntoTableMapMut[DK: IntoLuaTyped + FromLuaTyped, DV: IntoLuaTyped + FromLuaTyped] = And<IsInto<K, DK>, IsInto<V, DV>>;
    }
    impl_into! {
        [A: FromLuaMultiTyped, R: IntoLuaMultiTyped]
        impl crate::lua::LuaCallable<A, R>;
        type IsIntoCallWith[DA: IntoLuaMultiTyped, DR: FromLuaMultiTyped] = And<DA::IsIntoMulti<A>, R::IsIntoMulti<DR>>;
    }
    #[rustfmt::skip]
    macro_rules! defer_by {
        ($mac:ident) => {
            type IsInto<Dst: FromLuaTyped> = $mac!(IsInto<Dst>);
            type IsIntoTable = $mac!(IsIntoTable);
            type IsIntoNum = $mac!(IsIntoNum);
            type IsIntoInt = $mac!(IsIntoInt);
            type IsIntoStr = $mac!(IsIntoStr);
            type IsIntoFunc = $mac!(IsIntoFunc);
            type IsIntoBool = $mac!(IsIntoBool);
            type IsIntoTableSeqConst<_I: FromLuaTyped> = $mac!(IsIntoTableSeqConst<_I>);
            type IsIntoTableMapConst<_K: FromLuaTyped, _V: FromLuaTyped> = $mac!(IsIntoTableMapConst<_K, _V>);
            type IsIntoTableSeqMut<_I: IntoLuaTyped + FromLuaTyped> = $mac!(IsIntoTableSeqMut<_I>);
            type IsIntoTableMapMut<_K: IntoLuaTyped + FromLuaTyped, _V: IntoLuaTyped + FromLuaTyped> = $mac!(IsIntoTableMapMut<_K, _V>);
            type IsIntoCallWith<_A: IntoLuaMultiTyped, _R: FromLuaMultiTyped> = $mac!(IsIntoCallWith<_A, _R>);
        };
    }
    macro_rules! defer_to_true {
        ($($postfix:tt)*) => {
            True
        };
    }
    impl IntoLuaTyped for crate::lua::LuaBottom {
        type StripNil = Self;
        defer_by!(defer_to_true);
    }
    impl<T: mlua::IntoLua> IntoLuaTyped for crate::lua::LuaCastIntoAny<T> {
        type StripNil = Self;
        defer_by!(defer_to_true);
    }
    macro_rules! defer_to_T {
        ($($postfix:tt)*) => {
            T::$($postfix)*
        };
    }
    impl<T: IntoLuaTyped> IntoLuaTyped for crate::lua::LuaDeferErr<T> {
        type StripNil = T::StripNil;
        defer_by!(defer_to_T);
    }
    impl<T: IntoLuaTyped, F: FnOnce(&mlua::Lua) -> mlua::Result<T>> IntoLuaTyped
        for crate::lua::LuaDefer<F>
    {
        type StripNil = T::StripNil;
        defer_by!(defer_to_T);
    }
    macro_rules! defer_to_LR_and {
        ($($postfix:tt)*) => {
            And<L::$($postfix)*, R::$($postfix)*>
        };
    }
    impl<L: IntoLuaTyped, R: IntoLuaTyped> IntoLuaTyped for crate::lua::LuaEither<L, R> {
        type StripNil = crate::lua::LuaEither<L::StripNil, R::StripNil>;
        defer_by!(defer_to_LR_and);
    }
}

pub trait IntoLuaMultiTyped: mlua::IntoLuaMulti {
    type IsIntoMulti<Dst: FromLuaMultiTyped>: Bool;
    type IsIntoVariadic<T: FromLuaTyped>: Bool;

    type Head: IntoLuaTyped;
    type Tail: IntoLuaMultiTyped;
}
pub trait FromLuaMultiTyped: mlua::FromLuaMulti {
    type IsFromMulti<Src: IntoLuaMultiTyped>: Bool;
}
pub type IsIntoMulti<Src, Dst> = <Src as IntoLuaMultiTyped>::IsIntoMulti<Dst>;
mod multi_impls {
    use super::*;

    // (A, B, C) <:> (A, (B, C))
    // () <: ()
    // () <:> (Nil,)
    // (SH, ST) <: (DH, DT) iff SH <: DH, ST <: DT
    macro_rules! nest_tuple_var {
        () => {
            ()
        };
        ($Tail:ty) => { $Tail };
        ($H:ty $(,$T:ty)* $(,)?) => {
            ($H, nest_tuple_var!($($T),*))
        };
    }
    #[diagnostic::do_not_recommend]
    impl<T: FromLuaTyped> FromLuaMultiTyped for T {
        type IsFromMulti<Src: IntoLuaMultiTyped> = <Src::Head as IntoLuaTyped>::IsInto<Self>;
    }
    impl FromLuaMultiTyped for () {
        type IsFromMulti<Src: IntoLuaMultiTyped> = True;
    }
    impl<Head: FromLuaTyped, Tail: FromLuaMultiTyped> FromLuaMultiTyped for (Head, Tail) {
        type IsFromMulti<Src: IntoLuaMultiTyped> =
            And<IsInto<Src::Head, Head>, IsIntoMulti<Src::Tail, Tail>>;
    }
    impl<Tail: FromLuaMultiTyped> FromLuaMultiTyped for (Tail,) {
        type IsFromMulti<Src: IntoLuaMultiTyped> = IsIntoMulti<Src, Tail>;
    }
    impl<T: FromLuaTyped> FromLuaMultiTyped for mlua::Variadic<T> {
        type IsFromMulti<Src: IntoLuaMultiTyped> = Src::IsIntoVariadic<T>;
    }

    macro_rules! defer_into {
        ($t:ty) => {
            type IsIntoMulti<_Dst: FromLuaMultiTyped> =
                <$t as IntoLuaMultiTyped>::IsIntoMulti<_Dst>;
            type IsIntoVariadic<_T: FromLuaTyped> = <$t as IntoLuaMultiTyped>::IsIntoVariadic<_T>;

            type Head = <$t as IntoLuaMultiTyped>::Head;
            type Tail = <$t as IntoLuaMultiTyped>::Tail;
        };
    }

    #[diagnostic::do_not_recommend]
    impl<T: IntoLuaTyped> IntoLuaMultiTyped for T {
        type IsIntoMulti<Dst: FromLuaMultiTyped> = Dst::IsFromMulti<Self>;
        type IsIntoVariadic<DT: FromLuaTyped> = T::IsInto<DT>;
        type Head = T;
        type Tail = ();
    }
    impl IntoLuaMultiTyped for () {
        type IsIntoMulti<Dst: FromLuaMultiTyped> = Dst::IsFromMulti<Self>;
        type IsIntoVariadic<T: FromLuaTyped> = True;

        type Head = crate::lua::LuaNil;
        type Tail = Self;
    }
    impl<Head: IntoLuaTyped, Tail: IntoLuaMultiTyped> IntoLuaMultiTyped for (Head, Tail) {
        type IsIntoMulti<Dst: FromLuaMultiTyped> = Dst::IsFromMulti<Self>;
        type IsIntoVariadic<T: FromLuaTyped> = And<Head::IsInto<T>, Tail::IsIntoVariadic<T>>;

        type Head = Head;
        type Tail = Tail;
    }
    impl<Tail: IntoLuaMultiTyped> IntoLuaMultiTyped for (Tail,) {
        defer_into!(Tail);
    }
    impl<T: IntoLuaTyped> IntoLuaMultiTyped for mlua::Variadic<T> {
        type IsIntoMulti<Dst: FromLuaMultiTyped> = Dst::IsFromMulti<Self>;
        type IsIntoVariadic<DT: FromLuaTyped> = T::IsInto<DT>;

        type Head = Option<T>;
        type Tail = Self;
    }

    pub trait NestTuple {
        type NestedVar;
    }
    macro_rules! impl_tuple {
        ($TA:ident, $TB:ident) => {};
        ($Tail:ident $(,$T:ident)* $(,)?) => {
            impl_tuple!($($T),*);
            impl<$($T,)* $Tail> NestTuple for ($($T,)* $Tail,) {
                type NestedVar = nest_tuple_var!($($T,)* $Tail);
            }
            impl<$($T: IntoLuaTyped,)* $Tail: IntoLuaMultiTyped> IntoLuaMultiTyped for ($($T,)* $Tail,) {
                defer_into!(<Self as NestTuple>::NestedVar);
            }
            impl<$($T: FromLuaTyped,)* $Tail: FromLuaMultiTyped> FromLuaMultiTyped for ($($T,)* $Tail,) {
                type IsFromMulti<Src: IntoLuaMultiTyped> = Src::IsIntoMulti<<Self as NestTuple>::NestedVar>;
            }
        };
    }
    impl_tuple!(
        TP, TO, TN, TM, TL, TK, TJ, TI, TH, TG, TF, TE, TD, TC, TB, TA,
    );
}

pub trait LuaSub<Base: FromLuaTyped>: IntoLuaTyped<IsInto<Base> = True> {
    fn trans<Next: FromLuaTyped>(self) -> impl LuaSub<Next>
    where
        Base: LuaSub<Next>,
    {
        crate::lua::LuaCastIntoAny(self)
    }
}
impl<Sub: FromLuaTyped, T: IntoLuaTyped<IsInto<Sub> = True>> LuaSub<Sub> for T {}
pub trait LuaSubMulti<Sub: FromLuaMultiTyped>: IntoLuaMultiTyped<IsIntoMulti<Sub> = True> {}
impl<Sub: FromLuaMultiTyped, T: IntoLuaMultiTyped<IsIntoMulti<Sub> = True>> LuaSubMulti<Sub> for T {}
