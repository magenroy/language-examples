// Rust has an ownership system
//
// by default, access to variables is transient: when a variable gets used directly, it cannot be
// used afterwards
//
// This is thought of as "moving ownership" of the value of the variable; once the variable no
// longer owns its value, it cannot be used anymore.
//
// In order for a variable to be able to behave like it has "copy semantics", it must implement the
// `Copy` trait. This means that we can automatically copy it
//
// NOTE that instead of moving ownership of a variable's value, we can instead borrow it by using a
// _reference_ to the value. Given a variable `x`, the syntax `&x` is a _reference_ to "x" that "borrows" the value of "x".



fn diagonal<T>(x: T) -> (T, T) where T: Copy {
    (x, x)
}

fn move_ownership<T, U>(f: fn(T) -> U, x: T) -> T where T: Copy {
    f(x);
    x
}

fn borrow<T, U>(f: fn(&T) -> U, x: T) -> T {
    f(&x);
    x
}
