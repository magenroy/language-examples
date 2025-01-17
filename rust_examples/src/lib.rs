#![allow(unused)]
#![feature(try_trait_v2)]
mod errors_and_try;
mod ownership;
mod mutation;
mod patterns;
mod implement;
mod control_flow;

mod not_yet_sorted {

    fn tst1<F: Fn(i32)>(f: F, x: i32) -> F {
        f(x);
        f
    }

    fn tst2(g: impl Fn(i32), x: i32) -> impl Fn(i32) {
        g(x);
        g
    }

    /// `FnOnce` is not a reference; if used `Fn` instead, would have lifetime issues
    fn tst3(h: impl FnOnce(i32)) -> impl FnOnce(i32) {
        |x| h(x)
    }

    fn tst4() -> Box<dyn(Fn(i32))> {
        Box::new(|x| {})
    }

    fn t(f: impl Fn(i32), g: impl Fn(i32), h: impl FnOnce(i32), x: i32) {
        tst1(f,x)(x);
        tst2(g,x)(x);
        tst3(h)(x);
        tst4()(x);
    }
}
