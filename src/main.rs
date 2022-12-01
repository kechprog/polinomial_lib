mod math;
use std::vec;

use math as m;

/*
 * TODO
 * - div
 * - mul
 * - mod
 * - approximations
 */

fn main() {

    let p1 = m::Polinomial::new(&vec![1, 2, 1]);
    let p2 = m::Polinomial::new(&vec![1, 1]);

    let r = p1 / p2;

    println!("{:?}", r);
}