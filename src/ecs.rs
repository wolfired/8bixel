pub trait Entity {}

pub trait Component {}

pub trait System {
    fn exec(&self);
}

pub struct ASystem {}

impl System for ASystem {
    fn exec(&self) {
        
    }
}
