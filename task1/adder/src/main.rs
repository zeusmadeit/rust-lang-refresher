use std::ops::Deref;

use::add_one;
// use std::fmt::Display;

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let num = 10;
    println!("Hello, world! {} + 1 is = {}", num, add_one::add_one(num));
    let bx = Box::new(5);
    println!("b is {}", bx);
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Reference Counted Smart Pointer
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
