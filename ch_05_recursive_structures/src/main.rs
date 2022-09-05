use ptree::{TreeBuilder, print_tree};

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

fn add_subnode_to_number(base_node: &mut Node, parent_number: i32, new_subnode_number: i32) {
    if base_node.number == parent_number {
        let new_node = Node {
            number: new_subnode_number,
            children: Vec::new()
        };

        base_node.children.push(new_node);
    } else {
        for child_node in &mut base_node.children {
            add_subnode_to_number(child_node, parent_number, new_subnode_number);
        }
    }
}

fn construct_tree_from_node(base_node: Node, tree_builder: &mut TreeBuilder) -> &mut TreeBuilder {
    for child_node in base_node.children {
        println!("child_node.number: {}", child_node.number.to_string());

        tree_builder.begin_child(child_node.number.to_string());

        construct_tree_from_node(child_node, tree_builder);

        tree_builder.end_child();
    }

    tree_builder
}

fn main() {
    let mut base_node = Node {
        number: 1,
        children: Vec::new()
    };

    let mut n = 2;
    
    while n < 20 {
        let r = apply_g(n);

        // println!("G({n}) = {r}");

        add_subnode_to_number(&mut base_node, r, n);

        n = n + 1;
    }

    // https://docs.rs/ptree/latest/ptree/
    let mut tree_builder = TreeBuilder::new(base_node.number.to_string());

    construct_tree_from_node(base_node, &mut tree_builder);

    print_tree(&tree_builder.build()).ok();
}
