mod tree;

fn main() {
    let mut root = tree::Node::new(5);
    root.insert(3);
    root.insert(7);
    root.insert(2);
    root.insert(4);
    root.insert(6);
    root.insert(8);
    root.print_in_order();

    println!("{:#?}", root);


}
