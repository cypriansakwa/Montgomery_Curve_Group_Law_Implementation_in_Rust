use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: Option<BigInt>,
    y: Option<BigInt>,
}

impl Point {
    fn new(x: BigInt, y: BigInt) -> Self {
        Point {
            x: Some(x),
            y: Some(y),
        }
    }

    fn infinity() -> Self {
        Point { x: None, y: None }
    }

    fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    fn mod_inverse(value: &BigInt, modulo: &BigInt) -> BigInt {
        let mut t = BigInt::zero();
        let mut new_t = BigInt::one();
        let mut r = modulo.clone();
        let mut new_r = value.clone();

        while !new_r.is_zero() {
            let quotient = &r / &new_r;
            t = t - &quotient * &new_t;
            std::mem::swap(&mut t, &mut new_t);
            r = r - &quotient * &new_r;
            std::mem::swap(&mut r, &mut new_r);
        }

        if t < BigInt::zero() {
            t = t + modulo;
        }

        t
    }

    fn add(&self, other: &Point, a: &BigInt, b: &BigInt, modulo: &BigInt) -> Point {
        if self.is_infinity() {
            return other.clone();
        }
        if other.is_infinity() {
            return self.clone();
        }

        let (x1, y1) = (self.x.clone().unwrap(), self.y.clone().unwrap());
        let (x2, y2) = (other.x.clone().unwrap(), other.y.clone().unwrap());

        // Handle the special case where the points are inverses of each other
        if x1 == x2 && (y1.clone() + y2.clone()) % modulo == BigInt::zero() {
            return Point::infinity();
        }

        let slope = if x1 == x2 { 
            // Point doubling formula
            let mut num = (3 * &x1 * &x1 + 2 * a * &x1 + BigInt::one()) % modulo;
            let mut denom = (2 * b * y1.clone()) % modulo;
        
            // Ensure that num and denom are positive
            if num < BigInt::zero() {
                num += modulo;
            }
            if denom < BigInt::zero() {
                denom += modulo;
            }
        
            (num * Point::mod_inverse(&denom, modulo)) % modulo
        } else {
            // Point addition formula
            let mut num = (y2.clone() - y1.clone()) % modulo;
            let mut denom = (x2.clone() - x1.clone()) % modulo;
        
            // Ensure that num and denom are positive
            if num < BigInt::zero() {
                num += modulo;
            }
            if denom < BigInt::zero() {
                denom += modulo;
            }
        
            (num * Point::mod_inverse(&denom, modulo)) % modulo
        };

        // Calculate x3
        let mut x3 = (slope.clone() * &slope * b - a - &x1 - &x2) % modulo;
        if x3 < BigInt::zero() {
            x3 = (x3 + modulo) % modulo;
        }

        println!("Debug: x3 = {}", x3);

        // Calculate y3 with precise modular reduction
        let mut y3 = (slope * (&x1 - &x3) - y1) % modulo;
        if y3 < BigInt::zero() {
            y3 = (y3 + modulo) % modulo;
        }

        println!("Debug: y3 = {}", y3);

        Point::new(x3, y3)
    }

    fn double(&self, a: &BigInt, b: &BigInt, modulo: &BigInt) -> Point {
        self.add(self, a, b, modulo)
    }

    fn inverse(&self, modulo: &BigInt) -> Point {
        if self.is_infinity() {
            return self.clone();
        }
        let mut y_inv = -self.y.clone().unwrap();
        if y_inv < BigInt::zero() {
            y_inv = (y_inv + modulo) % modulo;
        }
        Point::new(self.x.clone().unwrap(), y_inv)
    }
}

fn main() {
    let a = BigInt::from(3);
    let b = BigInt::from(15);
    let modulo = BigInt::from(17);

    let p = Point::new(BigInt::from(12), BigInt::from(6));
    let q = Point::new(BigInt::from(5), BigInt::from(5));

    let sum = p.add(&q, &a, &b, &modulo);
    println!("P + Q = {:?}", sum);

    let doubled = p.double(&a, &b, &modulo);
    println!("2P = {:?}", doubled);

    let inverse = p.inverse(&modulo);
    println!("-P = {:?}", inverse);
}
