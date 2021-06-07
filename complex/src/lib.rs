//! Complex number examples.
//!
//! The chapter presents several different variations on how one might define
//! arithmetic on a generic `Complex` type, so what we have here are a bunch of
//! isolated modules, each of which defines its own `Complex` type in its own
//! way. The `first_cut` module is the most well-developed.
//!
//! If you actually need a `Complex` type for real use, consider the
//! `num_complex` crate, whose `Complex` type is incorporated into the `num`
//! crate.

macro_rules! define_complex {
    () => {
        #[derive(Clone, Copy, Debug)]
        struct Complex<T> {
            /// Real portion of the complex number
            re: T,

            /// Imaginary portion of the complex number
            im: T,
        }
    };
}

mod first_cut {
    #[derive(Clone, Copy, Debug)]
    struct Complex<T> {
        /// Real portion of the complex number
        re: T,

        /// Imaginary portion of the complex number
        im: T,
    }

    use std::ops::Add;

    impl<T> Add for Complex<T>
    where
        T: Add<Output = T>,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }

    use std::ops::Sub;

    impl<T> Sub for Complex<T>
    where
        T: Sub<Output = T>,
    {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            Complex {
                re: self.re - rhs.re,
                im: self.im - rhs.im,
            }
        }
    }

    use std::ops::Mul;

    impl<T> Mul for Complex<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            Complex {
                re: self.re.clone() * rhs.re.clone()
                    - (self.im.clone() * rhs.im.clone()),
                im: self.im * rhs.re + self.re * rhs.im,
            }
        }
    }

    #[test]
    fn try_it_out() {
        let mut z = Complex { re: 1, im: 2 };
        let c = Complex { re: 3, im: 4 };

        z = z * z + c;

        std::mem::forget(z);
    }

    impl<T: PartialEq> PartialEq for Complex<T> {
        fn eq(&self, other: &Complex<T>) -> bool {
            self.re == other.re && self.im == other.im
        }
    }

    #[test]
    fn test_complex_eq() {
        let x = Complex { re: 5, im: 2 };
        let y = Complex { re: 2, im: 5 };
        assert_eq!(x * y, Complex { re: 0, im: 29 });
    }

    impl<T: Eq> Eq for Complex<T> {}
}

mod non_generic_add {
    define_complex!();

    use std::ops::Add;

    impl Add for Complex<i32> {
        type Output = Complex<i32>;
        fn add(self, rhs: Self) -> Self {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }
}

mod somewhat_generic {
    define_complex!();

    use std::ops::Add;

    impl<T> Add for Complex<T>
    where
        T: Add<Output = T>,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }

    use std::ops::Neg;

    impl<T> Neg for Complex<T>
    where
        T: Neg<Output = T>,
    {
        type Output = Complex<T>;
        fn neg(self) -> Complex<T> {
            Complex {
                re: -self.re,
                im: -self.im,
            }
        }
    }
}

mod very_generic {
    define_complex!();

    use std::ops::Add;

    impl<L, R> Add<Complex<R>> for Complex<L>
    where
        L: Add<R>,
    {
        type Output = Complex<L::Output>;
        fn add(self, rhs: Complex<R>) -> Self::Output {
            Complex {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }
}

mod impl_compound {
    define_complex!();

    use std::ops::AddAssign;

    impl<T> AddAssign for Complex<T>
    where
        T: AddAssign<T>,
    {
        fn add_assign(&mut self, rhs: Complex<T>) {
            self.re += rhs.re;
            self.im += rhs.im;
        }
    }
}

mod derive_partialeq {
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Complex<T> {
        re: T,
        im: T,
    }
}

mod derive_everything {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct Complex<T> {
        /// Real portion of the complex number
        re: T,

        /// Imaginary portion of the complex number
        im: T,
    }
}

/// Examples from Chapter 17, Strings and Text
///
/// These use a separate, non-generic `Complex` type, for simplicity.
mod formatting {
    #[test]
    fn complex() {
        #[derive(Copy, Clone, Debug)]
        struct Complex { re: f64, im: f64 }

        let third = Complex { re: -0.5, im: f64::sqrt(0.75) };
        println!("{:?}", third);

        use std::fmt;

        impl fmt::Display for Complex {
            fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
                let im_sign = if self.im < 0.0 { '-' } else { '+' };
                write!(dest, "{} {} {}i", self.re, im_sign, f64::abs(self.im))
            }
        }

        let one_twenty = Complex { re: -0.5, im: 0.866 };
        assert_eq!(format!("{}", one_twenty),
                   "-0.5 + 0.866i");

        let two_forty = Complex { re: -0.5, im: -0.866 };
        assert_eq!(format!("{}", two_forty),
                   "-0.5 - 0.866i");
    }

    #[test]
    fn complex_fancy() {
        #[derive(Copy, Clone, Debug)]
        struct Complex { re: f64, im: f64 }

        use std::fmt;

        impl fmt::Display for Complex {
            fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
                let (re, im) = (self.re, self.im);
                if dest.alternate() {
                    let abs = f64::sqrt(re * re + im * im);
                    let angle = f64::atan2(im, re) / std::f64::consts::PI * 180.0;
                    write!(dest, "{} ∠ {}°", abs, angle)
                } else {
                    let im_sign = if im < 0.0 { '-' } else { '+' };
                    write!(dest, "{} {} {}i", re, im_sign, f64::abs(im))
                }
            }
        }

        let ninety = Complex { re: 0.0, im: 2.0 };
        assert_eq!(format!("{}", ninety),
                   "0 + 2i");
        assert_eq!(format!("{:#}", ninety),
                   "2 ∠ 90°");
    }
}
