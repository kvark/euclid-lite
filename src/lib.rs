extern crate cgmath;

use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div};
use cgmath::{BaseNum, Point2, Vector2};


pub struct Typed<V, U>(pub V, PhantomData<U>);

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


impl<V: Clone, U> Clone for Typed<V, U> {
    fn clone(&self) -> Self {
        Typed(self.0.clone(), self.1)
    }
}

impl<V: fmt::Debug, U> fmt::Debug for Typed<V, U> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl<W, V: PartialEq<W>, U> PartialEq<Typed<W, U>> for Typed<V, U> {
    fn eq(&self, other: &Typed<W, U>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<V: Eq, U> Eq for Typed<V, U> {
    //empty
}

impl<W, V: PartialOrd<W>, U> PartialOrd<Typed<W, U>> for Typed<V, U> {
    fn partial_cmp(&self, other: &Typed<W, U>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<V: Ord, U> Ord for Typed<V, U> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}


macro_rules! op_binary {
    ($($name:ident::$fun:ident),*) => {
        $(
            impl<W, V: $name<W>, U> $name<Typed<W, U>> for Typed<V, U> {
                type Output = Typed<V::Output, U>;
                fn $fun(self, other: Typed<W, U>) -> Self::Output {
                    Typed((self.0).$fun(other.0), PhantomData)
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
    use super::*;

    #[test]
    fn test_operations() {
        let pos: TypedPoint2<i32, ()> = TypedPoint2::new(1, 2);
        let vec: TypedVector2<i32, ()> = TypedVector2::new(2, 2);
        assert_eq!(pos + vec, TypedPoint2::new(3, 4));
    }
}
