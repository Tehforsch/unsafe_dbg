#![doc = include_str!("../README.md")]

use core::fmt::Debug;
use std::any::Any;

pub fn safe_dbg_fmt<T: Any, O: 'static + Debug>(t: &T) -> String {
    let o: &O = (t as &dyn Any)
        .downcast_ref()
        .unwrap_or_else(|| panic!("Wrong type passed to safe_dbg!"));
    format!("{:?}", o)
}

pub fn unsafe_dbg_fmt<T, O: Debug>(t: &T) -> String {
    let t: *const T = t;
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
                    std::file!(), std::line!(), std::stringify!($val), $crate::unsafe_dbg_fmt::<_, $t>(&tmp));
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

#[macro_export]
macro_rules! safe_dbg {
    () => {
        std::eprintln!("[{}:{}]", std::file!(), std::line!())
    };
    (($val:expr, $t: ty) $(,)?) => {
        match $val {
            tmp => {
                std::eprintln!("[{}:{}] {} = {:#?}",
                    std::file!(), std::line!(), std::stringify!($val), $crate::safe_dbg_fmt::<_, $t>(&tmp));
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
        ($($crate::safe_dbg!($tt)),+,)
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::safe_dbg!($val)),+,)
    };
}
