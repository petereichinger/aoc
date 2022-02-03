use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
enum Node {
    Leaf(u32),
    Inner(Box<Node>, Box<Node>),
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Leaf(value) => { write!(f, "{}", value) }
            Node::Inner(l, r) => { write!(f, "[{},{}]", l, r) }
        }
    }
}

impl Node {
    fn explode(&self) {
        let mut stack : Vec<&Node>;

        stack.push(&self);


    }

    // fn split(&self) -> Option<Self> {
    //     None
    // }
    //
    // fn reduce(self) -> Self {
    //     loop {}
    //
    //     self
    // }
}

impl std::ops::Add for Box<Node> {
    type Output = Box<Node>;

    fn add(self, rhs: Self) -> Self::Output {
        Box::new(Node::Inner(self, rhs))
    }
}

fn main() {}


#[cfg(test)]
mod tests {
    use crate::Node;
    use std::fmt::{Error, Write};

    #[test]
    fn test_add() {
        let left = Box::new(Node::Leaf(1));
        let right = Box::new(Node::Leaf(2));

        let add = left + right;

        assert_eq!(add, Box::new(Node::Inner(Box::new(Node::Leaf(1)), Box::new(Node::Leaf(2)))));
    }

    #[test]
    fn test_display() -> Result<(), Box<dyn std::error::Error>> {
        let left = Box::new(Node::Leaf(1));
        let right = Box::new(Node::Leaf(2));

        let add = left + right;

        let mut test = String::new();

        write!(&mut test, "{}", add);

        assert_eq!(test, "[1,2]");
        Ok(())
    }

    #[test]
    fn test_display_2() -> Result<(), Box<dyn std::error::Error>> {
        let add = Box::new(Node::Leaf(1)) + Box::new(Node::Leaf(2)) + Box::new(Node::Leaf(3));

        let mut test = String::new();

        write!(&mut test, "{}", add)?;

        assert_eq!(test, "[[1,2],3]");
        Ok(())
    }

    #[test]
    fn test_display_3() -> Result<(), Box<dyn std::error::Error>> {
        let add = (Box::new(Node::Leaf(1)) + Box::new(Node::Leaf(4))) + (Box::new(Node::Leaf(2)) + Box::new(Node::Leaf(3)));

        let mut test = String::new();

        write!(&mut test, "{}", add)?;

        assert_eq!(test, "[[1,4],[2,3]]");
        Ok(())
    }
}