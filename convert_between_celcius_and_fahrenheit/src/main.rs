use crate::Scale::{Celcius, Fahrenheit};

#[derive(PartialEq, Copy, Clone)]
enum Scale {
    Celcius,
    Fahrenheit,
}

impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Scale::Celcius => write!(f, "°C"),
            Scale::Fahrenheit => write!(f, "°F"),
        }
    }
}

#[derive(Copy, Clone)]
struct Temperature {
    degrees: f32,
    scale: Scale,
}

impl Temperature {
    fn to_celcius(&mut self) -> Self {
        if self.scale != Celcius {
            self.degrees = (self.degrees - 32.0) * (5.0 / 9.0);
            self.scale = Scale::Celcius;
            *self
        } else {
            *self
        }
    }

    fn to_fahrenheit(&mut self) -> Self {
        if self.scale != Fahrenheit {
            self.degrees = (self.degrees * (9.0 / 5.0)) + 32.0;
            self.scale = Scale::Fahrenheit;
            *self
        } else {
            *self
        }
    }
}

fn main() {
    let mut t1 = Temperature {
        degrees: 45.0,
        scale: Celcius,
    };

    println!("t1 is : {} {}", t1.degrees, t1.scale);

    t1.to_fahrenheit();

    println!("t1 in Fahrenheit is : {} {}", t1.degrees, t1.scale);

    let mut t2 = Temperature {
        degrees: 70.0,
        scale: Fahrenheit,
    };

    println!("t2 is : {} {}", t2.degrees, t2.scale);

    t2.to_celcius();

    println!("t2 in Celcius is : {} {}", t2.degrees, t2.scale);
}
