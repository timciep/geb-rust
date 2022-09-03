struct Node {
    number: i32,
    children: Vec<Node>
}

fn apply_g(x: i32) -> i32 {
    if x == 0 {
        0
    } else {
        let y = x - apply_g(apply_g(x - 1));
        y
    }
}

fn add_subnode_to_number(base_node: &mut Node, number: i32) {
    if base_node.number == number {
        let new_node = Node {
            number: number,
            children: Vec::new()
        };

        base_node.children.push(new_node);
    } else {
        for child_node in &mut base_node.children {
            add_subnode_to_number(child_node, number);
        }
    }
}

fn main() {
    let mut base_node = Node {
        number: 0,
        children: Vec::new()
    };

    let mut n = 1;
    
    while n < 10 {
        let r = apply_g(n);

        println!("{n}: {r}");

        add_subnode_to_number(&mut base_node, r);

        n = n + 1;
    }
}
