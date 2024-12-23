use std::collections::HashMap;

use num::{One, Zero};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UnitKetBra {
    ket: u32,
    bra: u32,
    n: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct KetBra<T> {
    scalar: T,
    ket: u32,
    bra: u32,
    n: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State<T> {
    scalar: T,
    superpositions: Vec<Ket<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UnitKet {
    ket: u32,
    n: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ket<T> {
    scalar: T,
    ket: u32,
    n: u32,
}

impl<T: Copy + std::ops::Mul<Output = T>> KetBra<T> {
    fn tensor(&self, other: &KetBra<T>) -> KetBra<T> {
        KetBra {
            scalar: self.scalar * other.scalar,
            ket: self.ket * 2u32.pow(other.n) + other.ket,
            bra: self.bra * 2u32.pow(other.n) + other.bra,
            n: self.n + other.n,
        }
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for KetBra<T> {
    type Output = KetBra<T>;

    fn neg(self) -> Self::Output {
        KetBra {
            scalar: -self.scalar,
            ket: self.ket,
            bra: self.bra,
            n: self.n,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Operator<T> {
    scalar: T,
    ones: Vec<KetBra<T>>,
}

impl<T: Copy + std::ops::Mul<Output = T>> Operator<T> {
    fn tensor(&self, other: &Operator<T>) -> Operator<T> {
        let mut ones = Vec::new();

        for kb in &self.ones {
            for other_kb in &other.ones {
                ones.push(kb.tensor(other_kb));
            }
        }

        Operator {
            scalar: self.scalar * other.scalar,
            ones,
        }
    }
}

impl<T: Copy + std::ops::Mul<Output = T> + std::ops::AddAssign + Zero + PartialEq>
    std::ops::Mul<Operator<T>> for Operator<T>
{
    type Output = Operator<T>;

    fn mul(self, rhs: Operator<T>) -> Self::Output {
        // accumulate scalar multipliers for each KetBra to simplify terms
        let mut ones: HashMap<UnitKetBra, T> = Default::default();

        for kb in &self.ones {
            for other_kb in &rhs.ones {
                if kb.bra == other_kb.ket {
                    let unit = UnitKetBra {
                        ket: kb.ket,
                        bra: other_kb.bra,
                        n: kb.n, // TODO assert kb.n == other_kb.n
                    };
                    let scalar = kb.scalar * other_kb.scalar;

                    *ones.entry(unit).or_insert(T::zero()) += scalar;
                }
            }
        }

        Operator {
            scalar: self.scalar * rhs.scalar,
            ones: ones
                .into_iter()
                .filter(|(_, scalar)| *scalar != T::zero())
                .map(|(k, scalar)| KetBra {
                    scalar,
                    ket: k.ket,
                    bra: k.bra,
                    n: k.n,
                })
                .collect(),
        }
    }
}

impl<T: Copy + std::ops::AddAssign + num::Zero + num::One + PartialEq> std::ops::Add<Operator<T>>
    for Operator<T>
{
    type Output = Operator<T>;

    fn add(self, rhs: Operator<T>) -> Self::Output {
        let mut ones: HashMap<UnitKetBra, T> = Default::default();

        for kb in self.ones.iter().chain(rhs.ones.iter()) {
            let unit = UnitKetBra {
                ket: kb.ket,
                bra: kb.bra,
                n: kb.n,
            };

            // TODO test operator scalar affecting result
            *ones.entry(unit).or_insert(T::zero()) += kb.scalar * self.scalar;
        }

        Operator {
            scalar: T::one(),
            ones: ones
                .into_iter()
                .filter(|(_, scalar)| *scalar != T::zero())
                .map(|(k, scalar)| KetBra {
                    scalar,
                    ket: k.ket,
                    bra: k.bra,
                    n: k.n,
                })
                .collect(),
        }
    }
}

impl<T: Copy + std::ops::Mul<Output = T> + std::ops::AddAssign + num::Zero + PartialEq>
    std::ops::Mul<State<T>> for Operator<T>
{
    type Output = State<T>;

    fn mul(self, rhs: State<T>) -> Self::Output {
        // accumulate scalar multipliers for each KetBra to simplify terms
        let mut superpositions: HashMap<UnitKet, T> = Default::default();

        for kb in &self.ones {
            for pos in &rhs.superpositions {
                if kb.bra == pos.ket {
                    let unit = UnitKet {
                        ket: kb.ket,
                        n: kb.n, // TODO assert kb.n == pos.n
                    };
                    let scalar = kb.scalar * pos.scalar;

                    *superpositions.entry(unit).or_insert(T::zero()) += scalar;
                }
            }
        }

        State {
            scalar: self.scalar * rhs.scalar,
            superpositions: superpositions
                .into_iter()
                .filter(|(_, scalar)| *scalar != T::zero())
                .map(|(k, scalar)| Ket {
                    scalar,
                    ket: k.ket,
                    n: k.n,
                })
                .collect(),
        }
    }
}

impl<T: std::ops::Mul<Output = T>> std::ops::Mul<T> for Operator<T> {
    type Output = Operator<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            scalar: self.scalar * rhs,
            ..self
        }
    }
}

impl<T: One> Operator<T> {
    fn identity(n: u32) -> Self {
        Operator {
            scalar: T::one(),
            // TODO this should depend on n (currently only works for 2x2 identity)
            ones: vec![
                KetBra {
                    scalar: T::one(),
                    ket: 0,
                    bra: 0,
                    n,
                },
                KetBra {
                    scalar: T::one(),
                    ket: 1,
                    bra: 1,
                    n,
                },
            ],
        }
    }
}

impl<T: std::fmt::Display + One + PartialEq> std::fmt::Display for Operator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.scalar != T::one() {
            write!(f, "{}(", self.scalar)?;
        }

        write!(f, "{}", self.ones[0])?;

        for kb in self.ones.iter().skip(1) {
            write!(f, " + {}", kb)?;
        }

        if self.scalar != T::one() {
            write!(f, ")")?;
        }

        Ok(())
    }
}

impl<T: std::fmt::Display + One + PartialEq> std::fmt::Display for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.scalar != T::one() {
            write!(f, "{}(", self.scalar)?;
        }

        write!(f, "{}", self.superpositions[0])?;

        for kb in self.superpositions.iter().skip(1) {
            write!(f, " + {}", kb)?;
        }

        if self.scalar != T::one() {
            write!(f, ")")?;
        }

        Ok(())
    }
}

impl<T: std::fmt::Display + One + PartialEq> std::fmt::Display for KetBra<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.scalar != T::one() {
            write!(f, "{}", self.scalar)?;
        }
        write!(
            f,
            "|{:0>width$b}⟩⟨{:0>width$b}|",
            self.ket,
            self.bra,
            width = self.n as usize
        )
    }
}

impl<T: std::fmt::Display + One + PartialEq> std::fmt::Display for Ket<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.scalar != T::one() {
            write!(f, "{}", self.scalar)?;
        }
        write!(f, "|{:0>width$b}⟩", self.ket, width = self.n as usize)
    }
}

#[cfg(test)]
mod tests {
    use crate::ket::{Ket, State};

    use super::{KetBra, Operator};

    fn k(ket: u32, n: u32) -> Ket<i32> {
        Ket { scalar: 1, ket, n }
    }

    fn k_f64(ket: u32, n: u32) -> Ket<f64> {
        Ket {
            scalar: 1.0,
            ket,
            n,
        }
    }

    fn kb(ket: u32, bra: u32, n: u32) -> KetBra<i32> {
        KetBra {
            scalar: 1,
            ket,
            bra,
            n,
        }
    }

    fn kb_f64(ket: u32, bra: u32, n: u32) -> KetBra<f64> {
        KetBra {
            scalar: 1.0,
            ket,
            bra,
            n,
        }
    }

    #[test]
    fn ket_bra_tensor() {
        let a = kb(0, 0, 1);
        let b = kb(0, 0, 1);
        let c = a.tensor(&b);

        assert_eq!(c, kb(0, 0, 2));

        let a = kb(0, 0, 2);
        let b = kb(0, 0, 1);
        let c = a.tensor(&b);

        assert_eq!(c, kb(0, 0, 3));

        let a = kb(0, 1, 1);
        let b = kb(0, 0, 1);
        let c = a.tensor(&b);

        assert_eq!(c, kb(0, 2, 2));

        let a = kb(0, 0, 1);
        let b = kb(0, 1, 1);
        let c = a.tensor(&b);

        assert_eq!(c, kb(0, 1, 2));

        let a = kb(0, 1, 1);
        let b = kb(0, 1, 1);

        assert_eq!(a.tensor(&b), kb(0, 3, 2));

        let a = kb(1, 0, 1);
        let b = kb(0, 0, 1);

        assert_eq!(a.tensor(&b), kb(2, 0, 2));

        let a = kb(0, 0, 1);
        let b = kb(1, 0, 1);

        assert_eq!(a.tensor(&b), kb(1, 0, 2));

        let a = kb(1, 0, 1);
        let b = kb(1, 0, 1);

        assert_eq!(a.tensor(&b), kb(3, 0, 2));

        let a = kb(1, 0, 1);
        let b = kb(0, 1, 1);

        assert_eq!(a.tensor(&b), kb(2, 1, 2));
    }

    #[test]
    fn operator_tensor_product() {
        let b = Operator {
            scalar: 1,
            ones: vec![kb(1, 1, 1)],
        };
        let id = Operator::<i32>::identity(1);

        let b0 = id.tensor(&b);

        println!("{}", b0);
    }

    #[test]
    fn operator_product() {
        let b = Operator {
            scalar: 1,
            ones: vec![kb(1, 1, 1)],
        };
        let id = Operator::<i32>::identity(1);

        let b0 = id.tensor(&b);
        let b1 = b.tensor(&id);

        let b1b0 = b1 * b0;

        println!("{}", b1b0);

        assert_eq!(
            b1b0,
            Operator {
                scalar: 1,
                ones: vec![kb(3, 3, 2)],
            }
        );
    }

    #[test]
    fn ex_3_4_1() {
        let h = Operator {
            scalar: 1.0 / 2.0_f64.sqrt(),
            ones: vec![
                kb_f64(0, 1, 1),
                kb_f64(1, 0, 1),
                kb_f64(0, 0, 1),
                -kb_f64(1, 1, 1),
            ],
        };
        let id = Operator::<f64>::identity(1);

        let h0 = id.tensor(&h);
        let h1 = h.tensor(&id);

        println!("h0 = {}", h0);
        println!("h1 = {}", h1);

        let h0h1 = h0 * h1;

        println!("h0h1 = {}", h0h1);

        // |00⟩ ⟨00| + |01⟩ ⟨01| + |11⟩ ⟨10| + |10⟩ ⟨11|
        let c10 = Operator {
            scalar: 1.0,
            ones: vec![
                kb_f64(0, 0, 2),
                kb_f64(1, 1, 2),
                kb_f64(3, 2, 2),
                kb_f64(2, 3, 2),
            ],
        };

        let c01 = h0h1.clone() * c10 * h0h1;

        println!("h0h1c10h0h1 = {} = c01", c01);
    }

    #[test]
    fn ex_3_5_1() {
        let x = Operator {
            scalar: 1.0,
            ones: vec![kb_f64(0, 1, 1), kb_f64(1, 0, 1)],
        };
        let x1 = x.tensor(&Operator::identity(1));

        println!("{}", x1);

        let z = Operator {
            scalar: 1.0,
            ones: vec![kb_f64(0, 0, 1), -kb_f64(1, 1, 1)],
        };
        let z1 = z.tensor(&Operator::identity(1));

        println!("{}", z1);

        let h = (x + z) * (1.0 / 2.0f64.sqrt());
        println!("{}", h);

        let h1 = (x1 + z1) * (1.0 / 2.0f64.sqrt());

        println!("{}", h1);

        // |00⟩ ⟨00| + |01⟩ ⟨01| + |11⟩ ⟨10| + |10⟩ ⟨11|
        let c10 = Operator {
            scalar: 1.0,
            ones: vec![
                kb_f64(0, 0, 2),
                kb_f64(1, 1, 2),
                kb_f64(3, 2, 2),
                kb_f64(2, 3, 2),
            ],
        };

        let op = c10 * h1;

        println!("{}", op);

        let inp = [
            State {
                scalar: 1.0,
                superpositions: vec![k_f64(0, 2)],
            },
            State {
                scalar: 1.0,
                superpositions: vec![k_f64(1, 2)],
            },
            State {
                scalar: 1.0,
                superpositions: vec![k_f64(2, 2)],
            },
            State {
                scalar: 1.0,
                superpositions: vec![k_f64(3, 2)],
            },
        ];

        for inp in inp {
            let res = op.clone() * inp.clone();
            println!("{inp} -> {}", res);
        }
    }
}
