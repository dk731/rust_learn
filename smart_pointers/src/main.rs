//! RefCell ???

use std::fmt::Display;
use std::mem::drop;
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct TestStruct {
    x: i32,
    b: f64,
    c: String,
}

impl TestStruct {
    fn new() -> TestStruct {
        TestStruct {
            x: 1,
            b: 1.1,
            c: "qwe".to_string(),
        }
    }
}

impl Display for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.b, self.c)
    }
}

#[derive(Debug)]
struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping pointer with data: {}", self.0)
    }
}

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);
    let c = MyBox::new(TestStruct::new());

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("'c' value is not dropped: {:?}", c);
    drop(c);
    println!("'c' value has been dropped");

    let a = Rc::new(List2::Cons(
        1,
        Rc::new(List2::Cons(2, Rc::new(List2::Cons(3, Rc::new(List2::Nil))))),
    ));
    let b = List2::Cons(-1, Rc::clone(&a));
    let c = List2::Cons(0, Rc::clone(&a));
}
