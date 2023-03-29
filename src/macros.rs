macro_rules! enum_type {
    { $t:ident $(: $st:path)? { $($tblock:item)* } $($s:ident { $($sblock:item)* })+ } => {
        pub trait $t $(: $st)? {
            fn name() -> &'static str;
            $($tblock)*
        }
        $(pub struct $s {}
        // impl $s {
        // }
        impl $t for $s {
            fn name() -> &'static str { stringify!($s) }
            $($sblock)*
        }
        impl std::fmt::Debug for $s {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", Self::name())
            }
        }
        impl std::fmt::Display for $s {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", Self::name())
            }
        }
        )*
    };
}
pub(crate) use enum_type;

// enum_type! {
//     Style {
//         fn code() -> char;
//     }
//     Call {
//         fn code() -> char { 'c' }
//     }
//     Put {
//         fn code() -> char { 'p' }
//     }
// }


// macro_rules! enum_type {
//     ($t:ident [$($s:ident)+] {$($b:tt)*}) => {
//         pub trait $t: std::fmt::Debug + std::fmt::Display {
//             fn name() -> &'static str;
//             $($b)*
//         }
//         $(pub struct $s {}
//         impl $s {
//             pub fn name() -> &'static str {
//                 stringify!($s)
//             }
//         }
//         impl std::fmt::Debug for $s {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 write!(f, "{}", Self::name())
//             }
//         }
//         impl std::fmt::Display for $s {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 write!(f, "{}", Self::name())
//             }
//         }
//         )*
//     };
// }

