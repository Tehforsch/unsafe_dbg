use unsafe_dbg::unsafe_dbg;

#[derive(Debug)]
struct A {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct B {
    x: String,
    y: bool,
}

#[derive(Debug)]
struct C {
    x: bool,
}

fn some_deeply_nested_generic_function<R, S, T>(r: R, s: &S, t: &mut T) {
    let (r_, s_, t_, x, (y, z)) =
        unsafe_dbg!((r, A), (s, &B), (t, &mut C), "a normal string", ("ok", 5));
    dbg!(x, y, z);
}

fn main() {
    let a = A { x: 10, y: 11 };
    let b = B {
        x: "hello world".into(),
        y: false,
    };
    let mut c = C { x: false };
    some_deeply_nested_generic_function(a, &b, &mut c)
}
