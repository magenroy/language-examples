// if we want our trait to apply to unsized objects, we can never take or return the Self type directly
trait ForUnsized {

    fn inspect(&self);

    fn mutate(&mut self);

    fn fancy_ref() -> Box<Self>;

    fn reference<'a, 'b>(&'a self) -> &'b Self where 'a : 'b {
        self
    }

}

// Need `Sized` if want to have functions that have `Self` as the type of a parameter or return value
trait Trait: Sized {
    type AssociatedType;

    fn borrowing_method<T>(&self) -> T;

    // `self` can be any type that dereferences to `Self`
    fn fancy_deref<T>(self: Box<Self>) -> Self;

    fn mutating_method(&mut self);

    fn owning_method<T>(self) -> T;

    fn associated_function<T>(x: T) -> Self;

    fn other_function<T>(x: T) -> T;

    fn and_another<T>(self, x: T) -> Self::AssociatedType;

    fn with_default_implementation<T>(mut self) -> Self::AssociatedType {
        self.mutating_method();
        let x: Self::AssociatedType = self.borrowing_method();
        let x = Self::other_function(x);

        Self::and_another(self, x)
    }

}

trait SubTrait: Trait {} 

// TODO: explain how to make consideration of using generic types or associated types for traits
// See https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types
trait WithGeneric<'a, T=i32, D=T> where D: ?Sized {

    fn f() -> D;

    fn g(x: &'a T) -> Self;
}

impl<'a, T> WithGeneric<'a, T> for &'a T where T: Default {

    fn f() -> T { Default::default() }

    fn g(x: &'a T) -> Self { x }

}

impl WithGeneric<'static> for () {
    fn f() -> i32 { 0 }
    fn g(x: &'static i32) {}
}

#[derive(Debug, PartialEq, Eq)]
enum MyEnum<U> { Nullary, Unary(U), Binary(U,U) }

impl<U> Trait for MyEnum<U> {
    type AssociatedType = U;

    fn borrowing_method<T>(&self) -> T {
        panic!()
    }

    fn fancy_deref<T>(self: Box<Self>) -> Self {
        *self
    }

    fn mutating_method(&mut self) {
        // std::mem::replace(self, MyEnum::Nullary);
        *self = MyEnum::Nullary;
    }

    fn owning_method<T>(self) -> T {
        panic!()
    }

    fn associated_function<T>(_: T) -> Self {
        MyEnum::Nullary
    }

    fn other_function<T>(x: T) -> T {
        x
    }

    fn and_another<T>(self, x: T) -> Self::AssociatedType {
        panic!()
    }

}

#[test]
fn tst() {
    let mut a = MyEnum::Unary(1);
    a.mutating_method();
    assert_eq!(a, MyEnum::Nullary);
}

// impl SubTrait for MyEnum<[u32]> {}
// cannot implement SubTrait for MyEnum<[u32]> since [u32] is unsized (does not have Sized trait)

// Seems like enums aren't allowed to use unsized types?
// But structs can
struct Unsized<T> where T: ?Sized {
    x: T
}

impl ForUnsized for Unsized<[u32]> {
    fn inspect(&self) {}

    fn mutate(&mut self) {
        let Unsized { x } = self;
        x[0] = 0;
    }

    fn fancy_ref() -> Box<Self> {
        Box::new(Unsized { x: [0,1,2,3,4] })
    }
}

impl<const N: usize> SubTrait for MyEnum<[u32; N]> {}

use std::ops::Add;
impl<U> MyEnum<U> {

    fn sum(self) -> i32 where U: Add<Output = i32>  + Default {
        match self {
            Self::Nullary => U::default() + U::default(),
            Self::Unary(x) => x + Default::default(), // same as `x + U::default()`
            Self::Binary(x,y) => x + y,
        }
    }

    fn f() -> <Self as Trait>::AssociatedType where U: Default {
        Default::default()
    }
}
