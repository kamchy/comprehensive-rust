pub trait ExpressableInMm {
    fn mms(&self) -> u32;
}

pub trait Unit {
    fn name(&self) -> &'static str;
    fn value(&self) -> u32;
}
