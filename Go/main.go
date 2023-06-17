package main

import "fmt"

func main() {

    // getExampleGraph is used only for getting a dummy graph hardcoded in graphFunctions
    graph, nodeList := getExampleGraph()

    // printStructure is used for visualising the stucture of the custom graph data structure
    printStructure(graph)


    // dfs is used to perform depth first search on the graph to find the given element

    foundNode, depth, err := dfs(graph, nodeList[0], nodeList[5])
    if err != nil {
        fmt.Println("Could not find that node")
    }

    fmt.Println("Found node: ", foundNode)
    fmt.Println("Depth: ", depth)

    foundNode, depth, err = dfs(graph, nodeList[0], nodeList[0])
    if err != nil {
        fmt.Println("Could not find that node")
    }

    fmt.Println("Found node: ", foundNode)
    fmt.Println("Depth: ", depth)
}
