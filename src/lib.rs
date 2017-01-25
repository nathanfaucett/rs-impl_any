#![feature(get_type_id)]
#![feature(alloc)]
#![no_std]


extern crate alloc;


#[doc(hidden)]
pub mod __ {
    pub use alloc::boxed::Box;

    pub use core::any::Any;
    pub use core::any::TypeId;
    pub use core::option::Option;
    pub use core::result::Result;
}


#[macro_export]
macro_rules! impl_any {
    ($trait_:ident) => {

        impl_any!($trait_, only core);

        #[allow(dead_code)]
        impl $trait_ {
            #[inline]
            pub fn downcast<T: $trait_>(self: $crate::__::Box<Self>)
                -> $crate::__::Result<$crate::__::Box<T>, $crate::__::Box<Self>>
            {
                if self.is::<T>() {
                    unsafe {
                        $crate::__::Result::Ok(self.downcast_unchecked())
                    }
                } else {
                    $crate::__::Result::Err(self)
                }
            }
            #[inline]
            pub unsafe fn downcast_unchecked<T: $trait_>(self: $crate::__::Box<Self>)
                -> $crate::__::Box<T>
            {
                $crate::__::Box::from_raw($crate::__::Box::into_raw(self) as *mut T)
            }
        }
    };
    ($trait_:ident, only core) => {
        #[allow(dead_code)]
        impl $trait_ {
            #[inline]
            pub fn is<T: $trait_>(&self) -> bool {
                $crate::__::TypeId::of::<T>() == $crate::__::Any::get_type_id(self)
            }
            #[inline]
            pub fn downcast_ref<T: $trait_>(&self) -> $crate::__::Option<&T> {
                if self.is::<T>() {
                    unsafe {
                        $crate::__::Option::Some(self.downcast_ref_unchecked())
                    }
                } else {
                    $crate::__::Option::None
                }
            }
            #[inline]
            pub unsafe fn downcast_ref_unchecked<T: $trait_>(&self) -> &T {
                &*(self as *const Self as *const T)
            }
            #[inline]
            pub fn downcast_mut<T: $trait_>(&mut self) -> $crate::__::Option<&mut T> {
                if self.is::<T>() {
                    unsafe {
                        $crate::__::Option::Some(self.downcast_mut_unchecked())
                    }
                } else {
                    $crate::__::Option::None
                }
            }
            #[inline]
            pub unsafe fn downcast_mut_unchecked<T: $trait_>(&mut self) -> &mut T {
                &mut *(self as *mut Self as *mut T)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::__::*;


    trait Thing: Any {
        fn does(&self) -> usize;
    }

    impl_any!(Thing);

    #[derive(Clone, Debug, PartialEq)]
    struct Foo {
        value: usize,
    }

    impl Thing for Foo {
        fn does(&self) -> usize {
            self.value + 5
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct Bar;

    impl Thing for Bar {
        fn does(&self) -> usize {
            5
        }
    }

    #[test]
    fn test_ref() {
        let benny = Foo { value: 5 };
        let benny_ptr: *const Foo = &benny;
        let person: &Thing = &benny;

        assert!(person.is::<Foo>());
        assert_eq!(person.downcast_ref::<Foo>().map(|x| x as *const Foo), Some(benny_ptr));
        assert_eq!(unsafe { person.downcast_ref_unchecked::<Foo>() as *const Foo }, benny_ptr);

        assert!(!person.is::<Bar>());
        assert_eq!(person.downcast_ref::<Bar>(), None);
    }

    #[test]
    fn test_mut() {
        let mut benny = Foo { value: 5 };
        let benny_ptr: *const Foo = &benny;
        let person: &mut Thing = &mut benny;
        assert!(person.is::<Foo>());
        assert_eq!(person.downcast_ref::<Foo>().map(|x| x as *const Foo), Some(benny_ptr));
        assert_eq!(person.downcast_mut::<Foo>().map(|x| &*x as *const Foo), Some(benny_ptr));
        assert_eq!(unsafe { person.downcast_ref_unchecked::<Foo>() as *const Foo }, benny_ptr);
        assert_eq!(unsafe { &*person.downcast_mut_unchecked::<Foo>() as *const Foo }, benny_ptr);

        assert!(!person.is::<Bar>());
        assert_eq!(person.downcast_ref::<Bar>(), None);
        assert_eq!(person.downcast_mut::<Bar>(), None);
    }

    #[test]
    fn test_box() {
        let mut benny = Foo { value: 5 };
        let mut person: Box<Thing> = Box::new(benny.clone());
        assert!(person.is::<Foo>());
        assert_eq!(person.downcast_ref::<Foo>(), Some(&benny));
        assert_eq!(person.downcast_mut::<Foo>(), Some(&mut benny));
        assert_eq!(person.downcast::<Foo>().map(|x| *x).ok(), Some(benny.clone()));

        person = Box::new(benny.clone());
        assert_eq!(unsafe { person.downcast_ref_unchecked::<Foo>() }, &benny);
        assert_eq!(unsafe { person.downcast_mut_unchecked::<Foo>() }, &mut benny);
        assert_eq!(unsafe { *person.downcast_unchecked::<Foo>() }, benny);

        person = Box::new(benny.clone());
        assert!(!person.is::<Bar>());
        assert_eq!(person.downcast_ref::<Bar>(), None);
        assert_eq!(person.downcast_mut::<Bar>(), None);
        assert!(person.downcast::<Bar>().err().is_some());
    }
}
