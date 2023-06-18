class Node {
    constructor(val) {
        this.val = val;
        this.children = [];
        this.parents = [];
    }
}

function getGraph() {
    let graph = {
        "1": new Node("1"),
        "2": new Node("2"),
        "3": new Node("3"),
        "4": new Node("4"),
        "5": new Node("5"),
        "6": new Node("6"),
    }

    graph["1"].children.push("2", "3");
    graph["2"].children.push("4");
    graph["3"].children.push("5", "6");

    graph["2"].parents.push("1");
    graph["3"].parents.push("1");
    graph["4"].parents.push("2");
    graph["5"].parents.push("3");
    graph["6"].parents.push("3");

    return graph;
}

function dfs(graph, root, nodeToFind) {

    if (root === nodeToFind) {
        return {
            node: graph[root],
            depth: 0
        }
    }

    let visited = new Set()
    let stack = [root];

    while (stack.length) {
        for (let child of graph[stack[stack.length - 1]].children) {
            if (!visited.has(child)) {
                visited.add(child);
                stack.push(child);
                if (child === nodeToFind) {
                    return {
                        node: graph[child],
                        depth: stack.length
                    }
                }
            }
        }

        stack.pop();
    }

    return null;
}

function bfs(graph, root, nodeToFind) {

    if (root === nodeToFind) {
        return {
            node: graph[root],
            depth: 0
        }
    }

    let queue = [root];
    while (queue.length) {
        let currentParent = queue.shift();

        for (let child of graph[currentParent].children) {
            if (!queue.includes(child)) {
                queue.push(child);
                if (child === nodeToFind) {
                    return graph[child]
                }
            }
        }
    }
}

module.exports = { getGraph, dfs, bfs };
