use std::fmt;

#[derive(Clone, Debug, PartialEq, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex{
    fn new<T1, T2>(real : T1, imag : T2) -> Complex
    where T1: Into<f64>,
          T2: Into<f64>
    {
        Complex{
            real : real.into(),
            imag : imag.into(),
        }
    
    }
    fn conjugate(self) -> Complex{
        Complex{
            real : self.real,
            imag : -self.imag,
        }
    }
}

impl From<i32> for Complex{
    fn from(real : i32) -> Complex{
        Complex{
            real : real as f64,
            imag : 0.0,
        }
    }
}

impl From<f64> for Complex{
    fn from(real : f64) -> Complex{
        Complex{
            real : real,
            imag : 0.0,
        }
    }
}

impl std::ops::Add<Complex> for Complex{
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex{
        Complex{
            real : self.real + rhs.real,
            imag : self.imag + rhs.imag,
        }
    }
}

impl std::ops::Sub<Complex> for Complex{
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex{
        Complex{
            real : self.real - rhs.real,
            imag : self.imag - rhs.imag,
        }
    }
}

impl std::ops::Mul<Complex> for Complex{
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex {
        real: self.real * rhs.real - self.imag * rhs.imag,
        imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl std::ops::Neg for Complex{
    type Output = Complex;
    fn neg(self) -> Complex{
        Complex{
            real : -self.real,
            imag : -self.imag,
        }
    }
}

impl fmt::Display for Complex{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        if self.imag== 0.0{
            write!(f, "{}", self.real)
        }
        else if self.real == 0.0{
            write!(f, "{}i", self.imag)
        }

        else if self.imag < 0.0{
            write!(f, "{}-{}i", self.real, -self.imag)
        }
        else{
            write!(f, "{}+{}i", self.real, self.imag)
        
        }
    }
}

impl std::ops::Add<i32> for Complex{
    type Output =  Complex;
    fn add(self, rhs: i32) -> Complex{
        Complex{
            real : self.real + rhs as f64,
            imag : self.imag,
        }
    }
}

impl std::ops::Add<f64> for Complex{
    type Output =  Complex;
    fn add(self, rhs: f64) -> Complex{
        Complex{
            real : self.real + rhs,
            imag : self.imag,
        }
    }
}

impl std::ops::Mul<i32> for Complex{
    type Output = Complex;
    fn mul(self, rhs: i32) -> Complex{
        Complex{
            real : self.real * rhs as f64,
            imag : self.imag * rhs as f64,
        }
    }
}
// Mul<i32> and Add<f64> are not mandatory for the main to run
// could also add Sub<i32> and Sub<f64>

impl std::ops::Mul<f64> for Complex{
    type Output = Complex;
    fn mul(self, rhs: f64) -> Complex{
        Complex{
            real : self.real * rhs,
            imag : self.imag * rhs,
        }
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}