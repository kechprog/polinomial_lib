use std::ops;
use num_traits::{Pow, Zero};



#[derive(Debug, Clone)]
pub struct Polinomial<T>
where T: ops::Sub<Output = T> + Copy + Eq
{
    pub coefficients: Vec<T>,
}

impl<T: ops::Sub<Output = T> + ops::Mul<Output=T> + Copy + Eq + Pow<usize, Output = T> + Zero> Polinomial<T> {

    /// Coefficients are from x^n ... c
    /// For example: 3x^2 + x + 1 = vec![3, 1, 1]
    pub fn new(coefficients: &[T]) -> Self {
        Self {
            coefficients: coefficients.to_vec().iter().rev().cloned().collect(),
        }
    }

    /// calculates f(x) of a given polinomial
    pub fn f(&self, x: T) -> T {
        let mut sum = T::zero();
        for (idx, coef) in self.coefficients.iter().enumerate() {
            let s = x.pow(idx);
            sum = sum + s * (*coef);
        }

        sum
    }
}


impl<T> ops::Div for Polinomial<T>
where T: ops::Sub<Output = T> + ops::Div<Output = T>
+ ops::Mul<Output=T> + Copy + Eq 
+ Pow<usize, Output = T> + Zero {

    type Output = Self;

    /// polinomial long division we learn in grade 11
    /// (x^2 + 2x + 1) / (x + 1) = (x+1)
    ///
    /// NOTE:
    ///  the remainder is not calculated
    ///  and not returned use % operator for that
    fn div(self, rhs: Self) -> Self::Output {
        todo!("Fix it is not working, reimplementation needed");
    }
}

impl<T> ops::Mul for Polinomial<T>
where T: ops::Sub<Output = T> 
+ ops::Mul<Output=T> + Copy + Eq 
+ Pow<usize, Output = T> + Zero {

    type Output = Self;

    /// multiplication of 2 polinomials
    /// (a + b)(c + d) = ac + ad + bc + bd
    fn mul(self, rhs: Self) -> Self::Output {
        let c1 = self.coefficients;
        let c2 = rhs.coefficients;

        let mut result = vec![];

        for (idx1, coef1) in c1.iter().enumerate() {
            for (idx2, coef2) in c2.iter().enumerate() {
                let idx = idx1 + idx2;
                let coef = (*coef1) * (*coef2);

                if result.len() <= idx {
                    result.push(coef);
                } else {
                    result[idx] = result[idx] + coef;
                }
            }
        }

        Self::new(&result)
    }
}


pub fn polinomial_power<T: ops::Sub<Output = T> + Copy + Eq>(mut values: Vec<T>) -> usize {

    let mut power = 0_usize;

    let all_same = |v: &Vec<_>| v.windows(2).all(|w| w[0] == w[1]);

    if all_same(&values) {
        return 0;
    }

    while values.len() > 0 {
        values = values.windows(2).map(|w| w[1] - w[0]).collect();
        if !all_same(&values) {
            power += 1;
        } else {
            break;
        }
    }

    power + 1
}