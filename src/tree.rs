type NodeBox = Option<Box<Node>>;

struct Node {
    payload: String,
    left: NodeBox,
    right: NodeBox,
}

impl Node {
    fn new(s: &str) -> Node {
        Node {
            payload: s.to_string(),
            left: None,
            right: None,
        }
    }
}

// pub fn initialize_tree() {
//     let s = Node::new("root");

//     s.left = Node::new("left");
//     s.right = Node::new("right");

//     print!("{:#?}", s);
// }
