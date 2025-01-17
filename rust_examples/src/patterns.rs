// See REF: https://cheats.rs/#pattern-matching

struct Simple<T>(T);

fn a<T>(Simple(v) : Simple<T>) -> T { v }

// REF: https://rust-lang.github.io/rfcs/2005-match-ergonomics.html
// REF: https://doc.rust-lang.org/reference/patterns.html#reference-patterns
fn pattern_reference<T>(x: &Simple<T>) -> &T {
    let Simple(v) = x;
    v
}

fn pass_by_reference<T>(x: &Simple<T>) -> &T {
    let Simple(ref v) = *x;
    v
}

struct NotSimple<T> {
    first: T,
    second: [T;2],
    third: Option<T>,
}

enum MyEnum<T> {
    A, B, C(T), D(Simple<T>), E(NotSimple<T>)
}

use MyEnum::*;
fn matching(value: MyEnum<i32>, nullary: fn(), unary: fn(i32), borrows: fn(&i32)) {
    let y: i32;
    match value {
        A => nullary(),
        // ...
        B | C(_) => nullary(),
        D(Simple(x)) => nullary(),
        // ...
        E(NotSimple { third: Some(1 | 2), first: y@ 0..7, .. }) => unary(y),
        E(NotSimple { first: ref x@ 1..11, ..}) => borrows(x),
        _ => nullary(),
        // ...
    }
}
