use crate::api::{ExpressableInMm, Unit};
use crate::Cm;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Mm {
    value: u32,
}

impl Mm {
    pub fn new(v: u32) -> Mm {
        Mm { value: v }
    }
}

impl fmt::Display for Mm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value(), self.name())
    }
}

impl Unit for Mm {
    fn name(&self) -> &'static str {
        "millimeters"
    }

    fn value(&self) -> u32 {
        self.value
    }
}

impl ExpressableInMm for Mm {
    fn mms(&self) -> u32 {
        1
    }
}

impl<T: Into<Mm>> Add<T> for Mm {
    type Output = Mm;
    fn add(self, other: T) -> Self {
        let o: Self = other.into();
        Mm::new(self.value + o.value())
    }
}
impl std::iter::Sum for Mm {
    fn sum<I: Iterator<Item = Mm>>(iter: I) -> Self {
        iter.fold(Mm::new(0), |acc, x| acc + x)
    }
}
impl From<Cm> for Mm {
    fn from(value: Cm) -> Self {
        Mm::new(value.value() * value.mms())
    }
}

#[test]
fn test_mm_name() {
    let v = Mm { value: 30 };
    assert_eq!(v.name(), "millimeters");
}

#[test]
fn test_mm_val() {
    let v = Mm { value: 30 };
    assert_eq!(v.value(), 30);
}

#[test]
fn test_vec() {
    let v: Vec<_> = vec![Mm::new(12), Cm::new(2).into(), 34.into()];
    assert_eq!(v.iter().cloned().sum::<Mm>(), Mm::new(66));
}

#[test]
fn test_display() {
    let v = Mm { value: 30 };
    assert_eq!("30 millimeters", v.to_string())
}

#[test]
fn simple_add() {
    let mm1 = Mm::new(1);
    let mm3 = Mm::new(3);
    assert_eq!(mm1 + mm3, Mm::new(4));
}

#[test]
fn simple_add_cm() {
    let mm1 = Mm::new(1);
    let cm3 = Cm::new(3);
    assert_eq!(mm1 + cm3, Mm::new(31));
}

#[test]
fn test_simple_add_mm_get_value() {
    let v = Mm { value: 30 };
    let z = Mm { value: 7 };
    assert_eq!(v.add(z).value(), 37);
}
#[test]
fn simple_add_u32() {
    impl From<u32> for Mm {
        fn from(v: u32) -> Mm {
            Mm::new(v)
        }
    }
    let mm1 = Mm::new(1);
    let v = 234;
    assert_eq!(mm1 + v, Mm::new(235));
}

#[test]
fn test_into() {
    let cm10 = Cm::new(10);
    let mm: Mm = cm10.into();
    assert_eq!(mm.value(), 100);
}
