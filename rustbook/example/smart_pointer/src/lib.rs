mod mock_objects;
pub mod ref_cycle;

pub use ref_cycle::node;

pub mod cons {
    use::std::rc::Rc;

    #[derive(Debug)]
    pub enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
}

// Rust can deref coercion:
// From &T to &U when T: Deref<Target=U>
// From &mut T to &mut U when T: DerefMut<Target=U>
// NOTE cannot make immutable reference mutable
pub mod deref {
    use std::ops::Deref;

    pub struct MyBox<T>(T);

    impl<T> MyBox<T> {
        pub fn new(x: T) -> MyBox<T> {
            MyBox(x)
        } 
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
        
    }
}

pub mod drop {
   pub struct CustomSmartPointer {
       pub data: String,
   }  

   impl Drop for CustomSmartPointer {
       fn drop(&mut self) {
           println!("Dropping CustomSmartPointer with data `{}`!", self.data);
       }
   }
}

