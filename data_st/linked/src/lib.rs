pub fn welcome(){
    println!("Linked list call");
}

pub struct Linked {
    pub data: i32,
    next: Option<Box<Linked>>,
}
