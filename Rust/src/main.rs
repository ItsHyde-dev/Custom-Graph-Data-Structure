mod graph_functions;
use graph_functions::*;

fn main() {

    // create graph is a dummy function where you will just get a dummy graph with a list of it's
    // nodes

    let (graph, node_list) = create_graph();

    // to get the children of a given node get_child_graph can be used.

    let child_graph = get_child_graph(graph.clone(), node_list[2]);
    println!("child graph: {:?}", child_graph);

    // to perform depth first search on the graph this function can be used.
    // It returns the node and the depth of the node

    let (found_node, depth) = dfs(graph.clone(), node_list[0], node_list[1]);
    println!("depth: {}", depth);
    println!("found node: {:?}", found_node);

    // to perform breadth first search on the graph this function can be used.
    // It returns just the node

    let found_node_bfs = bfs(graph.clone(), node_list[0], node_list[1]);
    println!("found node using bfs: {:?}", found_node_bfs);
}

