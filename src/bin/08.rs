use std::io::stdin;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

fn do_read_node(ns: &[i32]) -> (Node, i32) {
    let children_cnt = ns[0];
    let metadata_cnt = ns[1];

    let mut children = Vec::new();
    let mut next = 2;
    for _ in 0..children_cnt {
        let (child, child_next) = do_read_node(&ns[(next as usize)..]);
        children.push(child);
        next += child_next;
    }

    let mut metadata = Vec::new();
    for i in 0..metadata_cnt {
        metadata.push(ns[(next + i) as usize]);
    }

    (
        Node {
            children: children,
            metadata: metadata,
        },
        next as i32 + metadata_cnt,
    )
}

fn read_node(ns: &Vec<i32>) -> Node {
    let (node, _) = do_read_node(ns);
    return node;
}

fn sum_metadata_all(root: &Node) -> i32 {
    return root
        .children
        .iter()
        .map(|child| sum_metadata_all(child))
        .sum::<i32>()
        + root.metadata.iter().sum::<i32>();
}

fn read_input() -> Vec<i32> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    return input
        .split(" ")
        .map(|k| k.parse::<i32>().expect("should be a number"))
        .collect::<Vec<_>>();
}

fn part_one() {
    let numbers = read_input();
    let root = read_node(&numbers);
    let result = sum_metadata_all(&root);

    println!("{result}");
}

fn sum_metadata_children(root: &Node) -> i32 {
    let mut result = 0;
    if root.children.is_empty() {
        return root.metadata.iter().sum::<i32>();
    }
    for &i in &root.metadata {
        if i <= 0 || i > root.children.len() as i32 {
            continue;
        }
        result += sum_metadata_children(&root.children[(i - 1) as usize]);
    }

    return result;
}

fn part_two() {
    let numbers = read_input();
    let root = read_node(&numbers);
    let result = sum_metadata_children(&root);

    println!("{result}");
}

fn main() {
    part_two();
}
