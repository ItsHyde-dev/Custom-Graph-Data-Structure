# Functions on the graph

class Node:
    def __init__(self, val):
        self.val = val
        self.children = []
        self.parents = []


def getGraph():
    graph = {
        "1":  Node("1"),
        "2":  Node("2"),
        "3":  Node("3"),
        "4":  Node("4"),
        "5":  Node("5"),
        "6":  Node("6"),
    }

    graph["1"].children.append("2")
    graph["1"].children.append("3")
    graph["2"].children.append("4")
    graph["3"].children.append("5")
    graph["3"].children.append("6")

    graph["2"].parents.append("1")
    graph["3"].parents.append("1")
    graph["4"].parents.append("2")
    graph["5"].parents.append("3")
    graph["6"].parents.append("3")

    return graph


def dfs(graph, root, nodeToFind):

    if root == nodeToFind:
        return (root, 0)

    stack = [root]
    visited = [root]

    while len(stack) > 0:
        continueOuter = False
        for child in graph[stack[-1]].children:
            if child not in visited:
                visited.append(child)
                stack.append(child)
                if child is nodeToFind:
                    return (graph[child], len(stack))
                continueOuter = True
                break
        if continueOuter:
            continue
        stack.pop()

    return None
