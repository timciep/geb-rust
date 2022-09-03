struct Node<'a> {
    number: i32,
    children: Vec<&'a mut Node<'a>>
}

fn apply_g(x: i32) -> i32 {
    if x == 0 {
        0
    } else {
        let y = x - apply_g(apply_g(x - 1));
        y
    }
}

fn find_node<'a>(node: &'a mut Node<'a>, number: i32) -> Option<&'a mut Node<'a>> {
    if node.number == number {
        return Some(node);
    }

    for childNode in &mut node.children {
        if let Some(matchingNode) = find_node(childNode, number) {
            return Some(matchingNode);
        }
    }

    None
}

fn main() {
    let mut baseNode = Node {
        number: 0,
        children: Vec::new()
    };

    let mut n = 1;
    
    while n < 10 {
        let r = apply_g(n);

        println!("{n}: {r}");

        let mut node = Node {
            number: n,
            children: Vec::new()
        };

        if let Some(parentNode) = find_node(&mut baseNode, n) {
            parentNode.children.push(&mut node);
        }

        n = n + 1;
    }
}
