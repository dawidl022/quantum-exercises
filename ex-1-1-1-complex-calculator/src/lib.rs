pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
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

#[cfg(test)]
mod tests {
    use super::Complex;

    #[test]
    fn test_add() {
        let a = Complex::new(3, -1);
        let b = Complex::new(1, 4);
        let c = a + b;
        assert_eq!(c.re, 4);
        assert_eq!(c.im, 3);

        let a = Complex::new(-3, 1);
        let b = Complex::new(2, -4);
        let c = a + b;
        assert_eq!(c.re, -1);
        assert_eq!(c.im, -3);
    }

    #[test]
    fn test_mul() {
        let a = Complex::new(3, -1);
        let b = Complex::new(1, 4);
        let c = a * b;
        assert_eq!(c.re, 7);
        assert_eq!(c.im, 11);

        let a = Complex::new(-3, 1);
        let b = Complex::new(2, -4);
        let c = a * b;
        assert_eq!(c.re, -2);
        assert_eq!(c.im, 14);
    }
}
