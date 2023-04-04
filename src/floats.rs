use super::*;
use num_traits::Float;

impl<T: Float> C<T>{
    pub fn abs(&self) -> T {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    pub fn arg(&self)->T{ // theta
        (self.1 / self.0).atan()
    }
    pub fn exp(&self)->Self{
        C(self.0.exp()*self.1.cos(), self.0.exp()*self.1.sin())
    }
    pub fn ln(&self)->Self{
        C(self.abs().ln(), self.arg())
    }
    pub fn sin(&self)->Self{
        let nom = C(-self.1, self.0).exp() - C(self.1, -self.0).exp();
        let den = C(T::from(0).unwrap(), T::from(2).unwrap());
        nom / den
    }
    pub fn cos(&self)->Self{
        let nom = C(-self.1, self.0).exp() + C(self.1, -self.0).exp();
        let den = C(T::from(2).unwrap(), T::from(0).unwrap());
        nom / den
    }
    pub fn tan(&self)->Self{
        self.sin() / self.cos()
    }

    pub fn cot(&self)->Self{
        self.cos() / self.sin()
    }

    pub fn sec(&self)->Self{
        C(T::from(1).unwrap(),T::from(0).unwrap()) / self.cos()
    }

    pub fn csc(&self)->Self{
        C(T::from(1).unwrap(), T::from(0).unwrap()) / self.sin()
    }

    pub fn sinh(&self)->Self{
        let nom = self.exp() - (-self).exp();
        let den = C(T::from(2).unwrap(), T::from(0).unwrap());
        nom / den
    }

    pub fn cosh(&self)->Self{
        let nom = self.exp() + (-self).exp();
        let den = C(T::from(2).unwrap(), T::from(0).unwrap());
        nom / den
    }

    pub fn tanh(&self)->Self{
        self.sinh() / self.cosh()
    }

    pub fn coth(&self)->Self{
        self.cosh() / self.sinh()
    }

    pub fn sech(&self)->Self{
        C(T::from(1).unwrap(), T::from(0).unwrap()) / self.cosh()
    }

    pub fn csch(&self)->Self{
        C(T::from(1).unwrap(), T::from(0).unwrap()) / self.sinh()
    }
}