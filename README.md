# Custom Graph Data Structure

## Concept

* Each **Node** of the graph is represented by an entry in a **Hashmap** or an equivalent data structure.
    - This makes it so that any node can be accessed in O(1) time complexity.
* Each **Node** holds the id(key) of it's parents and children.
* This allows us to traverse the structure using the functions demonstrated in this repository.

## Use

* This structure can be used to establish graph relations between data using ids to denote a data element.
* An advantage of this structure over a traditional nested graph structure is that any node can be treated as the root for traversal without having to start any traversal from the root node.

## Future Scope 

Any suggestions or requests as to which language or operation I should add to this repo are welcome. As of now I have shown a simple dfs example in all of the languages which is pretty simple to understand on this structure.


