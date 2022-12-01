mod polinomial;
pub use polinomial::*;



/*      TEST      */
#[cfg(test)]
mod tests {
    use super::polinomial::*;

    #[test]
    fn polinomial_power_test() {
        assert_eq!(polinomial_power(vec![1, 2, 3, 4, 5]),       1);
        assert_eq!(polinomial_power(vec![1, 4, 9, 16, 25]),     2);
        assert_eq!(polinomial_power(vec![1, 8, 27, 64, 125]),   3);
        assert_eq!(polinomial_power(vec![1, 16, 81, 256, 625]), 4);
    }

    #[test]
    fn polinomial_f_test() {
        let p = Polinomial::new(&[1, 2, 3]);
        for i in 0..10 {
            let r = p.f(i);
            assert_eq!(r, (i as i32).pow(2u32) + 2 * i + 3);
        }
        let p = Polinomial::new(&[1, 2, 3, 4]);
        for i in 0..10 {
            let r = p.f(i);
            assert_eq!(r, (i as i32).pow(3u32) + 2 * (i as i32).pow(2u32) + 3 * i + 4);
        }
    }

    #[test]
    fn polinomial_mul_test() {
        let p1 = Polinomial::new(&[1, 2, 3]);
        let p2 = Polinomial::new(&[1, 2, 3]);
        let r = p1 * p2;
        assert_eq!(r.coefficients, vec![1, 4, 10, 12, 9]);
    }

    

}