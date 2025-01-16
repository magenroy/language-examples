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

enum MyEnum<U> { Nullary, Unary(U), Binary(U,U) }

impl<U> Trait for MyEnum<U> {
    type AssociatedType = U;

    fn borrowing_method<T>(&self) -> T {
        panic!()
    }

    fn mutating_method(&mut self) {
        std::mem::replace(self, MyEnum::Nullary);
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
