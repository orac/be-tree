# B𝛆 Tree library in Rust

Well, that's the goal. This was started as a "mob programming" experiment at a Rust meetup. At the meetup we didn't get as far as implementing a plain B-tree, but it's a start, and I've made some further changes since the meetup.

The idea is to implement the B𝛆-tree data structure from [this 2015 paper][1].
It's a refinement of the B-tree for faster writing (at some cost to queries). Instead of pushing changes to the relevant leaf node immediately, each branch node contains a (persistent) command queue which buffers changes. Thus, the cost of loading child nodes from disk is amortised over several operations.

[1]: http://supertech.csail.mit.edu/papers/BenderFaJa15.pdf

Unfortunately, queries are no faster (and a little slower, because they need to read the command queue). Operations that need to read an existing value and then update it would still be slow. The authors added an extra operation, *upsert*, which reads the value (if any), applies an operation to it, and writes the modified value. Upserts can be stored into the command queue like insertions, so they get the same performance benefit as inserts.

## Alternatives
* [C++ implementation of the same data structure][2], 2-clause BSD licence

[2]: https://github.com/oscarlab/Be-Tree

## Contributors

Every member of the "mob" at the meetup should go in this contributor list.
If that's you, please send a pull request that adds your name to the list.

* Dan Hulme <dan@five.ai>