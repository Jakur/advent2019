use super::*;

struct Node<'a> {
    parent: &'a str, // parent
    children: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn new_empty() -> Node<'a> {
        Node {
            parent: "",
            children: Vec::new(),
        }
    }
}

pub fn p6(input: &str) -> Answer {
    let mut nodes = HashMap::new();
    let mut san_parent = "";
    let mut you_parent = "";
    for line in input.lines() {
        let mut split = line.split(")");
        let parent = split.next().unwrap();
        let child = split.next().unwrap();
        let p = nodes.entry(parent).or_insert(Node::new_empty());
        p.children.push(child);
        let mut c = nodes.entry(child).or_insert(Node::new_empty());
        c.parent = parent;
        if child == "SAN" {
            san_parent = parent;
        }
        if child == "YOU" {
            you_parent = parent;
        }
    }
    let mut total_orbits = 0;
    let mut depth: i32 = 1;
    let mut queue = vec!["COM"];
    let mut santa_depth = -1;
    let mut you_depth = -1;
    while queue.len() != 0 {
        let level: Vec<_> = queue.drain(..).collect();
        for next_str in level {
            if next_str == san_parent {
                santa_depth = depth;
            }
            if next_str == you_parent {
                you_depth = depth;
            }
            let next = &nodes[next_str];
            for child in next.children.iter() {
                queue.push(*child);
                total_orbits += depth;
            }
        }
        depth += 1;
    }
    if san_parent == you_parent {
        return Answer::new(total_orbits, 0);
    }
    let mut santa_ancestors = HashSet::new();
    let mut node = nodes.get(san_parent).unwrap();
    depth = santa_depth;
    santa_ancestors.insert(san_parent);
    while node.parent != "" {
        depth -= 1;
        santa_ancestors.insert(node.parent);
        node = nodes.get(node.parent).unwrap();
    }
    assert_ne!(santa_ancestors.get("COM"), None);
    node = nodes.get(you_parent).unwrap();
    depth = you_depth;
    let lca_depth = loop {
        depth -= 1;
        if santa_ancestors.contains(node.parent) {
            break depth;
        }
        node = nodes.get(node.parent).unwrap();
    };
    let distance = santa_depth + you_depth - 2 * lca_depth;
    Answer::new(total_orbits, distance)
}