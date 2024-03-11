use node::Node;

pub mod node;

#[derive(PartialEq, Debug)]
pub enum BinaryTree<T> {
    Root(Box<Node<T>>),
    Nill,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree::Nill
    }
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn insert(&mut self, value: T) {
        match self {
            Self::Nill => *self = Self::Root(Box::new(Node::from(value))),
            Self::Root(root) => {
                root.insert(value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{BinaryTree, Node};

    #[test]
    fn insert_value() {
        let mut value = BinaryTree::new();

        //     BinaryTree -> Root -> Box -> |
        //                                  |
        //                                 100
        //                                /   \
        //                              50     200
        //                             / \     /  \
        //                           25  75   150  300

        value.insert(100);
        value.insert(50);
        value.insert(200);
        value.insert(25);
        value.insert(75);
        value.insert(150);
        value.insert(300);

        let expected = BinaryTree::Root(Box::new(Node {
            value: 100,
            left: Some(Box::new(Node {
                value: 50,
                left: Some(Box::new(Node {
                    value: 25,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 75,
                    left: None,
                    right: None,
                })),
            })),
            right: Some(Box::new(Node {
                value: 200,
                left: Some(Box::new(Node {
                    value: 150,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 300,
                    left: None,
                    right: None,
                })),
            })),
        }));

        assert_eq!(value, expected);
    }
}
