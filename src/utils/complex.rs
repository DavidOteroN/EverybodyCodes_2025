use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
pub use std::str::FromStr;

use regex::Regex;

/// # Complex
/// A signed integer (`i64`) complex type with overloaded operators.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex(i64, i64);

impl Complex {
    pub fn new(real: i64, imag: i64) -> Self {
        Self(real, imag)
    }

    /// Compute the squared norm.
    pub fn square_norm(&self) -> i64 {
        let (real, imag) = (self.0, self.1);
        real * real + imag * imag
    }

    /// Returns the real part.
    pub fn real(&self) -> i64 {
        self.0
    }

    /// Returns the imaginary part.
    pub fn imag(&self) -> i64 {
        self.1
    }

    /// Returns the complex conjugate.
    /// ### Examples:
    /// ```rust
    /// let real: i64 = 1;
    /// let imag: i64 = 2;
    /// let z = Complex(real, imag);
    /// let c = z.conjugate();
    ///
    /// assert_eq!(c.real(), real);
    /// assert_eq!(c.imag(), -imag);
    /// assert_eq!(a + c, Complex(2 * real, 0));
    /// ```
    fn conjugate(&self) -> Self {
        Complex::new(self.real(), -self.imag())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseComplexError;

impl FromStr for Complex {
    type Err = ParseComplexError;

    /// Create a Complex object from a string reference of the form `real +|- imaginary i`, or
    /// `[real, imaginary]`.
    /// ### Examples:
    /// ```rust
    /// let z = Complex::from_str("-4 + 3i").unwrap();
    /// assert_eq!(z, Complex::new(-4, 3));
    ///
    /// let z2 = Complex::from_str("[10, -2]").unwrap();
    /// assert_eq!(z2, Complex::new(10, -2));
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // TODO: This Regular expression can be improved to only look for a ',' and a closing ']' if the initial '[' is pressent.
        let re = Regex::new(r"\[?\s*(?<real>[+-]?\d+)\s*\,?(?<imag>[+-]?\d+)\]?").unwrap();
        if let Some(caps) = re.captures(input) {
            let real = caps["real"].parse::<i64>().map_err(|_| ParseComplexError)?;
            let imag = caps["imag"].parse::<i64>().map_err(|_| ParseComplexError)?;
            Ok(Complex::new(real, imag))
        } else {
            Err(ParseComplexError)
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}{:+}i", self.real(), self.imag())
    }
}

impl Add<Complex> for Complex {
    type Output = Self;
    fn add(self, rhs: Complex) -> Self::Output {
        Self::new(self.real() + rhs.real(), self.imag() + rhs.imag())
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.real(), -self.imag())
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let a = self.real();
        let b = self.imag();
        let c = rhs.real();
        let d = rhs.imag();

        Complex::new(a * c - b * d, a * d + b * c)
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let conj = rhs.conjugate();
        let sq_norm = conj.square_norm();
        let temp = self * conj;
        let real = temp.real() / sq_norm;
        let imag = temp.imag() / sq_norm;

        Complex::new(real, imag)
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
