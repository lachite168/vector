use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

pub trait Scalar<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + AddAssign<Rhs>
    + Sub<Rhs, Output = Output>
    + SubAssign<Rhs>
    + Mul<Rhs, Output = Output>
    + MulAssign<Rhs>
    + Neg<Output = Output>
    + PartialEq
    + Eq
    + Clone
    + Into<f64>
    + fmt::Display
{
}

pub trait Scalarf<Rhs = Self, Output = Self>:
    Scalar + Div<Rhs, Output = Output> + DivAssign<Rhs> + From<f64>
{
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Vector2<T: Scalar> {
    x: T,
    y: T,
}

impl<T: Scalar> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn size(&self) -> f64 {
        (self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone())
            .into()
            .sqrt()
    }

    pub fn angle(&self, other: Self) -> f64 {
        ((self.x.clone() * other.x.clone() + self.y.clone() * other.y.clone()).into()
            / (self.size() * other.size()))
        .acos()
    }
}

impl<T: Scalar> fmt::Display for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Scalarf> Vector2<T> {
    pub fn to_unit(&self) -> Self {
        let size = self.size();
        Self {
            x: (self.x.clone().into() / size).into(),
            y: (self.y.clone().into() / size).into(),
        }
    }
}

impl<T: Scalar> Add for Vector2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Scalar> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Scalar> Sub for Vector2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Scalar> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Scalar> Mul<T> for Vector2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
        }
    }
}

impl<T: Scalar> Mul<Vector2<T>> for Vector2<T> {
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
        self.x.clone() * rhs.x.clone() + self.y.clone() * rhs.y.clone()
    }
}

impl<T: Scalar> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
    }
}

impl<T: Scalarf> Div<T> for Vector2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
        }
    }
}

impl<T: Scalarf> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
    }
}

impl<T: Scalar> Neg for Vector2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Vector3<T: Scalar> {
    x: T,
    y: T,
    z: T,
}

impl<T: Scalar> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn size(&self) -> f64 {
        (self.x.clone() * self.x.clone()
            + self.y.clone() * self.y.clone()
            + self.z.clone() * self.z.clone())
        .into()
        .sqrt()
    }

    pub fn angle(&self, other: Self) -> f64 {
        ((self.x.clone() * self.x.clone()
            + self.y.clone() * self.y.clone()
            + self.z.clone() * self.z.clone())
        .into()
            / (self.size() * other.size()))
        .acos()
    }
}

impl<T: Scalar> fmt::Display for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Scalarf> Vector3<T> {
    pub fn to_unit(&self) -> Self {
        let size = self.size();
        Self {
            x: (self.x.clone().into() / size).into(),
            y: (self.y.clone().into() / size).into(),
            z: (self.z.clone().into() / size).into(),
        }
    }
}

impl<T: Scalar> Add for Vector3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Scalar> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Scalar> Sub for Vector3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Scalar> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Scalar> Mul<T> for Vector3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs.clone(),
            y: self.y * rhs.clone(),
            z: self.z * rhs.clone(),
        }
    }
}

impl<T: Scalar> Mul<Vector3<T>> for Vector3<T> {
    type Output = T;
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        self.x.clone() * rhs.x + self.y.clone() * rhs.y + self.z.clone() * rhs.z
    }
}

impl<T: Scalar> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs.clone();
    }
}

impl<T: Scalarf> Div<T> for Vector3<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs.clone(),
            y: self.y / rhs.clone(),
            z: self.z / rhs.clone(),
        }
    }
}

impl<T: Scalarf> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.clone();
        self.y /= rhs.clone();
        self.z /= rhs.clone();
    }
}

impl<T: Scalar> Rem<Vector3<T>> for Vector3<T> {
    type Output = Self;
    fn rem(self, rhs: Vector3<T>) -> Self::Output {
        Self::Output {
            x: self.y.clone() * rhs.z.clone() - self.z.clone() * rhs.y.clone(),
            y: self.z.clone() * rhs.x.clone() - self.x.clone() * rhs.z.clone(),
            z: self.x.clone() * rhs.y.clone() - self.y.clone() * rhs.x.clone(),
        }
    }
}

impl<T: Scalar> RemAssign for Vector3<T> {
    fn rem_assign(&mut self, rhs: Vector3<T>) {
        let old = Self::new(self.x.clone(), self.y.clone(), self.z.clone());
        self.x = old.y.clone() * rhs.z.clone() - old.z.clone() * rhs.y.clone();
        self.y = old.z.clone() * rhs.x.clone() - old.x.clone() * rhs.z.clone();
        self.z = old.x.clone() * rhs.y.clone() - old.y.clone() * rhs.x.clone();
    }
}

impl<T: Scalar> Neg for Vector3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Scalar for i32 {}

fn main() {
    {
        let a = Vector2::new(1, 2);
        let b = Vector2::new(3, 4);
        let mut c = Vector2::new(0, 0);
        let mut d = 0;

        println!("{:?} {}", a, b);

        c = a + b;
        println!("{}", c);

        c += a;
        println!("{}", c);

        c = a - b;
        println!("{}", c);

        c -= a;
        println!("{}" , c);

        d = a * b;
        println!("{}", d);

        c = a * 2;
        println!("{}", c);

        c *= 3;
        println!("{}", c);

        c = -a;
        println!("{}", c);
    }
    {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(4, 5, 6);
        let mut c = Vector3::new(0, 0, 0);
        let mut d = 0;

        println!("{:?} {}", a, b);

        c = a + b;
        println!("{}", c);

        c += a;
        println!("{}", c);

        c = a - b;
        println!("{}", c);

        c -= a;
        println!("{}" , c);

        d = a * b;
        println!("{}", d);

        c = a * 2;
        println!("{}", c);

        c *= 3;
        println!("{}", c);

        c = a % b;
        println!("{}", c);

        c %= a;
        println!("{}", c);

        c = -a;
        println!("{}", c);
    }
}
