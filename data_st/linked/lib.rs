pub fn welcome(){
    println!("Linked list call");
}

#[derive(Clone,Debug)]
struct LinkedNode {
    data: i32,
    next: Option<Box<LinkedNode>>
}


#[derive(Clone,Debug)]
pub struct Linked {
    head : Option<LinkedNode>
}


impl LinkedNode {
    fn new(dt : i32) -> Self {
        LinkedNode {
            data: dt,
            next: None
        }
    }

    fn get_next(&mut self) -> Option<LinkedNode> {
        if self.next.is_some() {
            let nx = self.next.clone();
            return Some(*nx.unwrap());
        }
        None
    }
}


impl Linked {
    pub fn new() -> Self {
        Linked {
            head: None
        }
    }
        
    pub fn add(&mut self, data: i32) {
        let mut new_node = LinkedNode::new(data);
        if self.head.is_some() {
            let h = self.head.clone();
            new_node.next = Some(Box::new(h.unwrap()));
        } else {
            new_node.next = None;
        } 
        self.head = Some(new_node);

    }

    pub fn remove(&mut self) {
        if self.head.is_some() {
            let h = self.head.clone();
            self.head = h.unwrap().get_next();
        }

    }

}
