#![feature(get_type_id)]
#![feature(raw)]
#![no_std]


pub use core::any::Any;
pub use core::any::TypeId;
pub use core::raw::TraitObject;
pub use core::mem;


#[macro_export]
macro_rules! impl_any {
    () => (
        pub fn is<T: $crate::Any>(&self) -> bool {
            let t = $crate::TypeId::of::<T>();
            let boxed = self.get_type_id();
            t == boxed
        }
        pub fn downcast_ref<T: $crate::Any>(&self) -> Option<&T> {
            if self.is::<T>() {
                unsafe {
                    let to: $crate::TraitObject = $crate::mem::transmute(self);
                    Some(&*(to.data as *const T))
                }
            } else {
                None
            }
        }
        pub fn downcast_mut<T: $crate::Any>(&mut self) -> Option<&mut T> {
            if self.is::<T>() {
                unsafe {
                    let to: $crate::TraitObject = $crate::mem::transmute(self);
                    Some(&mut *(to.data as *const T as *mut T))
                }
            } else {
                None
            }
        }
    )
}
