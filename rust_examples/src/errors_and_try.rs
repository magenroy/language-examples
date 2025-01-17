
// Currently the `Try` trait is part of the nightly feature `try_trait_v2`
//#![feature(try_trait_v2)]
use std::ops::{ControlFlow, Try};

fn f(x: impl Try) {}
