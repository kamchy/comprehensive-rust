use crate::api::{ExpressableInMm, Unit};
use std::fmt;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cm {
    value: u32,
}

impl Cm {
    pub fn new(v: u32) -> Cm {
        Cm { value: v }
    }
}

impl fmt::Display for Cm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value(), self.name())
    }
}
impl Unit for Cm {
    fn name(&self) -> &'static str {
        "centimeters"
    }
    fn value(&self) -> u32 {
        self.value
    }
}

impl ExpressableInMm for Cm {
    fn mms(&self) -> u32 {
        10
    }
}

#[test]
fn test_name() {
    let cms = 123;
    let v = Cm::new(cms);
    assert_eq!(v.name(), "centimeters")
}

#[test]
fn test_value() {
    let cms = 123;
    let v = Cm::new(cms);
    assert_eq!(v.value(), 123);
}
#[test]
fn test_mms() {
    let v = Cm::new(23);
    assert_eq!(v.mms(), 10);
}
#[test]
fn test_display() {
    let v = Cm::new(23);
    assert_eq!("23 centimeters", v.to_string())
}
