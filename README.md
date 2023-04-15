# unsafe_dbg

## Motivation
Sometimes, when you're debugging or testing generic rust functions, you end up wanting to know the value of one of your variables `t` which is of some generic type `T`. You want to just write `dbg!(t)`, check the output and get on with your life. However, `T` doesn't implement `Debug`. Of course, you could just add a `T: Debug` constraint on your function, which can be a decent solution sometimes. However, if your function is really deep in the callchain of your code, you end up having to add `T: Debug` constraints everywhere, while also having to `#[derive(Debug)]` for all types that you enter the callchain with. Even after having printed the value, the ordeal is not over. If you decide to leave the constraints around, this means that there are now `T: Debug` constraints remaining everywhere in the code, that really don't need to be there. However, if you remove them, you already know that you will have to debug print another `T` not too far in the future.

This macro is for those scenarios where you know that currently, when you're running your code, `T` is really just your favorite struct `A`. `A` implements `Debug` and is just generally really friendly and nice to be around and you really just want to see what it has to say.

## Usage 
```rust
use unsafe_dbg::unsafe_dbg;

#[derive(Debug)]
struct A {
    x: i32,
    y: String,
}

fn deeply_nested_generic_function<T>(t: T) {
    unsafe_dbg!((t, A));
}

let a = A { x: 10, y: "hello world".into() };
deeply_nested_generic_function(a)
```

## Safety
This macro is horribly unsafe and should only ever be used to briefly debug something. It simply reinterprets some object in whatever you type you give it and debug prints it. There are absolutely no safety handles here. During the ~20 times I tested this macro while writing it, I got complete gibberish about 10 times because I accidentally interpreted some `&T` as `T`, some `T` as `&T` or something else. If you're lucky, the gibberish will be obvious and you can give the macro the correct type. If you're not you might be horribly mislead.

Here is another thing that wont't work (a `dyn T` is nothing like a `T` after all):

```rust should_panic

use unsafe_dbg::unsafe_dbg;

trait MyTrait {}

impl MyTrait for String {}

fn deeply_nested_dynamic_dispatch_function(t: &dyn MyTrait) {
    unsafe_dbg!((t, String));
    panic!("This absolutely doesn't work!");
}

deeply_nested_dynamic_dispatch_function(&"hi".to_owned());
```

## Behavior
The macro behaves like the normal debug macro, by returning the value
given to it. Importantly, the returned type is the generic type `T`,
not the type into which we casted horribly unsafely, because if it
did, it wouldn't take long for someone to get the idea to use this
casted result for something further down the line, which is a lot
worse than just having unsafe debug prints.

```rust
# use unsafe_dbg::unsafe_dbg;

fn deeply_nested_generic_function<T: Clone + PartialEq + Eq>(t: T) {
    let my_t = unsafe_dbg!((t.clone(), String));
    // Intentionally not using assert_eq here, because that
    // requires T to implement Debug, which defeats the whole purpose :)
    assert!(t == my_t);
}

deeply_nested_generic_function::<String>("hi".into());
```

The macro supports multiple arguments, each of which can be casted horribly unsafely or debug printed normally, as in the normal `dbg!` macro:


```rust
use unsafe_dbg::unsafe_dbg;

fn deeply_nested_generic_function<T: Clone + PartialEq + Eq>(t: T) {
    let (my_t, my_string, my_bool) = unsafe_dbg!((t.clone(), String), "hello world", true);
    // Intentionally not using assert_eq here, because that
    // requires T to implement Debug, which defeats the whole purpose.
    assert!(t == my_t);
    assert_eq!(my_string, "hello world");
    assert_eq!(my_bool, true);
}

deeply_nested_generic_function::<usize>(1000);
```
