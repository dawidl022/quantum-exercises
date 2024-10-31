#[derive(Debug, Clone)]
pub struct Vector<T, const N: usize>([T; N]);

#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const N: usize> std::ops::Add<Vector<T, N>> for Vector<T, N>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn add(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result = Vector(self.0);
        for i in 0..N {
            result.0[i] = result.0[i] + rhs.0[i];
        }
        result
    }
}

impl<T, const M: usize, const N: usize> std::ops::Add<Matrix<T, M, N>> for Matrix<T, M, N>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Matrix<T, M, N>;

    fn add(self, rhs: Matrix<T, M, N>) -> Self::Output {
        let mut result = Matrix(self.0);
        for i in 0..M {
            for j in 0..N {
                result.0[i][j] = result.0[i][j] + rhs.0[i][j];
            }
        }
        result
    }
}

impl<T, S, const N: usize> std::ops::Mul<S> for Vector<T, N>
where
    T: Copy,
    S: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn mul(self, rhs: S) -> Self::Output {
        let mut result = Vector(self.0);
        for i in 0..N {
            result.0[i] = rhs * result.0[i];
        }
        result
    }
}

impl<T, S, const M: usize, const N: usize> std::ops::Mul<S> for Matrix<T, M, N>
where
    T: Copy,
    S: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Matrix<T, M, N>;

    fn mul(self, rhs: S) -> Self::Output {
        let mut result = Matrix(self.0);
        for i in 0..M {
            for j in 0..N {
                result.0[i][j] = rhs * result.0[i][j];
            }
        }
        result
    }
}

impl<T, const N: usize> std::ops::Neg for Vector<T, N>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Vector<T, N>;

    fn neg(self) -> Self::Output {
        let mut result = Vector(self.0);
        for i in 0..N {
            result.0[i] = -result.0[i];
        }
        result
    }
}

impl<T, const N: usize> num::Zero for Vector<T, N>
where
    T: num::Zero + Copy,
{
    fn zero() -> Self {
        Vector([T::zero(); N])
    }

    fn is_zero(&self) -> bool {
        for i in 0..N {
            if !self.0[i].is_zero() {
                return false;
            }
        }
        true
    }
}

impl<T, const N: usize> std::cmp::PartialEq for Vector<T, N>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}

impl<T, const N: usize, const M: usize> std::cmp::PartialEq for Matrix<T, M, N>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            for j in 0..N {
                if self.0[i][j] != other.0[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl<T, const N: usize> std::fmt::Display for Vector<T, N>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut longest = 0;

        for i in 0..N {
            let s = self.0[i].to_string();
            if s.len() > longest {
                longest = s.len();
            }
        }
        write!(f, "┌{}┐", " ".repeat(longest + 2))?;
        for i in 0..N {
            let s = self.0[i].to_string();
            write!(f, "\n│ {}{} │", s, " ".repeat(longest - s.len()))?;
        }
        write!(f, "\n└{}┘", " ".repeat(longest + 2))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use num::Zero;

    use super::Vector as V;
    use crate::Complex as C;

    #[test]
    fn test_add() {
        let v1 = V([1, 2, 3]);
        let v2 = V([4, 5, 6]);
        let res = v1 + v2;
        assert_eq!(res.0, [5, 7, 9]);
    }

    #[test]
    fn ex_2_1_1() {
        let v1 = V([
            C::new(5.0, 13.0),
            C::new(6.0, 2.0),
            C::new(0.53, -6.0),
            C::new(12.0, 0.0),
        ]);
        let v2 = V([
            C::new(7.0, -8.0),
            C::new(0.0, 4.0),
            C::new(2.0, 0.0),
            C::new(9.4, 3.0),
        ]);

        let res = v1 + v2;
        println!("{}", res);
    }

    #[test]
    fn test_mul() {
        let v = V([1, 2, 3]);
        let res = v * 2;
        assert_eq!(res.0, [2, 4, 6]);
    }

    #[test]
    fn ex_2_1_3() {
        let v = V([
            C::new(16.0, 2.3),
            C::new(0.0, -7.0),
            C::new(6.0, 0.0),
            C::new(5.0, -4.0),
        ]);
        let res = v * C::new(8.0, -2.0);
        println!("{}", res);
    }

    #[test]
    fn test_neg() {
        let v = V([C::new(1, 0), C::new(2, -3), C::new(-3, 2)]);
        let res = -v;
        assert_eq!(res.0, [C::new(-1, 0), C::new(-2, 3), C::new(3, -2)]);
    }

    #[test]
    fn inversion_property() {
        let v = V([C::new(1, 2), C::new(3, 4), C::new(5, 6)]);
        let res = v.clone() + -v.clone();
        assert_eq!(res, V::zero());

        let res = -v.clone() + v.clone();
        assert_eq!(res, V::zero());
    }

    #[test]
    fn test_add_matrix() {
        let m1 = super::Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let m2 = super::Matrix([[9, 8, 7], [6, 5, 4], [3, 2, 1]]);
        let res = m1 + m2;
        assert_eq!(res.0, [[10, 10, 10], [10, 10, 10], [10, 10, 10]]);
    }

    #[test]
    fn test_mul_matrix() {
        let m = super::Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let res = m * 2;
        assert_eq!(res.0, [[2, 4, 6], [8, 10, 12], [14, 16, 18]]);
    }

    #[test]
    fn ex2_2_3() {
        let a = super::Matrix([[C::new(1, -1), C::new(3, 0)], [C::new(2, 2), C::new(4, 1)]]);
        let c1 = C::new(0, 2);
        let c2 = C::new(1, 2);

        assert_eq!((a.clone() * c2) * c1, a.clone() * (c1 * c2));
        assert_eq!(a.clone() * (c1 + c2), a.clone() * c1 + a * c2);
    }

    #[test]
    fn ex_2_2_8() {
        let ba00 = C::new(5, 0) * C::new(3, 2)
            + C::new(2, -1) * C::new(1, 0)
            + C::new(6, -4) * C::new(4, -1);

        let ba01 = C::new(2, -1) * C::new(4, 2);

        let ba02 = C::new(5, 0) * C::new(5, -6)
            + C::new(2, -1) * C::new(0, 1)
            + C::new(6, -4) * C::new(4, 0);

        // println!("{}", Matrix([[ba00, ba01, ba02]]));
        println!("{}", ba00);
        println!("{}", ba01);
        println!("{}", ba02);
    }
}
