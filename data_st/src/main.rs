use linked;
use linked::Linked;

fn main() {
    println!("Hello, world!");
    linked::welcome();
    let mut eg = Linked::new();
    println!("{:?}",eg);
    eg.add(3);
    println!("{:?}",eg);
    eg.add(9);
    println!("{:?}",eg);
    eg.add(27);
    println!("{:?}",eg);
    eg.remove();
    println!("{:?}",eg);
    eg.remove();
    eg.remove();
    eg.remove();
    println!("{:?}",eg);
    
}
