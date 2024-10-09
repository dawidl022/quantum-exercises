use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct ComplexPolar<T> {
    pub mag: T,
    pub pha: T,
}

impl super::Complex<f64> {
    pub fn polar(&self) -> ComplexPolar<f64> {
        let mag = self.modulus();
        let pha = if self.re.abs() != 0.0 {
            (self.im / self.re).atan()
        } else if self.im > 0.0 {
            PI / 2.0
        } else {
            -PI / 2.0
        };

        ComplexPolar { mag, pha }
    }
}

impl ComplexPolar<f64> {
    pub fn cartesian(&self) -> super::Complex<f64> {
        super::Complex {
            re: self.mag * self.pha.cos(),
            im: self.mag * self.pha.sin(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::Complex;

    const EPSILON: f64 = 0.0000001;

    #[test]
    fn test_polar() {
        // test case 1
        let c = Complex::new(1.0, 1.0);
        let c_p = c.polar();

        assert_eq!(c_p.mag, 2.0f64.sqrt());
        assert_eq!(c_p.pha, PI / 4.0);

        let c = c_p.cartesian();
        assert!((c.re - 1.0).abs() < EPSILON); // slight float imprecision introduced
        assert_eq!(c.im, 1.0);

        // test case 2
        let c = Complex::new(2.0, 2.0);
        let c_p = c.polar();

        assert_eq!(c_p.mag, 8.0f64.sqrt());
        assert_eq!(c_p.pha, PI / 4.0);

        let c = c_p.cartesian();
        assert!((c.re - 2.0).abs() < EPSILON);
        assert_eq!(c.im, 2.0);

        // test case 3
        let c = Complex::new(1.0, -1.0);
        let c_p = c.polar();

        assert_eq!(c_p.mag, 2.0f64.sqrt());
        assert_eq!(c_p.pha, -PI / 4.0);

        let c = c_p.cartesian();
        assert!((c.re - 1.0).abs() < EPSILON);
        assert_eq!(c.im, -1.0);
    }

    #[test]
    fn test_polar_imaginary() {
        let c = Complex::new(0.0, 1.0);
        let c_p = c.polar();

        assert_eq!(c_p.mag, 1.0);
        assert_eq!(c_p.pha, PI / 2.0);

        let c = c_p.cartesian();
        assert!(c.re.abs() < EPSILON);
        assert_eq!(c.im, 1.0);
    }

    #[test]
    fn test_polar_neg_imaginary() {
        let c = Complex::new(0.0, -1.0);
        let c_p = c.polar();

        assert_eq!(c_p.mag, 1.0);
        assert_eq!(c_p.pha, -PI / 2.0);

        let c = c_p.cartesian();
        assert!(c.re.abs() < EPSILON);
        assert_eq!(c.im, -1.0);
    }
}
