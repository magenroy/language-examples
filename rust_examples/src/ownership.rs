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

// TODO: not only variables can own values/memory
// I think that, for example, the fact that functions can own stuff helps explain why we don't just
// pass everything as references.
// I guess the other reason is that we want to pass ownership when we know we won't need the
// variable anymore, so the memory can be freed -- it's a way to keep track of how long we need to
// keep the memory allocated


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
fn no_own<T>(f: fn(&T), x: &T) -> &T {
    f(x);
    x
}

// TODO: Deref coercion!

#[cfg(test)]
mod test {

    fn mutate<T>(mut x: T, y: T) {
        x = y;
    }

    fn other_mutate<T>(x: T, y: T) {
        let mut z = x;
        z = y;
    }

    // see https://quinedot.github.io/rust-learning/st-reborrow.html
    fn mutate_forreal<T>(x: &mut T, y: T) {
        *x = y;
    }

    #[test]
    fn mutate_tst() {
        {
            let mut a = "a";
            mutate(a,"b");
            assert_ne!(a, "b");
        }

        {
            let mut a = String::from("a");
            mutate(a,String::from("b"));
            // `a` gets changed, but also no longer exists in this scope because ownership was
            // moved into `mutate`
        }

        {
            let mut a = String::from("a");
            mutate_forreal(&mut a, String::from("b"));
            assert_eq!(a, "b");
        }
    }
}
