extern crate cgmath;

use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};
use cgmath::{BaseNum, Point2, Vector2};


pub struct Typed<V, U>(pub V, PhantomData<U>);

impl<V: Clone, U> Clone for Typed<V, U> {
    fn clone(&self) -> Self {
        Typed(self.0.clone(), self.1)
    }
}

impl<V, U> Typed<V, U> {
    //Note: left for backwards compatibility,
    // should implement From<V> instead
    pub fn from_untyped(value: V) -> Self {
        Typed(value, PhantomData)
    }
    //Note: left for backwards compatibility,
    // can just take self.0 instead
    pub fn to_untyped(self) -> V {
        self.0
    }
}

macro_rules! op_binary {
    ($($name:ident::$fun:ident),*) => {
        $(
            impl<W, V: $name<W>, U> $name<Typed<W, U>> for Typed<V, U> {
                type Output = Typed<V::Output, U>;
                fn $fun(self, other: Typed<W, U>) -> Self::Output {
                    let v = self.0;
                    Typed(v.$fun(other.0), PhantomData)
                }
            }
        )*
    }
}

op_binary!(Add::add, Sub::sub, Mul::mul, Div::div);

macro_rules! define_type {
    ($name:ident = $inner:ident [$($field:ident),*]) => {
        pub type $name<T, U> = Typed<$inner<T>, U>;
        impl<T: BaseNum, U> $name<T, U> {
            pub fn new( $($field: T),* ) -> Self {
                Typed($inner::new( $($field),* ), PhantomData)
            }
            pub fn from_lengths( $($field: Length<T, U>),* ) -> Self {
                Self::new( $($field.0),* )
            }
        }
    }
}

pub type Length<T, U> = Typed<T, U>;
define_type!(TypedVector2 = Vector2 [x, y]);
define_type!(TypedPoint2 = Point2 [x, y]);


#[cfg(test)]
mod test {
    #[test]
    fn test_operations() {
        let pos = TypedPoint2::new(1i32, 2i32);
        let vec = TypedVector2::new(2i32, 2i32);
        let res = pos + vec;
    }
}
