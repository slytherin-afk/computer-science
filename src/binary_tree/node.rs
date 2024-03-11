#[derive(PartialEq, Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn from(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: PartialOrd> Node<T> {
    pub fn insert(&mut self, value: T) {
        let desired_node = if value <= self.value {
            &mut self.left
        } else {
            &mut self.right
        };

        match desired_node {
            Some(node) => node.insert(value),
            None => *desired_node = Some(Box::new(Self::from(value))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Node;

    #[test]
    fn insert_value() {
        let mut value = Node::from(100);

        //              100
        //             /   \
        //           50     200
        //          / \     /  \
        //        25  75   150  300

        value.insert(50);
        value.insert(200);
        value.insert(25);
        value.insert(75);
        value.insert(150);
        value.insert(300);

        let expected = Node {
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
        };

        assert_eq!(value, expected);
    }
}
