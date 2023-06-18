from graphFunctions import *

# File to call the graph functions

# getGraph gets a dummy graph
graph = getGraph()
print(graph)

# dfs can be used to get the node as well as the depth of it if required
foundNode = dfs(graph, "1", "6")

if foundNode is None:
    print("Node not found")
else:
    print("Node: ", foundNode[0])
    print("Depth: ", foundNode[1])

# for just getting a node in constant time from anywhere in the graph we can do the following

wantedNode = "6"

if wantedNode in graph:
    print("Node found")
    node = graph[wantedNode]

    print(node)

