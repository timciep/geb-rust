mod rule_appliers;

use rule_appliers::*;

use ptree::{TreeBuilder, print_tree};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// Simple program for printing a GEB Diagram G.
struct Args {
    /// Which rule to use? (g, g_flip, f, m)
    #[clap(short, long, value_parser)]
    rule: String,

    /// Number of iterations.
    #[clap(short, long, value_parser, default_value_t=20)]
    iterations: i32,

    /// Show steps?
    #[clap(short, long, value_parser, default_value_t=false)]
    steps: bool,
}

struct Node {
    number: i32,
    children: Vec<Node>
}

fn main() {
    let args = Args::parse();

    let mut base_node = Node {
        number: 1,
        children: Vec::new()
    };

    let mut n = 2;
    
    while n <= args.iterations {
        let r = match args.rule.as_str() {
            "g" => apply_g(n),
            "g_flip" => apply_g_flip(n),
            "f" => apply_f(n),
            "m" => apply_m(n),
            _ => panic!("Must provide a valid --rule")
        };

        if args.steps {
            println!("{}({n}) = {r}", args.rule);
        }

        add_subnode_to_number(&mut base_node, r, n);

        n = n + 1;
    }

    // https://docs.rs/ptree/latest/ptree/
    let mut tree_builder = TreeBuilder::new(base_node.number.to_string());

    construct_tree_from_node(base_node, &mut tree_builder);

    if args.steps {
        println!("--------------------------------");
    }
    print_tree(&tree_builder.build()).ok();
}

fn add_subnode_to_number(base_node: &mut Node, parent_number: i32, new_subnode_number: i32) {
    if base_node.number == parent_number || parent_number == new_subnode_number {
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
        tree_builder.begin_child(child_node.number.to_string());

        construct_tree_from_node(child_node, tree_builder);

        tree_builder.end_child();
    }

    tree_builder
}
