use std::ops::Deref;
struct MyBox<T>(T);

impl <T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/*
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn hello(name: &str) {
    println!("hello {}!", name);
}
 */

fn main() {
    let c =CustomSmartPointer {data: String::from("my stuff")};
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
fn drop(&mut self) {
    println!("Droping CustomSmartPointer with {}", self.data);
}
}
