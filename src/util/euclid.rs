use std::ops::{AddAssign, ShrAssign, Sub};

use num::traits::{Euclid, Num, Signed};

fn abs<T>(a: T) -> T
where
    T: Num + Sub<Output = T> + PartialOrd,
{
    if a < T::zero() {
        T::zero() - a
    } else {
        a
    }
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Euclid + Num + Copy + PartialOrd,
{
    let mut a = abs(a);
    let mut b = abs(b);
    let mut t;
    while b != T::zero() {
        t = b;
        b = a.rem_euclid(&b);
        a = t;
    }
    a
}

pub fn egcd<T>(mut a: T, mut b: T) -> (T, T, T)
where
    T: Num + Copy,
{
    let one = T::one();
    let zero = T::zero();
    let (mut sa, mut ta, mut sb, mut tb) = (one, zero, zero, one);
    while b != zero {
        let (q, r) = (a / b, a % b);
        (sa, ta, sb, tb) = (sb, tb, sa - q * sb, ta - q * tb);
        (a, b) = (b, r);
    }
    (a, sa, ta)
}

pub fn sgcd<T>(x: T, y: T) -> T
where
    T: Num + Copy,
{
    if y == T::zero() {
        x
    } else {
        sgcd(y, x % y)
    }
}

pub fn crt<T>(a: &[T], m: &[T]) -> Option<T>
where
    T: Num + Copy + Euclid + AddAssign,
{
    use std::iter::zip;
    let one = T::one();
    let zero = T::zero();
    let (mut x, mut m_prod) = (zero, one);
    for (&ai, &mi) in zip(a, m) {
        let (g, s, _) = egcd(m_prod, mi);
        if (ai - x).rem_euclid(&g) != zero {
            return None;
        }
        x += m_prod * ((s * ((ai - x).rem_euclid(&mi))).div_euclid(&g));
        m_prod = (m_prod * mi).div_euclid(&sgcd(m_prod, mi));
    }
    Some(x.rem_euclid(&m_prod))
}

pub fn mod_inverse<T>(a: T, n: T) -> T
where
    T: Num + Copy + std::fmt::Display + PartialOrd + Signed + Euclid,
{
    let mut t = T::zero();
    let mut new_t = T::one();
    let mut r = n;
    let mut new_r = a;
    while new_r != T::zero() {
        let quotient = r.div_euclid(&new_r);
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }
    if r > T::one() {
        panic!("{a} is not invertible mod {n}");
    }
    if t < T::zero() {
        t = t + n;
    }

    t
}

pub fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: Copy + Num + PartialOrd + ShrAssign,
{
    let two = T::one() + T::one();
    if modulus == T::one() {
        return T::zero();
    }
    let mut result = T::one();
    base = base % modulus;
    while exp > T::zero() {
        if exp % two == T::one() {
            result = result * base % modulus;
        }
        exp >>= T::one();
        base = base * base % modulus;
    }
    result
}
