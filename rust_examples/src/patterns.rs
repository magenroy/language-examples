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

enum MyEnum<T> {
    A, B(T), C(Simple<T>)
}
