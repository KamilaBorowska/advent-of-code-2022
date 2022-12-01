#[macro_export]
macro_rules! test {
    (__internal $ident:tt . $part:tt) => {};
    (
        __internal $ident:tt . $part:tt
        $( #[ $meta:meta ] )*
        input: $output:expr,
        $($tests:tt)*
    ) => {
        #[test] $( #[ $meta ] )* fn input() {
            assert_eq!(($ident.$part)(include_str!("input")).unwrap(), concat!($output));
        }
        super::test!(__internal $ident.$part $($tests)*);
    };
    (__internal $ident:tt . $part:tt) => {};
    (
        __internal
        $ident:tt . $part:tt
        $( #[ $meta:meta ] )*
        $name:ident: $input:expr => $output:expr,
        $($tests:tt)*
    ) => {
        #[test] $( #[ $meta ] )* fn $name() {
            assert_eq!(($ident.$part)($input).unwrap(), concat!($output));
        }
        super::test!(__internal $ident.$part $($tests)*);
    };
    (
        __internal $ident:tt . $part:tt
        $( #[ $meta:meta ] )*
        fn $name:ident() $b:block
        $($tests:tt)*
    ) => {
        #[test] $( #[ $meta ] )* fn $name() $b
        super::test!(__internal $ident.$part $($tests)*);
    };
    (
        $ident:tt . $part:tt,
        $($tests:tt)*
    ) => {
        mod $part {
            #[allow(unused_imports)]
            use crate::lines;
            use super::super::$ident;
            use super::*;
            super::test!(__internal $ident.$part $($tests)*);
        }
    };
}

#[macro_export]
macro_rules! lines {
    (__internal $out:tt) => {
        concat!$out
    };
    (__internal ($($out:tt)*) - $input:tt $($rest:tt)*) => {
        lines!(__internal ($($out)* concat!('-', $input), '\n',) $($rest)*)
    };
    (__internal ($($out:tt)*) $input:tt $($rest:tt)*) => {
        lines!(__internal ($($out)* $input, '\n',) $($rest)*)
    };
    ($($t:tt)*) => {
        lines!(__internal () $($t)*)
    };
}
