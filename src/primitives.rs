use super::*;

pub trait Im<T: Copy + PartialEq>{
    fn im(&self)->C<T>;
}
macro_rules!  prim_to_im{
    ($T:tt) => {
        impl Im<$T> for $T{
            fn im(&self)->C<$T> {
                C(0_u8 as $T, *self)
            }
        }
    };
}

prim_to_im!(u8);
prim_to_im!(u16);
prim_to_im!(u32);
prim_to_im!(u64);
prim_to_im!(u128);
prim_to_im!(i8);
prim_to_im!(i16);
prim_to_im!(i32);
prim_to_im!(i64);
prim_to_im!(i128);
prim_to_im!(f32);
prim_to_im!(f64);
prim_to_im!(usize);
prim_to_im!(isize);