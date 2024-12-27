fn diagonal<T, U>(f: fn(T, T) -> U, x: T) -> U where T: Copy {
    f(x, x)
}

fn move_ownership<T, U>(f: fn(T) -> U, x: T) -> T where T: Copy {
    f(x);
    x
}

fn borrow<T, U>(f: fn(&T) -> U, x: T) -> T {
    f(&x);
    x
}
