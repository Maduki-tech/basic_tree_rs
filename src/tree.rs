#[derive(Debug)]
pub struct Node{
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}



impl Node{
    pub fn new(value: i32) -> Self{
        Node{
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: i32){
        if value < self.value{
            match self.left{
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(Node::new(value))),
            }
        }else{
            match self.right{
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    pub fn print_in_order(&self){
        match self.left{
            Some(ref left) => left.print_in_order(),
            None => {},
        }
        println!("{}", self.value);
        match self.right{
            Some(ref right) => right.print_in_order(),
            None => {},
        }
    }
}
