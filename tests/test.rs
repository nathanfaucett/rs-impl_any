#![feature(get_type_id)]
#![feature(raw)]
#![no_std]


#[macro_use]
extern crate impl_any;


use core::any::Any;


trait Value: Any {
    fn get(&self) -> Any;
    fn set(&self, value: &Any);
}
impl Value {
    impl_any!();
}
