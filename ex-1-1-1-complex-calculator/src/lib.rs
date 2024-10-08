pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
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
    fn ex_1_2_1() {
        let res = C::new(-3, -1) * C::new(1, -2);
        println!("{}", res);
    }

    #[test]
    fn ex_1_2_3() {
        let res = C::new(0.0, 3.0) / C::new(-1.0, -1.0);
        println!("{}", res);
    }
}
