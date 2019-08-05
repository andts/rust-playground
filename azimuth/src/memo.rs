//a smart structure to hold all variants of a rel tree during its optimisation.
//think about saving the lineage of each rel node.

//actions:
// - populate with initial tree
// - add new node as an alternative for some specific node
// - find the cheapest tree among all permutations

//to think about:
// - ways to inspect the whole tree in a rule
// - how to evaluate the cost? should the memo do this or some other component?
