use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct Node {
    val: uid::Id<u32>,
    children: Vec<uid::Id<u32>>,
    parents: Vec<uid::Id<u32>>,
}

pub fn create_graph() -> (HashMap<uid::Id<u32>, Node>, Vec<uid::Id<u32>>) {
    let mut graph: HashMap<uid::Id<u32>, Node> = HashMap::new();
    let mut node_list: Vec<uid::Id<u32>> = Vec::new();

    for i in 0..10 {
        let id: uid::Id<u32> = uid::Id::new();
        if i % 2 == 0 {
            node_list.push(id);
            graph.insert(
                id,
                Node {
                    val: id,
                    children: vec![],
                    parents: vec![],
                },
            );
        } else {
            let parent_id: uid::Id<u32> = node_list.last().unwrap().clone();
            node_list.push(id);
            graph.insert(
                id,
                Node {
                    val: id,
                    children: vec![],
                    parents: vec![parent_id],
                },
            );

            graph.get_mut(&parent_id).unwrap().children.push(id);
        }
    }

    return (graph, node_list);
}

pub fn get_node(graph: &HashMap<uid::Id<u32>, Node>, node_id: uid::Id<u32>) -> Node {
    return graph.get(&node_id).cloned().unwrap();
}

pub fn get_child_graph(graph: HashMap<uid::Id<u32>, Node>, node_id: uid::Id<u32>) -> HashMap<uid::Id<u32>, Node> {

    let root = get_node(&graph, node_id);
    let mut visited: HashSet<uid::Id<u32>> = HashSet::new();
    let mut child_graph: HashMap<uid::Id<u32>, Node> = HashMap::new();
    let mut to_visit: Vec<&Node> = vec![&root];

    visited.insert(root.val.clone());
    child_graph.insert(root.val.clone(), root.clone());

    while to_visit.len() > 0 {
        let current_node = to_visit.pop().unwrap();
        println!("Current Node: {:?}", current_node);
        for child in current_node.children.iter() {
            if !visited.contains(child) {
                visited.insert(*child);
                child_graph.insert(child.clone(), get_node(&graph, *child));
                to_visit.push(graph.get(child).unwrap());
            }
        }
    }

    return child_graph;
}

pub fn dfs(graph: HashMap<uid::Id<u32>, Node>, root: uid::Id<u32>, search_node: uid::Id<u32>) -> (Option<Node>, i32){

    if root == search_node {
        return (Some(get_node(&graph, root)), 0);
    }

    let mut visited: HashSet<uid::Id<u32>> = HashSet::new();
    let mut stack: Vec<Node> = vec![get_node(&graph, root)];


    'outer: while stack.len() > 0 {

        for child in stack.last().unwrap().children.clone().iter() {
            if !visited.contains(child) {
                if child == &search_node {
                    return (Some(get_node(&graph, *child)), stack.len() as i32);
                }
                visited.insert(*child);
                stack.push(get_node(&graph, *child));
                continue 'outer;
            }
        }

        stack.pop();
    }

    return ( None, 0 );

}

pub fn bfs(graph: HashMap<uid::Id<u32>, Node>, root: uid::Id<u32>, search_node: uid::Id<u32>) -> Option<Node>{

    if root == search_node {
        return Some(get_node(&graph, root));
    }

    let mut current_row: Vec<uid::Id<u32>> = vec![root];

    while current_row.len() > 0 {
        if current_row.first().unwrap() == &search_node {
            return Some(get_node(&graph, *current_row.first().unwrap()));
        }

        current_row.append(&mut graph.get(&current_row.first().unwrap()).unwrap().children.clone());
        current_row.remove(0);
    }

    return None;

}
