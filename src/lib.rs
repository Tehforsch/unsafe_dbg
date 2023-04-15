#![doc = include_str!("../README.md")]

use core::fmt::Debug;

pub fn unsafe_dbg_fmt<T, O: Debug>(t: T) -> String {
    let t: *const T = &t;
    let o = t as *const O;
    unsafe { format!("{:?}", &*o) }
}

#[macro_export]
macro_rules! unsafe_dbg {
    () => {
        std::eprintln!("[{}:{}]", std::file!(), std::line!())
    };
    (($val:expr, $t: ty) $(,)?) => {
        match $val {
            tmp => {
                std::eprintln!("[{}:{}] {} = {:#?}",
                    std::file!(), std::line!(), std::stringify!($val), $crate::unsafe_dbg_fmt::<_, &$t>(&tmp));
                tmp
            }
        }
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                std::eprintln!("[{}:{}] {} = {:#?}",
                    std::file!(), std::line!(), std::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($tt: tt),+ $(,)?) => {
        ($($crate::unsafe_dbg!($tt)),+,)
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::unsafe_dbg!($val)),+,)
    };
}
