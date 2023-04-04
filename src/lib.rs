use std::ops::{Neg,Mul,Add};
use num_traits::Float;
mod primitives;
mod ops;
mod floats;

pub use crate::primitives::Im;



#[derive(Debug,PartialEq,Eq)]
pub struct C<T: Copy + PartialEq>(pub T, pub T);

impl <T: Copy + PartialEq> C<T> {
   pub fn re(&self)->T{
    self.0
   } 
   pub fn im(&self)->T{
    self.1
   }
}

impl <T: Copy + PartialEq + Neg<Output = T>> C<T>{
   pub fn conj(&self)->C<T>{
        C(self.0, -self.1)
   }
}

impl<T: Copy + PartialEq + Add<Output = T> + Mul<Output = T>> C<T> {
    pub fn r_square(&self) -> T {
        self.0 * self.0 + self.1 * self.1
    }
}

impl <T: Float> C<T>{
   pub fn from_polar(r: T, angle: T)->C<T>{
        let unit = C(T::from(0).unwrap(), angle).exp();
        C(r * unit.0, r * unit.1)
   }
}

#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn test_add() {
    assert_eq!( 1 + 1.im(), C(1,1));
    assert_eq!( 1.im() + 1, C(1,1));
    assert_eq!( C(0., 2.) + C(2., 3.), C(2.,5.));
  }

  #[test]
  fn test_sub() {
    assert_eq!( 1 - 1.im(), C(1,-1));
    assert_eq!( 1.im() - 1, C(-1,1));
    assert_eq!( C(0., 2.) - C(2., 3.), C(-2.,-1.));
  }

}