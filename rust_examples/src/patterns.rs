struct Simple<T>(T);

fn a<T>(Simple(v) : Simple<T>) -> T { v }

// REF: https://rust-lang.github.io/rfcs/2005-match-ergonomics.html
fn pattern_reference<T>(x: &Simple<T>) -> &T {
    let Simple(v) = x;
    v
}

fn pass_by_reference<T>(x: &Simple<T>) -> &T {
    let Simple(ref v) = *x;
    v
}
