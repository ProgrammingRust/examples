#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    /// Real portion of the complex number
    re: T,

    /// Imaginary portion of the complex number
    im: T
}

use std::ops::Add;
use std::ops::Mul;

#[cfg(skip)]
impl Add for Complex<i32> {
    type Output = Complex<i32>;
    fn add(self, rhs: Self) -> Self {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output=T>
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

#[cfg(skip)]
impl<L, R, O> Add<Complex<R>> for Complex<L>
    where L: Add<R, Output=O>
{
    type Output = Complex<O>;
    fn add(self, rhs: Complex<R>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

#[cfg(skip)]
impl<'a, P, Rhs> Add for &'a Complex<P>
    where P: Add<Output=P>,
          Rhs: AsRef<Complex<P>>
{
    type Output = Complex<P>;
    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs = rhs.as_ref();
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

use std::ops::Sub;

impl<T> Mul<Complex<T>> for Complex<T>
    where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex { re: self.re * rhs.re - self.im * rhs.im,
                  im: self.re * rhs.im + self.im * rhs.re }
    }
}

#[test]
fn test() {
    let z = Complex { re: -2, im: 6 };
    let c = Complex { re:  1, im: 2 };
    assert_eq!(z + c,    Complex { re: -1, im: 8 });
    assert_eq!(z * c,    Complex { re: -14, im: 2 });
    assert_eq!(z.add(c), Complex { re: -1, im: 8 });
}

#[test]
fn test_explicit() {
    use std::ops::Add;

    assert_eq!(4.125f32.add(5.75), 9.875);
    assert_eq!(10.add(20), 10 + 20);
}

impl Add<Complex<f64>> for f64 {
    type Output = Complex<f64>;
    fn add(self, rhs: Complex<f64>) -> Complex<f64> {
        Complex { re: rhs.re + self, im: rhs.im }
    }
}

#[test]
fn add_complex_to_real() {
    assert_eq!(30f64 + Complex { re: 10.0f64, im: 20.0 },
               Complex { re: 40.0, im: 20.0 });
}

use std::ops::Neg;

impl<T, O> Neg for Complex<T>
    where T: Neg<Output=O>
{
    type Output = Complex<O>;
    fn neg(self) -> Complex<O> {
        Complex { re: -self.re, im: -self.im }
    }
}

#[test]
fn negate_complex() {
    let z = Complex { re: 3, im: 4 };
    assert_eq!(-z, Complex { re: -3, im: -4 });
}

use std::ops::AddAssign;

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

#[test]
fn compound_assignment() {
    let mut z = Complex { re: 5, im: 6 };
    z += Complex { re: 7, im: 8 };
    assert_eq!(z, Complex { re: 12, im: 14 });

    let mut title = "Love".to_string();
    title += ", Actually";
    assert_eq!(title, "Love, Actually");
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Complex<T>) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: Eq> Eq for Complex<T> { }

#[test]
fn comparison() {
    let x = Complex { re: 5, im: 2 };
    let y = Complex { re: 2, im: 5 };
    assert_eq!(x * y, Complex { re: 0, im: 29 });
}

use std::fmt;

// To make the formatting examples mesh with the rest of this file, I've adapted
// them to work on the type `Complex<f64>`, where the book simply defines a new
// non-generic `Complex` type. The only changes are adding `<f64>`, and changing
// the field names.

#[cfg(skip)]
impl fmt::Display for Complex<f64> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let i_sign = if self.i < 0.0 { '-' } else { '+' };
        write!(dest, "{} {} {}i", self.r, i_sign, f64::abs(self.i))
    }
}

impl fmt::Display for Complex<f64> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let (r, i) = (self.re, self.im);
        if dest.alternate() {
            let abs = f64::sqrt(r * r + i * i);
            let angle = f64::atan2(i, r) / std::f64::consts::PI * 180.0;
            write!(dest, "{} ∠ {}°", abs, angle)
        } else {
            let i_sign = if i < 0.0 { '-' } else { '+' };
            write!(dest, "{} {} {}i", r, i_sign, f64::abs(i))
        }
    }
}

#[test]
fn custom_display_impl() {
    let one_twenty = Complex { re: -0.5, im: 0.866 };
    assert_eq!(format!("{}", one_twenty),
               "-0.5 + 0.866i");

    let two_forty = Complex { re: -0.5, im: -0.866 };
    assert_eq!(format!("{}", two_forty),
               "-0.5 - 0.866i");

    let ninety = Complex { re: 0.0, im: 2.0 };
    assert_eq!(format!("{}", ninety),
               "0 + 2i");
    assert_eq!(format!("{:#}", ninety),
               "2 ∠ 90°");
}
