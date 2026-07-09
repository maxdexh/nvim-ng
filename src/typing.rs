pub mod logic {
    pub trait Bool {
        type And<R: Bool>: Bool;
        type Or<R: Bool>: Bool;
    }
    pub struct True;
    pub struct False;
    impl Bool for True {
        type And<R: Bool> = R;
        type Or<R: Bool> = True;
    }
    impl Bool for False {
        type And<R: Bool> = False;
        type Or<R: Bool> = R;
    }
    pub type And<L, R> = <L as Bool>::And<R>;
    pub type Or<L, R> = <L as Bool>::Or<R>;
}
use logic::*;

pub trait LuaIsInto: IntoLuaTyped {
    type IsInto<Base: FromLuaTyped>: Bool;
}
impl<T: IntoLuaTyped> LuaIsInto for T {
    type IsInto<Base: FromLuaTyped> = Base::IsFrom<T>;
}
pub trait LuaSub<Base: FromLuaTyped>: LuaIsInto<IsInto<Base> = True> {}
impl<Base: FromLuaTyped, T: LuaIsInto<IsInto<Base> = True>> LuaSub<Base> for T {}

pub trait LuaIsIntoMulti: IntoLuaMultiTyped {
    type IsIntoMulti<Base: FromLuaMultiTyped>: Bool;
}
impl<T: IntoLuaMultiTyped> LuaIsIntoMulti for T {
    type IsIntoMulti<Base: FromLuaMultiTyped> = Base::IsFromMulti<T>;
}
pub trait LuaSubMulti<Base: FromLuaMultiTyped>: LuaIsIntoMulti<IsIntoMulti<Base> = True> {}
impl<Base: FromLuaMultiTyped, T: LuaIsIntoMulti<IsIntoMulti<Base> = True>> LuaSubMulti<Base> for T {}

// TODO: Try to use an associated FromLua instead of a bound
pub trait FromLuaTyped: mlua::FromLua {
    type IsFrom<Src: IntoLuaTyped>: Bool;
}
pub trait IntoLuaTyped: mlua::IntoLua {
    type IsNum: Bool;
    type IsInt: Bool;
    type IsStr: Bool;
    type IsFunc: Bool;
    type IsBool: Bool;
    type IsNil: Bool;

    type IsUnion<L: FromLuaTyped, R: FromLuaTyped>: Bool;

    type IsTableSeqConst<I: FromLuaTyped>: Bool;
    type IsTableSeqMut<I: IntoLuaTyped + FromLuaTyped>: Bool;
    type IsTableMapConst<K: FromLuaTyped, V: FromLuaTyped>: Bool;
    type IsTableMapMut<K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped>: Bool;
    type IsCallableWith<A: IntoLuaMultiTyped, R: FromLuaMultiTyped>: Bool;
    type IsStruct<Fields: IntoLuaMultiTyped + FromLuaMultiTyped>: Bool;
}
pub type IsInto<Src, Dst> = <Src as LuaIsInto>::IsInto<Dst>;
pub type IsEquiv<T, U> = And<IsInto<T, U>, IsInto<U, T>>;

mod into_impls {
    use super::*;

    macro_rules! add_bounds {
        ({
            $(type $item:ident$(<$($P:ident),* $(,)?>)? = $val:ty;)*
        }) => {$(
            add_bounds!{ @sel $item, $item, $val, [$($($P),*)?] }
        )*};
        (@sel IsNum, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsInt, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsStr, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsFunc, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsBool, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsNil, $name:ident, $val:ty, []) => {
            type $name = $val;
        };
        (@sel IsUnion, $name:ident, $val:ty, [$L:ident, $R:ident]) => {
            type $name<
                $L: crate::typing::FromLuaTyped,
                $R: crate::typing::FromLuaTyped,
            > = $val;
        };
        (@sel IsTableSeqConst, $name:ident, $val:ty, [$I:ident]) => {
            type $name<$I: crate::typing::FromLuaTyped> = $val;
        };
        (@sel IsTableSeqMut, $name:ident, $val:ty, [$I:ident]) => {
            type $name<$I: crate::typing::FromLuaTyped + crate::typing::IntoLuaTyped> = $val;
        };
        (@sel IsTableMapConst, $name:ident, $val:ty, [$K:ident, $V:ident]) => {
            type $name<
                $K: crate::typing::FromLuaTyped,
                $V: crate::typing::FromLuaTyped,
            > = $val;
        };
        (@sel IsTableMapMut, $name:ident, $val:ty, [$K:ident, $V:ident]) => {
            type $name<
                $K: crate::typing::IntoLuaTyped + crate::typing::FromLuaTyped,
                $V: crate::typing::IntoLuaTyped + crate::typing::FromLuaTyped,
            > = $val;
        };
        (@sel IsCallableWith, $name:ident, $val:ty, [$A:ident, $R:ident]) => {
            type $name<
                $A: crate::typing::IntoLuaMultiTyped,
                $R: crate::typing::FromLuaMultiTyped,
            > = $val;
        };
        (@sel IsStruct, $name:ident, $val:ty, [$Fs:ident]) => {
            type $name<$Fs: crate::typing::IntoLuaMultiTyped + crate::typing::FromLuaMultiTyped> = $val;
        };
        (@sel $item:ident, $name:ident, $val:ty, [$($t:tt)*]) => {
            compile_error! {
                concat!(
                    "Unknown item or incorrect number of params: ",
                    stringify!($item),
                )
            }
        };
        ($($input:tt)*) => {
            compile_error! {
                concat!(
                    "Malformed input:\n",
                    stringify!($($input)*),
                )
            }
        };
    }
    macro_rules! mk_defer_defaults {
        ($($name:ident$(<$($P:ident),* $(,)?>)?),* $(,)?) => {
            macro_rules! defer_defaults {
                $(($name, $postfix_handler:ident) => {
                    add_bounds! {{
                        type $name$(<$($P),*>)? = $postfix_handler!($name$(<$($P),*>)?);
                    }}
                };)*
                ($t:ident, $postfix_handler:ident) => {
                    compile_error! { concat!("defer_defaults: Unknown item ", stringify!($t)) }
                };
            }
        };
    }
    mk_defer_defaults![
        IsNum,
        IsInt,
        IsStr,
        IsFunc,
        IsBool,
        IsNil,
        IsUnion<_L, _R>,
        IsTableSeqConst<_I>,
        IsTableSeqMut<_I>,
        IsTableMapConst<_K, _V>,
        IsTableMapMut<_K, _V>,
        IsCallableWith<_A, _R>,
        IsStruct<_Fs>,
    ];
    macro_rules! mk_defaults_except {
        ($mac:ident, [$($item:ident),* $(,)?], $default_mac:ident) => {
            macro_rules! $mac {
                () => {
                    $mac!(@sel IsNum);
                    $mac!(@sel IsInt);
                    $mac!(@sel IsStr);
                    $mac!(@sel IsFunc);
                    $mac!(@sel IsBool);
                    $mac!(@sel IsNil);
                    $mac!(@sel IsUnion);
                    $mac!(@sel IsTableMapConst);
                    $mac!(@sel IsTableMapMut);
                    $mac!(@sel IsTableSeqConst);
                    $mac!(@sel IsTableSeqMut);
                    $mac!(@sel IsCallableWith);
                    $mac!(@sel IsStruct);
                };
                $( (@sel $item) => {}; )*
                (@sel $other:ident) => {
                    defer_defaults!($other, $default_mac);
                }
            }
        };
    }
    macro_rules! general_defaults {
        (IsUnion<$L:ident, $R:ident>) => {
            Or<IsInto<Self, $L>, IsInto<Self, $R>>
        };
        ($($postfix:tt)*) => {
            crate::typing::logic::False
        };
    }
    macro_rules! impl_into {
        ({
            $(#[params($($g:tt)*)])?
            impl $t:ty {}

            default!($default_mac:ident);

            $(
                type $item:ident$(<$($P:ident),* $(,)?>)? = $ity:ty;
            )*
        }) => {
            const _: () = {
                mk_defaults_except!(default_items, [$($item),*], $default_mac);
                impl$(<$($g)*>)? IntoLuaTyped for $t {
                    default_items!();
                    add_bounds!({
                        $(type $item$(<$($P),*>)? = $ity;)*
                    });
                }
            };
        };
    }
    impl_into!({
        impl bool {}

        default!(general_defaults);

        type IsBool = True;
    });
    impl_into!({
        impl crate::lua::LuaNil {}

        default!(general_defaults);

        type IsNil = True;
    });
    impl_into!({
        impl crate::lua::LuaVal {}

        default!(general_defaults);
    });
    impl_into!({
        impl crate::lua::LuaString {}

        default!(general_defaults);

        type IsStr = True;
    });
    const _: () = {
        macro_rules! default_defer_str {
            ($($postfix:tt)*) => {
                <crate::lua::LuaString as IntoLuaTyped>::$($postfix)*
            };
        }
        macro_rules! lua_strs {
            ($($t:ty),*) => {$(
                impl_into!({
                    impl $t {}

                    default!(default_defer_str);
                });
            )*};
        }
        lua_strs!(&str, String);
    };
    impl_into!({
        impl mlua::Function {}

        default!(general_defaults);

        type IsFunc = True;
    });
    impl_into!({
        impl f32 {}

        default!(general_defaults);

        type IsNum = True;
    });
    impl_into!({
        impl f64 {}

        default!(general_defaults);

        type IsNum = True;
    });
    macro_rules! ints {
        ($($int:ty),* $(,)?) => {$(
            impl_into!({
                impl $int {}

                default!(general_defaults);

                type IsNum = True;
                type IsInt = True;
            });
        )*};
    }
    ints![i8, u8, i16, u16, i32, u32, i64, u64, isize, usize];
    const _: () = {
        macro_rules! defer_union_T_nil {
            ($($postfix:tt)*) => {
                <crate::lua::LuaUnion<T, crate::lua::LuaNil> as IntoLuaTyped>::$($postfix)*
            };
        }
        impl_into!({
            #[params(T: IntoLuaTyped)]
            impl Option<T> {}

            default!(defer_union_T_nil);
        });
    };
    impl_into!({
        #[params(T: FromLuaTyped + IntoLuaTyped)]
        impl crate::lua::LuaSeq<T> {}

        default!(general_defaults);

        type IsTableSeqConst<U> = IsInto<T, U>;
        type IsTableMapConst<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
    });
    impl_into!({
        #[params(T: FromLuaTyped + IntoLuaTyped)]
        impl crate::lua::LuaSeqMut<T> {}

        default!(general_defaults);

        type IsTableSeqConst<U> = IsInto<T, U>;
        type IsTableSeqMut<U> = IsEquiv<T, U>;
        type IsTableMapConst<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type IsTableMapMut<K, V> = And<IsEquiv<crate::lua::LuaInt, K>, IsEquiv<T, V>>;
    });
    impl_into!({
        #[params(T: IntoLuaTyped)]
        impl crate::lua::LuaSeqOwned<T> {}

        default!(general_defaults);

        type IsTableSeqConst<U> = IsInto<T, U>;
        type IsTableSeqMut<U> = IsInto<T, U>;
        type IsTableMapConst<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
        type IsTableMapMut<K, V> = And<IsInto<crate::lua::LuaInt, K>, IsInto<T, V>>;
    });
    const _: () = {
        macro_rules! defer_into_table_T {
            ($($postfix:tt)*) => {
                <crate::lua::LuaSeqOwned<T> as IntoLuaTyped>::$($postfix)*
            };
        }
        impl_into!({
            #[params(T: IntoLuaTyped, const N: usize)]
            impl [T; N] {}

            default!(defer_into_table_T);
        });
    };
    impl_into!({
        #[params(K: IntoLuaTyped, V: IntoLuaTyped)]
        impl crate::lua::LuaMap<K, V> {}

        default!(general_defaults);

        type IsTableMapConst<DK, DV> = And<IsInto<K, DK>, IsInto<V, DV>>;
    });
    impl_into!({
        #[params(K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped)]
        impl crate::lua::LuaMapMut<K, V> {}

        default!(general_defaults);

        type IsTableMapConst<DK, DV> = And<IsInto<K, DK>, IsInto<V, DV>>;
        type IsTableMapMut<DK, DV> = And<IsEquiv<K, DK>, IsEquiv<V, DV>>;
    });
    impl_into!({
        #[params(K: IntoLuaTyped, V: IntoLuaTyped)]
        impl crate::lua::LuaMapOwned<K, V> {}

        default!(general_defaults);

        type IsTableMapConst<DK, DV> = And<IsInto<K, DK>, IsInto<V, DV>>;
        type IsTableMapMut<DK, DV> = And<IsInto<K, DK>, IsInto<V, DV>>;
    });
    impl_into!({
        #[params(A: FromLuaMultiTyped, R: IntoLuaMultiTyped)]
        impl crate::lua::LuaCallable<A, R> {}

        default!(general_defaults);

        type IsCallableWith<DA, DR> = And<IsIntoMulti<DA, A>, IsIntoMulti<R, DR>>;
    });

    const _: () = {
        macro_rules! default_true {
            ($($postfix:tt)*) => {
                True
            };
        }
        impl_into!({
            impl crate::lua::LuaBottom {}

            default!(default_true);
        });
        impl_into!({
            #[params(T: IntoLuaTyped)]
            impl crate::lua::LuaCastIntoAny<T> {}

            default!(default_true);
        });
    };
    const _: () = {
        macro_rules! defer_to_T {
            ($($postfix:tt)*) => {
                T::$($postfix)*
            };
        }
        impl_into!({
            #[params(T: IntoLuaTyped)]
            impl crate::lua::LuaDeferErr<T> {}

            default!(defer_to_T);
        });
        impl_into!({
            #[params(T: IntoLuaTyped, F: FnOnce(&mlua::Lua) -> mlua::Result<T>)]
            impl crate::lua::LuaDefer<F> {}

            default!(defer_to_T);
        });
    };
    const _: () = {
        macro_rules! defer_to_LR_and {
            ($($postfix:tt)*) => {
                And<L::$($postfix)*, R::$($postfix)*>
            };
        }
        impl_into!({
            #[params(L: IntoLuaTyped, R: IntoLuaTyped)]
            impl crate::lua::LuaUnion<L, R> {}

            default!(defer_to_LR_and);

            type IsUnion<A, B> =
                And<Or<IsInto<L, A>, IsInto<L, B>>, Or<IsInto<R, A>, IsInto<R, B>>>;
        });
    };
    impl_into!({
        #[params(S: crate::lua::LuaStructInner<Repr: mlua::IntoLua>)]
        impl crate::lua::LuaStruct<S> {}

        default!(general_defaults);

        type IsStruct<Fs> = And<IsIntoMulti<S::Fields, Fs>, IsIntoMulti<Fs, S::Fields>>;
    });
}

mod from_impls {
    use super::*;

    impl FromLuaTyped for crate::lua::LuaVal {
        type IsFrom<Src: IntoLuaTyped> = True;
    }
    impl FromLuaTyped for crate::lua::LuaString {
        type IsFrom<Src: IntoLuaTyped> = Src::IsStr;
    }
    impl FromLuaTyped for mlua::Function {
        type IsFrom<Src: IntoLuaTyped> = Src::IsFunc;
    }
    impl FromLuaTyped for bool {
        type IsFrom<Src: IntoLuaTyped> = Src::IsBool;
    }
    impl FromLuaTyped for crate::lua::LuaNil {
        type IsFrom<Src: IntoLuaTyped> = Src::IsNil;
    }
    impl FromLuaTyped for crate::lua::LuaNum {
        type IsFrom<Src: IntoLuaTyped> = Src::IsNum;
    }
    impl FromLuaTyped for crate::lua::LuaInt {
        type IsFrom<Src: IntoLuaTyped> = Src::IsInt;
    }
    impl<A: IntoLuaMultiTyped, R: FromLuaMultiTyped> FromLuaTyped for crate::lua::LuaCallable<A, R> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsCallableWith<A, R>;
    }
    impl<T: FromLuaTyped> FromLuaTyped for Option<T> {
        type IsFrom<Src: IntoLuaTyped> = IsInto<Src, crate::lua::LuaUnion<T, crate::lua::LuaNil>>;
    }
    impl FromLuaTyped for crate::lua::LuaBottom {
        type IsFrom<Src: IntoLuaTyped> = False;
    }
    impl<T: FromLuaTyped> FromLuaTyped for crate::lua::LuaSeq<T> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsTableSeqConst<T>;
    }
    impl<T: IntoLuaTyped + FromLuaTyped> FromLuaTyped for crate::lua::LuaSeqMut<T> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsTableSeqMut<T>;
    }
    impl<K: FromLuaTyped, V: FromLuaTyped> FromLuaTyped for crate::lua::LuaMap<K, V> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsTableMapConst<K, V>;
    }
    impl<K: IntoLuaTyped + FromLuaTyped, V: IntoLuaTyped + FromLuaTyped> FromLuaTyped
        for crate::lua::LuaMapMut<K, V>
    {
        type IsFrom<Src: IntoLuaTyped> = Src::IsTableMapMut<K, V>;
    }
    impl<L: FromLuaTyped, R: FromLuaTyped> FromLuaTyped for crate::lua::LuaUnion<L, R> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsUnion<L, R>;
    }
    impl<S: crate::lua::LuaStructInner<Repr: mlua::FromLua>> FromLuaTyped for crate::lua::LuaStruct<S> {
        type IsFrom<Src: IntoLuaTyped> = Src::IsStruct<S::Fields>;
    }
}

pub trait IntoLuaMultiTyped: mlua::IntoLuaMulti {
    type IsIntoVariadic<T: FromLuaTyped>: Bool;

    type Head: IntoLuaTyped;
    type Tail: IntoLuaMultiTyped;
}
pub trait FromLuaMultiTyped: mlua::FromLuaMulti {
    type IsFromMulti<Src: IntoLuaMultiTyped>: Bool;
}
pub type IsIntoMulti<Src, Dst> = <Src as LuaIsIntoMulti>::IsIntoMulti<Dst>;
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
            type IsIntoVariadic<_T: FromLuaTyped> = <$t as IntoLuaMultiTyped>::IsIntoVariadic<_T>;

            type Head = <$t as IntoLuaMultiTyped>::Head;
            type Tail = <$t as IntoLuaMultiTyped>::Tail;
        };
    }

    #[diagnostic::do_not_recommend]
    impl<T: IntoLuaTyped> IntoLuaMultiTyped for T {
        type IsIntoVariadic<DT: FromLuaTyped> = IsInto<T, DT>;
        type Head = T;
        type Tail = ();
    }
    impl IntoLuaMultiTyped for () {
        type IsIntoVariadic<T: FromLuaTyped> = True;

        type Head = crate::lua::LuaNil;
        type Tail = Self;
    }
    impl<Head: IntoLuaTyped, Tail: IntoLuaMultiTyped> IntoLuaMultiTyped for (Head, Tail) {
        type IsIntoVariadic<T: FromLuaTyped> = And<IsInto<Head, T>, Tail::IsIntoVariadic<T>>;

        type Head = Head;
        type Tail = Tail;
    }
    impl<Tail: IntoLuaMultiTyped> IntoLuaMultiTyped for (Tail,) {
        defer_into!(Tail);
    }
    impl<T: IntoLuaTyped> IntoLuaMultiTyped for mlua::Variadic<T> {
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
                type IsFromMulti<Src: IntoLuaMultiTyped> = IsIntoMulti<Src, <Self as NestTuple>::NestedVar>;
            }
        };
    }
    impl_tuple!(
        TP, TO, TN, TM, TL, TK, TJ, TI, TH, TG, TF, TE, TD, TC, TB, TA,
    );
}
