const GraphFunctions = require('./graphFunctions');

console.log(GraphFunctions.getGraph());
console.log(GraphFunctions.dfs(GraphFunctions.getGraph(), "1", "2"));

// BFS is implemented just to demonstrate how to perform bfs on this data structure
// It is useless if you just want to get a node from the graph or check if it exists in the graph

// To check if a node exists in the graph

let graph = GraphFunctions.getGraph();

// "1" can be replaced by the id of any node whose existence needs to be checked.
if ("1" in graph) {
    console.log("1 exists in the graph");
}

