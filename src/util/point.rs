use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
    str::FromStr,
};

use anyhow::anyhow;
use num::traits::{CheckedAdd, CheckedSub, Euclid, Num, NumAssign};
use strum::{EnumIter, IntoEnumIterator};

use super::euclid::gcd;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pt<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    pub fn turn(&mut self, turn: Turn) {
        *self = *self + turn;
    }
}

impl Add<Turn> for Dir {
    type Output = Dir;

    fn add(self, rhs: Turn) -> Self::Output {
        match (self, rhs) {
            (Dir::S, Turn::R) | (Dir::N, Turn::L) => Dir::W,
            (Dir::N, Turn::R) | (Dir::S, Turn::L) => Dir::E,
            (Dir::W, Turn::R) | (Dir::E, Turn::L) => Dir::N,
            (Dir::W, Turn::L) | (Dir::E, Turn::R) => Dir::S,
        }
    }
}

impl Add<Turn> for &Dir {
    type Output = Dir;

    fn add(self, rhs: Turn) -> Self::Output {
        *self + rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir2 {
    U,
    D,
    L,
    R,
}

impl<T> Display for Pt<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl FromStr for Dir2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "D" => Ok(Self::D),
            "L" => Ok(Self::L),
            "R" => Ok(Self::R),
            _ => Err(anyhow!("{s} is not a valid Dir2")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum Turn {
    L,
    R,
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Turn::L => 'L',
                Turn::R => 'R',
            }
        )
    }
}

impl<T> Add<Pt<T>> for Pt<T>
where
    T: Add<Output = T>,
{
    type Output = Pt<T>;

    fn add(self, rhs: Pt<T>) -> Self::Output {
        Pt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<T> AddAssign<Dir> for Pt<T>
where
    T: Num + NumAssign,
{
    fn add_assign(&mut self, rhs: Dir) {
        match rhs {
            Dir::N => self.y -= T::one(),
            Dir::S => self.y += T::one(),
            Dir::E => self.x += T::one(),
            Dir::W => self.x -= T::one(),
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<T> Add<Dir> for Pt<T>
where
    T: Num,
{
    type Output = Pt<T>;

    fn add(self, rhs: Dir) -> Self::Output {
        let (x, y) = match rhs {
            Dir::N => (self.x, self.y - T::one()),
            Dir::S => (self.x, self.y + T::one()),
            Dir::E => (self.x + T::one(), self.y),
            Dir::W => (self.x - T::one(), self.y),
        };
        Pt { x, y }
    }
}

impl<T> Sub<Pt<T>> for Pt<T>
where
    T: Sub<Output = T>,
{
    type Output = Pt<T>;

    fn sub(self, rhs: Pt<T>) -> Self::Output {
        Pt {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

pub const ORIGINI32: Pt<i32> = Pt { x: 0, y: 0 };

impl<T> Pt<T>
where
    T: Copy + Num + Euclid + Ord,
{
    pub fn normalize(self) -> Self {
        let gcd = gcd(self.x, self.y);
        Self {
            x: self.x / gcd,
            y: self.y / gcd,
        }
    }
}

impl<T> Pt<T> {
    pub fn checked_add_dir(&self, dir: Dir) -> Option<Self>
    where
        T: CheckedAdd + CheckedSub + Num + Copy,
    {
        match dir {
            Dir::N => self.y.checked_sub(&T::one()).map(|y| Self { x: self.x, y }),
            Dir::S => self.y.checked_add(&T::one()).map(|y| Self { x: self.x, y }),
            Dir::E => self.x.checked_add(&T::one()).map(|x| Self { x, y: self.y }),
            Dir::W => self.x.checked_sub(&T::one()).map(|x| Self { x, y: self.y }),
        }
    }

    pub fn neighbors(self) -> impl Iterator<Item = Pt<T>>
    where
        T: CheckedAdd + CheckedSub + Num + Copy,
    {
        Dir::iter().filter_map(move |d| self.checked_add_dir(d))
    }
}

impl Pt<usize> {
    pub fn checked_add_signed(&self, other: &Pt<isize>) -> Option<Self> {
        self.x
            .checked_add_signed(other.x)
            .and_then(|x| self.y.checked_add_signed(other.y).map(|y| Pt { x, y }))
    }
}

impl Pt<i32> {
    pub fn manhattan_distance(&self, other: &Pt<i32>) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl<T> AddAssign<Pt<T>> for Pt<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Pt<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> From<(T, T)> for Pt<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Pt<T>> for (T, T) {
    fn from(value: Pt<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T> FromStr for Pt<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(',')
            .ok_or(anyhow!("{s} must include a comma to be parsed as Pt"))?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
pub struct Pt3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Sum for Pt3<T>
where
    T: Default + AddAssign,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut out = Pt3::default();
        for n in iter {
            out += n;
        }
        out
    }
}

impl<T> AddAssign for Pt3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> From<Pt3<T>> for (T, T, T) {
    fn from(value: Pt3<T>) -> Self {
        (value.x, value.y, value.z)
    }
}

impl<T> From<(T, T, T)> for Pt3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl<T> Add for Pt3<T>
where
    T: Add<Output = T>,
{
    type Output = Pt3<T>;

    fn add(self, rhs: Pt3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Pt3<T>
where
    T: Sub<Output = T>,
{
    type Output = Pt3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Pt3<isize> {
    pub fn abs_norm(&self, other: &Pt3<isize>) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

impl<T> FromStr for Pt3<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr: [&str; 3] = s
            .trim()
            .trim_start_matches(['(', '<'])
            .trim_end_matches([')', '>'])
            .split(',')
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Ok(Self {
            x: arr[0].parse()?,
            y: arr[1].parse()?,
            z: arr[2].parse()?,
        })
    }
}

impl<T> Div<T> for Pt3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Pt3<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Mul<T> for Pt3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Pt3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Pt3<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> Display for Pt3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}
