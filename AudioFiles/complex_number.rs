#[derive(Clone)]
pub(crate) struct ComplexNumber {
    pub(crate) real: f32,
    pub(crate) imaginary: f32,
}

impl ComplexNumber {
    pub fn from_real(real: f32) -> Self {
        Self {
            real,
            imaginary: 0.0,
        }
    }
    pub fn zero() -> Self {
        Self {
            real: 0.0,
            imaginary: 0.0,
        }
    }
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.real.powf(2.0) + self.imaginary.powf(2.0))
    }
    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl std::ops::Add<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl std::ops::Sub<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl std::ops::Mul<&ComplexNumber> for &ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: &ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

impl std::ops::Mul<f32> for &ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: f32) -> ComplexNumber {
        ComplexNumber {
            real: self.real * rhs,
            imaginary: self.imaginary * rhs,
        }
    }
}

impl std::fmt::Debug for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn trunc(num: f32) -> f32 {
            if num.abs() < 1e-6 {
                0.0
            } else {
                (num * 100.0).trunc() / 100.0
            }
        }
        let (re, im) = (trunc(self.real), trunc(self.imaginary));
        if re == 0.0 && im == 0.0 {
            write!(f, "0")
        } else if re == 0.0 {
            write!(f, "{im}i")
        } else if im == 0.0 {
            write!(f, "{re}")
        } else {
            write!(f, "{re} + {im}i")
        }
    }
}
