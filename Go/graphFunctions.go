package main

import (
	"fmt"
	"strings"

	"github.com/google/uuid"
)

type Node struct {
    val string
    children []string
    parents []string
}

func getExampleGraph() ( map[string]Node, []string ) {
    graph := make(map[string]Node)
    nodeList := []string{}


    for i := 0; i<6; i++ {
        nodeList = append(nodeList, uuid.New().String())
    }

    graph[nodeList[0]] = Node{
        val: nodeList[0],
        children: []string{nodeList[1], nodeList[2], nodeList[4]},
        parents: []string{},
    }

    graph[nodeList[1]] = Node{
        val: nodeList[1],
        children: []string{nodeList[5]},
        parents: []string{nodeList[0]},
    }

    graph[nodeList[2]] = Node{
        val: nodeList[2],
        children: []string{nodeList[3]},
        parents: []string{nodeList[0]},
    }

    graph[nodeList[3]] = Node{
        val: nodeList[3],
        children: []string{},
        parents: []string{nodeList[2]},
    }

    graph[nodeList[4]] = Node{
        val: nodeList[4],
        children: []string{nodeList[5]},
        parents: []string{nodeList[0]},
    }

    graph[nodeList[5]] = Node{
        val: nodeList[5],
        children: []string{},
        parents: []string{nodeList[4]},
    }

    return graph, nodeList;
}

func printStructure(graph map[string]Node) {
    rootNodes := []string{}
    for key, value := range graph {
        if len(value.parents) == 0 {
            rootNodes = append(rootNodes, key)
        }
    }

    for _, root := range rootNodes {
        fmt.Println(root)
        indent := 0
        for _, child := range graph[root].children {
            printChildrenRecursively(child, graph, indent+1)
        }
    }
}

func printChildrenRecursively(node string, graph map[string]Node, indent int) {
    fmt.Println(strings.Repeat(" |", indent), "|-", node)
    indent += 1;
    for _, child := range graph[node].children {
        fmt.Println(strings.Repeat(" |", indent), "|-", child)
        printChildrenRecursively(child, graph, indent+1)
    }
}

func dfs(graph map[string]Node, root string, nodeToSearch string) (Node, int, error) {

    if root == nodeToSearch {
        return graph[root], 0, nil;
    }

    visited := make(map[string]bool)
    stack := []string{}

    stack = append(stack, root)


    OUTER:
    for len(stack) > 0 {
        for _, child := range graph[stack[len(stack)-1]].children {
            if _, ok := visited[child]; !ok {
                visited[child] = true;
                stack = append(stack, child)
                if child == nodeToSearch {
                    return graph[child], len(stack), nil;
                }
                continue OUTER;
            }
        }

        stack = stack[:len(stack)-1]
    }

    return Node{}, 0, nil;
}

