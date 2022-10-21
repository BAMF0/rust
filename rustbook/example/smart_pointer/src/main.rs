use smart_pointer::cons::List::{Cons, Nil};
use smart_pointer::deref::MyBox;
use smart_pointer::drop::CustomSmartPointer;
use std::rc::Rc;

fn main() {
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("List using cons: {:?}", list);

    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // variables c and d are dropped during runtime
    // in reverse
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // TODO RefCell<T> and the Interior Mutability Pattern
}
