

use smart_pointer::cons::List::{Cons, Nil};
use smart_pointer::deref::MyBox;
use smart_pointer::drop::CustomSmartPointer;
use crate::RefList::{Consr, Nilr};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum RefList {
    Consr(Rc<RefCell<i32>>, Rc<RefList>),
    Nilr,
}


fn main() {
    list_main();
    custom_pointer_main();
    rc_main();
    rc_and_refcell_main();
}

fn list_main() {
    let list = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    println!("List using cons: {:?}", list);

    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn custom_pointer_main() {
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
}

fn rc_main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn rc_and_refcell_main() {
    // Combining Rc and RefCell
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Consr(Rc::clone(&value), Rc::new(Nilr)));


    let b = Consr(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Consr(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    print!("a after = {:?}", a);
    print!("b after = {:?}", b);
    print!("c after = {:?}", c);
}
