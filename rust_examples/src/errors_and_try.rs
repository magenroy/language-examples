fn tuple_option<A,B>(x: Option<A>, y: Option<B>) -> Option<(A,B)> {
    Some((x?, y?))
}

fn tuple_result<A,B,E>(first: Result<A,E>, second: Result<B,E>) -> Result<(A,B), E> {
    Ok((first?, second?))
}

#[test]
fn tuple() {
    let first: Result<char, u8> = Err(1);
    let second: Result<bool, u8> = Err(2);
    assert_eq!(Err(1), tuple_result(first, second));
    assert_eq!(Err(1), tuple_result(first, Ok(true)));
    assert_eq!(Err(2), tuple_result(Ok('a'), second));

    let ok: Result<(char, bool), u8> = Ok(('a', true));
    assert_eq!(ok, tuple_result(Ok('a'), Ok(true)));
}

use std::ops::ControlFlow;

// todo tuple_flow

// Currently the `Try` trait is part of the nightly feature `try_trait_v2`
//#![feature(try_trait_v2)]
use std::ops::Try;

fn f(x: impl Try) {}
