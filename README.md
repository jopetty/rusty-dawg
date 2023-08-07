# Rusty DAWG

A library for building suffix automata for string indexing and searching in Rust.

For most basic use cases, it might be easier to use the Python wrapper [py-rusty-dawg](https://github.com/viking-sudo-rm/py-rusty-dawg).

# How to Use

## Building

To run tests for the repo, you can use Cargo:

```
cargo test
```

To compile an optimized release build, you can run:

```
cargo build --release
```

Note that the `--release` flag is very important for performance. The code will be 10-100x slower without it.

## Running Benchmarking Script

To run the benchmarking script, you first need to download the [data](https://drive.google.com/file/d/1XRZA2eki_Z8M0QrYN4BrbN7dghMYqYby/view?usp=sharing) directory, unzip it, and put it in the root of the repository directory (i.e., rusty-dawg/data). If you'd prefer, you can also retrieve the data directories for Wikitext2 and Wikitext103 from /home/willm/splits on NFS.

Now you will be able to benchmark building the DAWG on Wikitext2 or Wikitext103!

```
source scripts/benchmark.sh wikitext-2-raw
source scripts/benchmark.sh wikitext-103-raw
```

The total runtime will be printed out by the script's progress bar. You can find the size of the resulting DAWG by looking at:

```
ls -l /tmp/wikitext-2-raw.dawg
ls -l /tmp/wikitext-103-raw.dawg
```

# Documentation

This library implements the construction of a suffix automaton (or Directed Acyclic Word Graph, i.e., DAWG) on a large corpus. The suffix automaton is a finite-state machine (really, a graph) that can be used for very fast substring matching over the corpus.

The most relevant modules are the following:

### Graph Implementation: `src/graph`

Implements graph types for representing the DAWG. The two key ones are:

1. **src/graph/avl_graph**: A memory-efficient representation of a graph by a single list of nodes and a single list of edges. The list of edges associated with a node are stored by a balanced binary tree. This type is very memory efficient, but currently balancing is not fully implemented, so **it is not (yet) used!**
2. **src/graph/vec_graph**: Represent the edges out of each node by a sorted list of edges. This takes a lot of memory overhead because a vector needs to be allocated and reallocated for each node in the graph.

### DAWG Construction Algorithm: `src/dawg`

Builds a DAWG (represented as a graph) following the classical construction algorithm due to [Blumer (1984)](https://drive.google.com/file/d/1_FjsV3iSo1rA18DLzVpo_w2Zv4OhBWOl/view?usp=sharing).

### Weights of DAWG States: `src/weight`

Roughly speaking, the states/nodes of the DAWG represent different substrings (actually sets of "equivalent" substrings/indices) in the corpus. We associate additional information with each state via a **weight** to record information about that state, such as the length of the substring it represents and optionally its frequency in the corpus.

This is implemented by `src/weight`. There are various options here depending on how much memory we want to use and which quantities we want the weights to track.

### Tokenization: ``src/tokenize``

This module implements two tokenization strategies: either tokenize by whitespace and identify each token string to an integer ID (`TokenIndex`) or assume the data has already been tokenized and is space-separated (`NullTokenIndex`).

By default, main.rs will use `NullTokenIndex`, but you can pass `--tokenize` to use a custom `TokenIndex`.

### Other Modules

Much of the other modules are for n-gram language modeling and can be ignored for our purposes. `src/lms` implements various types of n-gram language models on top of a DAWG. `src/evaluator` implements logic to evaluate these n-gram language models on a test set. `src/stat_utils` implements a library of basic statistical functions for n-gram language modeling.

# Contributions

Very welcome! There are lots of interesting algorithmic improvements under the hood to make Rusty DAWG more efficient and scalable. Get in contact if you want to help out!