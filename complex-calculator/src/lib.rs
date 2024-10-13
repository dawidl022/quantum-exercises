mod matrix;
mod polar;

#[derive(Debug, Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T> PartialEq<T> for Complex<T>
where
    T: PartialEq + num::traits::Zero,
{
    fn eq(&self, other: &T) -> bool {
        self.re == *other && self.im == T::zero()
    }
}

impl<T> std::fmt::Display for Complex<T>
where
    T: std::fmt::Display,
    T: num::traits::Zero,
    T: PartialOrd,
    T: std::ops::Neg<Output = T>,
    T: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im < T::zero() {
            return write!(f, "{} - {}i", self.re, -self.im);
        }
        write!(f, "{} + {}i", self.re, self.im)
    }
}

impl<T, O> std::ops::Add<Complex<T>> for Complex<T>
where
    T: std::ops::Add<Output = O>,
{
    type Output = Complex<O>;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T, O> std::ops::Sub<Complex<T>> for Complex<T>
where
    T: std::ops::Sub<Output = O>,
{
    type Output = Complex<O>;

    fn sub(self, rhs: Complex<T>) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T, O, O1> std::ops::Mul<Complex<T>> for Complex<T>
where
    T: Copy + std::ops::Mul<Output = O1>,
    O1: std::ops::Add<Output = O>,
    O1: std::ops::Sub<Output = O>,
{
    type Output = Complex<O>;

    fn mul(self, rhs: Complex<T>) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T, O, O1, O2> std::ops::Div<Complex<T>> for Complex<T>
where
    T: Copy + std::ops::Mul<Output = O1>,
    O1: std::ops::Add<Output = O2>,
    O1: std::ops::Sub<Output = O2>,
    O2: Copy + std::ops::Div<Output = O>,
{
    type Output = Complex<O>;

    fn div(self, rhs: Complex<T>) -> Self::Output {
        let den = rhs.re * rhs.re + rhs.im * rhs.im;
        Complex {
            re: (self.re * rhs.re + self.im * rhs.im) / den,
            im: (self.im * rhs.re - self.re * rhs.im) / den,
        }
    }
}

impl<T> Complex<T> {
    pub fn mod_squared<O, O1>(&self) -> O
    where
        T: Copy + std::ops::Mul<Output = O1>,
        O1: std::ops::Add<Output = O>,
    {
        self.re * self.re + self.im * self.im
    }

    pub fn conjugate(&self) -> Complex<T>
    where
        T: Copy + std::ops::Neg<Output = T>,
    {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
}

impl Complex<f64> {
    pub fn modulus(&self) -> f64 {
        self.mod_squared().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;

    type C<T> = Complex<T>;

    #[test]
    fn test_add() {
        let a = Complex::new(3, -1);
        let b = Complex::new(1, 4);
        let c = a + b;
        assert_eq!(c.re, 4);
        assert_eq!(c.im, 3);
        assert_eq!(format!("{}", c), "4 + 3i");

        let a = Complex::new(-3, 1);
        let b = Complex::new(2, -4);
        let c = a + b;
        assert_eq!(c.re, -1);
        assert_eq!(c.im, -3);
        assert_eq!(format!("{}", c), "-1 - 3i");
    }

    #[test]
    fn test_mul() {
        let a = Complex::new(3, -1);
        let b = Complex::new(1, 4);
        let c = a * b;
        assert_eq!(c.re, 7);
        assert_eq!(c.im, 11);
        assert_eq!(format!("{}", c), "7 + 11i");

        let a = Complex::new(-3, 1);
        let b = Complex::new(2, -4);
        let c = a * b;
        assert_eq!(c.re, -2);
        assert_eq!(c.im, 14);
        assert_eq!(format!("{}", c), "-2 + 14i");
    }

    #[test]
    fn test_mod() {
        let a = C::new(1.0, -1.0);
        assert_eq!(a.modulus(), 2.0_f64.sqrt());

        let a = C::new(4.0, -3.0);
        assert_eq!(a.modulus(), 5.0);
    }

    #[test]
    fn test_conj() {
        let a = C::new(1, -1);
        let b = a.conjugate();
        assert_eq!(b.re, 1);
        assert_eq!(b.im, 1);
    }

    #[test]
    fn c_mult_c_conj_eq_mod_squared() {
        let a = C::new(3, 2);
        let b = a * a.conjugate();
        let c = a.mod_squared();

        assert_eq!(b, c);
    }

    #[test]
    fn ex_1_2_1() {
        let res = C::new(-3, -1) * C::new(1, -2);
        println!("{}", res);
    }

    #[test]
    fn ex_1_2_3() {
        let res = C::new(0.0, 3.0) / C::new(-1.0, -1.0);
        println!("{}", res);
    }

    #[test]
    fn ex_1_3_4() {
        let res = C::new(-2, -1) * C::new(-1, -2);
        println!("{}", res);
    }

    #[test]
    fn ex_1_3_7() {
        let res = C::new(2, 2) / C::new(1, -1);
        println!("{}", res);
    }

    #[test]
    fn ex_1_3_8() {
        let a = C::new(1, -1);
        let res = a * a * a * a * a;
        println!("{}", res);
    }
}
