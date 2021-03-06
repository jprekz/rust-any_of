use std::any::TypeId;
use std::ops::{Deref, DerefMut};
use std::fmt;

pub struct AnyOf<T: 'static + ?Sized> {
    type_id: TypeId,
    value: T,
}
impl<T: 'static> AnyOf<T> {
    pub fn new(value: T) -> AnyOf<T> {
        AnyOf {
            type_id: TypeId::of::<T>(),
            value: value,
        }
    }
}
impl<T: 'static + ?Sized> AnyOf<T> {
    pub fn is<U: 'static>(&self) -> bool {
        let t = TypeId::of::<U>();
        let boxed = self.type_id;
        t == boxed
    }
    pub fn downcast_ref<U: 'static>(&self) -> Option<&U> {
        if self.is::<U>() {
            Some(unsafe { &*(&self.value as *const T as *const U) })
        } else {
            None
        }
    }
    pub fn downcast_mut<U: 'static>(&mut self) -> Option<&mut U> {
        if self.is::<U>() {
            Some(unsafe { &mut *(&mut self.value as *mut T as *mut U) })
        } else {
            None
        }
    }
}

impl<T: 'static + ?Sized> Deref for AnyOf<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}
impl<T: 'static + ?Sized> DerefMut for AnyOf<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}
impl<T: 'static + ?Sized + fmt::Debug> fmt::Debug for AnyOf<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AnyOf({:?})", &self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait MyTrait: fmt::Debug {
        fn get(&self) -> i32;
        fn set(&mut self, v: i32);
    }
    #[derive(Debug, PartialEq, Clone)]
    struct MyStruct1(i32);
    impl MyTrait for MyStruct1 {
        fn get(&self) -> i32 {
            self.0
        }
        fn set(&mut self, v: i32) {
            self.0 = v
        }
    }
    #[derive(Debug, PartialEq)]
    struct MyStruct2(i32);
    impl MyTrait for MyStruct2 {
        fn get(&self) -> i32 {
            self.0 * 2
        }
        fn set(&mut self, v: i32) {
            self.0 = v
        }
    }

    #[test]
    fn test_ref() {
        let anyof: &AnyOf<MyTrait> = &AnyOf::new(MyStruct1(1i32));
        assert_eq!(anyof.get(), 1i32);
        assert_eq!(anyof.downcast_ref::<MyStruct1>(), Some(&MyStruct1(1i32)));
        assert_eq!(anyof.downcast_ref::<MyStruct2>(), None);
        let anyof: &AnyOf<MyTrait> = &AnyOf::new(MyStruct2(1i32));
        assert_eq!(anyof.get(), 2i32);
        assert_eq!(anyof.downcast_ref::<MyStruct1>(), None);
        assert_eq!(anyof.downcast_ref::<MyStruct2>(), Some(&MyStruct2(1i32)));
    }

    #[test]
    fn test_box() {
        let anyof: Box<AnyOf<MyTrait>> = Box::new(AnyOf::new(MyStruct1(1i32)));
        assert_eq!(anyof.get(), 1i32);
        assert_eq!(anyof.downcast_ref::<MyStruct1>(), Some(&MyStruct1(1i32)));
        assert_eq!(anyof.downcast_ref::<MyStruct2>(), None);
        let anyof: Box<AnyOf<MyTrait>> = Box::new(AnyOf::new(MyStruct2(1i32)));
        assert_eq!(anyof.get(), 2i32);
        assert_eq!(anyof.downcast_ref::<MyStruct1>(), None);
        assert_eq!(anyof.downcast_ref::<MyStruct2>(), Some(&MyStruct2(1i32)));
    }

    #[test]
    fn test_ref_mut() {
        let anyof: &mut AnyOf<MyTrait> = &mut AnyOf::new(MyStruct1(0i32));
        anyof.set(1i32);
        assert_eq!(anyof.get(), 1i32);
        assert_eq!(anyof.downcast_ref::<MyStruct1>(), Some(&MyStruct1(1i32)));
        assert_eq!(anyof.downcast_ref::<MyStruct2>(), None);
        assert_eq!(anyof.downcast_mut::<MyStruct1>(), Some(&mut MyStruct1(1i32)));
        assert_eq!(anyof.downcast_mut::<MyStruct2>(), None);
        let anyof: &mut AnyOf<MyTrait> = &mut AnyOf::new(MyStruct2(0i32));
        anyof.set(1i32);
        assert_eq!(anyof.get(), 2i32);
        assert_eq!(anyof.downcast_ref::<MyStruct1>(), None);
        assert_eq!(anyof.downcast_ref::<MyStruct2>(), Some(&MyStruct2(1i32)));
        assert_eq!(anyof.downcast_mut::<MyStruct1>(), None);
        assert_eq!(anyof.downcast_mut::<MyStruct2>(), Some(&mut MyStruct2(1i32)));
    }

    #[test]
    fn test_box_mut() {
        let mut anyof: Box<AnyOf<MyTrait>> = Box::new(AnyOf::new(MyStruct1(0i32)));
        anyof.set(1i32);
        assert_eq!(anyof.get(), 1i32);
        assert_eq!(anyof.downcast_ref::<MyStruct1>(), Some(&MyStruct1(1i32)));
        assert_eq!(anyof.downcast_ref::<MyStruct2>(), None);
        assert_eq!(anyof.downcast_mut::<MyStruct1>(), Some(&mut MyStruct1(1i32)));
        assert_eq!(anyof.downcast_mut::<MyStruct2>(), None);
        let mut anyof: Box<AnyOf<MyTrait>> = Box::new(AnyOf::new(MyStruct2(0i32)));
        anyof.set(1i32);
        assert_eq!(anyof.get(), 2i32);
        assert_eq!(anyof.downcast_ref::<MyStruct1>(), None);
        assert_eq!(anyof.downcast_ref::<MyStruct2>(), Some(&MyStruct2(1i32)));
        assert_eq!(anyof.downcast_mut::<MyStruct1>(), None);
        assert_eq!(anyof.downcast_mut::<MyStruct2>(), Some(&mut MyStruct2(1i32)));
    }
}
