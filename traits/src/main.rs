mod api;
mod cm;
mod mm;
use crate::cm::Cm;
use crate::mm::Mm;

fn main() {
    let len = Mm::new(12);
    print!("{}", len)
}

#[test]
fn add() {
    let m = Mm::new(23);
    let c = Cm::new(1);
    assert_eq!(m + c, Mm::new(33));
}
