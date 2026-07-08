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

pub trait LuaSub<Base: FromLuaTyped>: IntoLuaTyped<IsInto<Base> = True> {}
impl<Sub: FromLuaTyped, T: IntoLuaTyped<IsInto<Sub> = True>> LuaSub<Sub> for T {}

pub trait LuaSubMulti<Sub: FromLuaMultiTyped>: IntoLuaMultiTyped<IsIntoMulti<Sub> = True> {}
impl<Sub: FromLuaMultiTyped, T: IntoLuaMultiTyped<IsIntoMulti<Sub> = True>> LuaSubMulti<Sub> for T {}

pub trait FromLuaTyped: mlua::FromLua {
    type IsFrom<Src: IntoLuaTyped>: Bool;
}
pub trait IntoLuaTyped: mlua::IntoLua {
    type IsInto<Dst: FromLuaTyped>: Bool;

    type MinusNil: IntoLuaTyped;

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

mod into_impls {
    use super::*;

    macro_rules! add_bounds {
        ({
            $(type $item:ident$(<$($P:ident),* $(,)?>)? = $val:ty;)*
        }) => {$(
            add_bounds!{ @sel $item, $item, $val, [$($($P),*)?] }
        )*};
        (@sel IsInto, $name:ident, $val:ty, [$Dst:ident]) => {
            type $name<$Dst: FromLuaTyped> = $val;
        };
        (@sel IsIntoTable, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsIntoNum, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsIntoInt, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsIntoStr, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsIntoFunc, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsIntoBool, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsIntoTableSeqConst, $name:ident, $val:ty, [$I:ident]) => {
            type $name<$I: crate::typing::FromLuaTyped> = $val;
        };
        (@sel IsIntoTableSeqMut, $name:ident, $val:ty, [$I:ident]) => {
            type $name<$I: crate::typing::FromLuaTyped + crate::typing::IntoLuaTyped> = $val;
        };
        (@sel IsIntoTableMapConst, $name:ident, $val:ty, [$K:ident, $V:ident]) => {
            type $name<
                $K: crate::typing::FromLuaTyped,
                $V: crate::typing::FromLuaTyped,
            > = $val;
        };
        (@sel IsIntoTableMapMut, $name:ident, $val:ty, [$K:ident, $V:ident]) => {
            type $name<
                $K: crate::typing::IntoLuaTyped + crate::typing::FromLuaTyped,
                $V: crate::typing::IntoLuaTyped + crate::typing::FromLuaTyped,
            > = $val;
        };
        (@sel IsIntoCallWith, $name:ident, $val:ty, [$A:ident, $R:ident]) => {
            type $name<
                $A: crate::typing::IntoLuaMultiTyped,
                $R: crate::typing::FromLuaMultiTyped,
            > = $val;
        };
        (@sel MinusNil, $name:ident, $val:ty, []) => {
            type $name = $val;
        }
    }
    macro_rules! default_item {
        (IsInto) => {
            add_bounds!({
                type IsInto<Dst> = Dst::IsFrom<Self>;
            });
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
            add_bounds!({
                type IsIntoTableSeqConst<_I> = crate::typing::False;
            });
        };
        (IsIntoTableSeqMut) => {
            add_bounds!({
                type IsIntoTableSeqMut<_I> = crate::typing::False;
            });
        };
        (IsIntoTableMapConst) => {
            add_bounds!({
                type IsIntoTableMapConst<_K, _V> = crate::typing::False;
            });
        };
        (IsIntoTableMapMut) => {
            add_bounds!({
                type IsIntoTableMapMut<_K, _V> = crate::typing::False;
            });
        };
        (IsIntoCallWith) => {
            add_bounds!({
                type IsIntoCallWith<_A, _R> = crate::typing::False;
            });
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
                };
                $( (@sel $item) => {}; )*
                (@sel $other:ident) => {
                    default_item!($other);
                }
            }
        };
    }
    macro_rules! impl_into {
        ({
            $(#[params($($g:tt)*)])?
            impl $t:ty {}
            $(
                type $item:ident$(<$($P:ident),* $(,)?>)? = $ity:ty;
            )*
        }) => {
            const _: () = {
                mk_defaults_except!(default_items, [$($item),*]);
                impl$(<$($g)*>)? IntoLuaTyped for $t {
                    default_items!();
                    add_bounds!({
                        $(type $item$(<$($P),*>)? = $ity;)*
                    });
                }
            };
        };
    }
    #[rustfmt::skip]
    macro_rules! defer_by {
        ($mac:ident) => { add_bounds! {{
            type IsInto<_Dst> = $mac!(IsInto<_Dst>);
            type IsIntoTable = $mac!(IsIntoTable);
            type IsIntoNum = $mac!(IsIntoNum);
            type IsIntoInt = $mac!(IsIntoInt);
            type IsIntoStr = $mac!(IsIntoStr);
            type IsIntoFunc = $mac!(IsIntoFunc);
            type IsIntoBool = $mac!(IsIntoBool);
            type IsIntoTableSeqConst<_I> = $mac!(IsIntoTableSeqConst<_I>);
            type IsIntoTableSeqMut<_I> = $mac!(IsIntoTableSeqMut<_I>);
            type IsIntoTableMapConst<_K, _V> = $mac!(IsIntoTableMapConst<_K, _V>);
            type IsIntoTableMapMut<_K, _V> = $mac!(IsIntoTableMapMut<_K, _V>);
            type IsIntoCallWith<_A, _R> = $mac!(IsIntoCallWith<_A, _R>);
        }} };
    }

    impl_into!({
        impl bool {}
        type IsIntoBool = True;
        type MinusNil = Self;
    });
    impl_into!({
        impl crate::lua::LuaVal {}
        // NOTE: This is overly restrictive and can be relaxed by adding a LuaNonNil type.
        type MinusNil = Self;
    });
    impl_into!({
        impl crate::lua::LuaString {}
        type IsIntoStr = True;
        type MinusNil = Self;
    });
    const _: () = {
        macro_rules! defer_to_str {
            ($($postfix:tt)*) => {
                <crate::lua::LuaString as IntoLuaTyped>::$($postfix)*
            };
        }
        macro_rules! lua_strs {
            ($($t:ty),*) => {$(
                impl IntoLuaTyped for $t {
                    defer_by!(defer_to_str);
                    type MinusNil = Self;
                }
            )*};
        }
        lua_strs!(&str, String);
    };
    impl_into!({
        impl crate::lua::LuaTableAny {}
        type IsIntoTable = True;
        type MinusNil = Self;
    });
    impl_into!({
        impl crate::lua::LuaFuncAny {}
        type IsIntoFunc = True;
        type MinusNil = Self;
    });
    impl_into!({
        impl f32 {}
        type IsIntoNum = True;
        type MinusNil = Self;
    });
    impl_into!({
        impl f64 {}
        type IsIntoNum = True;
        type MinusNil = Self;
    });
    macro_rules! ints {
        ($($int:ty),* $(,)?) => {$(
            impl_into!({
                impl $int {}
                type IsIntoNum = True;
                type IsIntoInt = True;
                type MinusNil = Self;
            });
        )*};
    }
    ints![i8, u8, i16, u16, i32, u32, i64, u64, isize, usize];
    impl_into!({
        #[params(T: IntoLuaTyped, const N: usize)]
        impl [T; N] {}

        type IsIntoTable = True;
        type IsIntoTableSeqConst<U> = IsInto<T, U>;
        type IsIntoTableSeqMut<U> = Self::IsIntoTableSeqConst<U>;
        type IsIntoTableMapConst<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type IsIntoTableMapMut<K, V> = Self::IsIntoTableMapConst<K, V>;
        type MinusNil = Self;
    });
    impl_into!({
        #[params(T: IntoLuaTyped)]
        impl Option<T> {}
        type MinusNil = T::MinusNil;
    });
    impl_into!({
        #[params(T: FromLuaTyped + IntoLuaTyped)]
        impl crate::lua::LuaTableSeq<T> {}
        type IsIntoTable = True;
        type IsIntoTableSeqConst<U> = IsInto<T, U>;
        type IsIntoTableMapConst<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type MinusNil = Self;
    });
    impl_into!({
        #[params(T: FromLuaTyped + IntoLuaTyped)]
        impl crate::lua::LuaTableSeqMut<T> {}
        type IsIntoTable = True;
        type IsIntoTableSeqConst<U> = IsInto<T, U>;
        type IsIntoTableSeqMut<U> = IsEquiv<T, U>;
        type IsIntoTableMapConst<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type IsIntoTableMapMut<K, V> = And<IsEquiv<crate::lua::LuaInt, K>, IsEquiv<T, V>>;
        type MinusNil = Self;
    });
    impl_into!({
        #[params(T: IntoLuaTyped)]
        impl crate::lua::LuaTableSeqOwned<T> {}
        type IsIntoTable = True;
        type IsIntoTableSeqConst<U> = IsInto<T, U>;
        type IsIntoTableSeqMut<U> = IsInto<T, U>;
        type IsIntoTableMapConst<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type IsIntoTableMapMut<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type MinusNil = Self;
    });
    impl_into!({
        #[params(K: IntoLuaTyped, V: IntoLuaTyped)]
        impl crate::lua::LuaTableMap<K, V> {}
        type IsIntoTable = True;
        type IsIntoTableMapConst<DK, DV> = And<IsInto<K, DK>, IsInto<V, DV>>;
        type MinusNil = Self;
    });
    impl_into!({
        #[params(K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped)]
        impl crate::lua::LuaTableMapMut<K, V> {}
        type IsIntoTable = True;
        type IsIntoTableMapConst<DK, DV> = And<IsInto<K, DK>, IsInto<V, DV>>;
        type IsIntoTableMapMut<DK, DV> = And<IsEquiv<K, DK>, IsEquiv<V, DV>>;
        type MinusNil = Self;
    });
    impl_into!({
        #[params(K: IntoLuaTyped, V: IntoLuaTyped)]
        impl crate::lua::LuaTableMapOwned<K, V> {}
        type IsIntoTable = True;
        type IsIntoTableMapConst<DK, DV> = And<IsInto<K, DK>, IsInto<V, DV>>;
        type IsIntoTableMapMut<DK, DV> = And<IsInto<K, DK>, IsInto<V, DV>>;
        type MinusNil = Self;
    });
    impl_into!({
        #[params(A: FromLuaMultiTyped, R: IntoLuaMultiTyped)]
        impl crate::lua::LuaCallable<A, R> {}
        type IsIntoCallWith<DA, DR> = And<IsIntoMulti<DA, A>, IsIntoMulti<R, DR>>;
        type MinusNil = Self;
    });
    const _: () = {
        macro_rules! defer_to_true {
            ($($postfix:tt)*) => {
                True
            };
        }
        impl IntoLuaTyped for crate::lua::LuaBottom {
            type MinusNil = Self;
            defer_by!(defer_to_true);
        }
        impl<T: mlua::IntoLua> IntoLuaTyped for crate::lua::LuaCastIntoAny<T> {
            type MinusNil = Self;
            defer_by!(defer_to_true);
        }
    };
    const _: () = {
        macro_rules! defer_to_T {
        ($($postfix:tt)*) => {
            T::$($postfix)*
        };
    }
        impl<T: IntoLuaTyped> IntoLuaTyped for crate::lua::LuaDeferErr<T> {
            type MinusNil = T::MinusNil;
            defer_by!(defer_to_T);
        }
        impl<T: IntoLuaTyped, F: FnOnce(&mlua::Lua) -> mlua::Result<T>> IntoLuaTyped
            for crate::lua::LuaDefer<F>
        {
            type MinusNil = T::MinusNil;
            defer_by!(defer_to_T);
        }
    };
    const _: () = {
        macro_rules! defer_to_LR_and {
        ($($postfix:tt)*) => {
            And<L::$($postfix)*, R::$($postfix)*>
        };
    }
        impl<L: IntoLuaTyped, R: IntoLuaTyped> IntoLuaTyped for crate::lua::LuaUnion<L, R> {
            type MinusNil = crate::lua::LuaUnion<L::MinusNil, R::MinusNil>;
            defer_by!(defer_to_LR_and);
        }
    };
}

mod from_impls {
    use super::*;

    impl FromLuaTyped for crate::lua::LuaVal {
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
        type IsFrom<Src: IntoLuaTyped> = IsInto<Src::MinusNil, T>;
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
    impl<L: FromLuaTyped, R: FromLuaTyped> FromLuaTyped for crate::lua::LuaUnion<L, R> {
        // FIXME: This is too restrictive; in particular, L | R is not a subtype of L | R
        type IsFrom<Src: IntoLuaTyped> = Or<IsInto<Src, L>, IsInto<Src, R>>;
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
        type IsFromMulti<Src: IntoLuaMultiTyped> = IsInto<Src::Head, Self>;
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
        type IsIntoVariadic<DT: FromLuaTyped> = IsInto<T, DT>;
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
        type IsIntoVariadic<T: FromLuaTyped> = And<IsInto<Head, T>, Tail::IsIntoVariadic<T>>;

        type Head = Head;
        type Tail = Tail;
    }
    impl<Tail: IntoLuaMultiTyped> IntoLuaMultiTyped for (Tail,) {
        defer_into!(Tail);
    }
    impl<T: IntoLuaTyped> IntoLuaMultiTyped for mlua::Variadic<T> {
        type IsIntoMulti<Dst: FromLuaMultiTyped> = Dst::IsFromMulti<Self>;
        type IsIntoVariadic<DT: FromLuaTyped> = IsInto<T, DT>;

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
