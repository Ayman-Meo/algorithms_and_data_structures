# Algorithms and Data Structures Roadmap

This guide is organized as a learning path. Start with the foundations, 
then move into core algorithmic techniques, data structures, graph algorithms, and advanced topics.

---

## 1. Mathematical Foundations

### Proof Techniques

- Mathematical induction
- Proof by contradiction
- Proof by construction
- Loop invariants
- Summations
  - Arithmetic series
  - Geometric series
  - Harmonic series
  - Summation rules

### Sets, Relations, and Functions

- Sets
- Relations
- Equivalence relations
- Functions

### Counting and Probability

- Counting principles
- Permutations
- Combinations
- Inclusion-exclusion principle
- Conditional probability
- Expected value
- Random variables

### Matrices

- Matrix operations
- Matrix multiplication
- Matrix properties

### Number Theory Basics

- Prime numbers
- GCD and LCM
- Euclidean algorithm
- Modular arithmetic
- Fast exponentiation

---

## 2. Introduction to Algorithms

### What Algorithms Are

- Algorithms as problem-solving procedures
- Algorithms as technology
- Correctness
- Efficiency

### Computational and Asymptotic Complexity

- Best-case analysis
- Average-case analysis
- Worst-case analysis

### Asymptotic Notation

- Big O notation
- Big Theta notation
- Big Omega notation
- Little o notation
- Little omega notation

### Amortized Complexity

- Aggregate analysis
- Accounting method
- Potential method

### Standard Notation and Common Functions

- Logarithmic functions
- Polynomial functions
- Exponential functions
- Factorials
- Common growth rates

### Complexity Classes Overview

- P
- NP
- NP-hard
- NP-complete

---

## 3. Divide and Conquer

> Divide and conquer is a powerful strategy for designing efficient
> algorithms by splitting a problem into smaller subproblems, solving them recursively, and combining the results.

### Core Ideas

- Divide step
- Conquer step
- Combine step
- Recursive algorithm structure

### Classic Algorithms

- Binary search
- Merge sort
- Matrix multiplication
- Strassen's algorithm for matrix multiplication

### Solving Recurrences

- Substitution method
- Recursion-tree method
- Master method
- Continuous master theorem
- Akra-Bazzi recurrences

---

## 4. Sorting and Order Statistics

> Sorting algorithms are a gateway to understanding algorithm design,
> tradeoffs, and performance analysis.

### Elementary Sorting Algorithms

- Insertion sort
- Selection sort
- Bubble sort
- Comb sort

### Intermediate Sorting Algorithms

- Shell sort
- Heap sort

### Divide-and-Conquer Sorting

- Merge sort
- Quick sort

### Linear-Time Sorting

- Counting sort
- Radix sort
- Bucket sort
- Sorting in linear time

### Order Statistics

- Selection problem
- Quickselect
- Median of medians

---

## 5. Data Structures

> Data structures organize information so programs can access, update, and process it efficiently.

### Linear Data Structures

- Arrays
- Dynamic arrays
- Stack
- Queue
- Deque
- Linked lists
  - Singly linked list
  - Doubly linked list
  - Circular linked list

### Hash-Based Structures

- Hash tables
  - Chaining
  - Open addressing
  - Rehashing

### Heaps

- Binary heap
- Min heap
- Max heap
- Priority queue

### Trees

- Binary trees
- Binary search trees
- AVL trees
- Red-black trees
- B-tree family
  - B-trees
  - B*-trees
  - B+-trees
  - Prefix B+-trees
  - K-d B-trees
  - Bit-trees
  - R-trees
  - 2-4 trees

### Trie-Based Structures

- Trie
- Compressed trie

### Range Query Structures

- Segment tree
- Lazy propagation
- Fenwick tree, also known as binary indexed tree
- Sparse table

### Augmented Data Structures

- Order-statistic trees
- Interval trees
- Augmenting existing structures safely

### Disjoint Set Data Structures

- Union-find
- Path compression
- Union by rank
- Union by size

---

## 6. Hashing

> Hashing is important in programming, databases, storage systems, and security.

### Core Concepts

- Hash functions
- Hash table operations
- Load factor
- Collision handling

### Collision Resolution

- Chaining
- Open addressing
- Deletion

### Advanced Hashing

- Perfect hash functions
- Universal hashing
- Rehashing
- Rolling hash
- Hash functions for extendible files
- Hashing with buckets

---

## 7. Greedy Algorithms

> Greedy algorithms build a solution piece by piece by choosing the locally best option at each step. They are often fast and simple, but they require proof that local choices lead to a globally optimal solution.

### Core Ideas

- Greedy choice property
- Optimal substructure
- Exchange arguments
- Matroids

### Classic Problems

- Activity selection problem
- Fractional knapsack
- Huffman coding
- Scheduling problems

### Greedy Graph Algorithms

- Kruskal's algorithm
- Prim's algorithm
- Dijkstra's algorithm

---

## 8. Dynamic Programming

### Core Ideas

- Optimal substructure
- Overlapping subproblems
- Memoization
- Tabulation
- State definition
- Transition recurrence

### Classical Problems

- Fibonacci
- Rod cutting
- Coin change
- Knapsack
- Matrix chain multiplication
- Longest common subsequence
- Longest increasing subsequence
- Edit distance

### Advanced Dynamic Programming

- Bitmask DP
- Tree DP
- Graph DP

---

## 9. Backtracking and Search

### Core Ideas

- State space search
- Constraint checking
- Pruning
- Branch and bound

### Classic Problems

- Subset generation
- Permutations
- Combinations
- N-Queens
- Sudoku solver

---

## 10. Probabilistic Analysis and Randomized Algorithms

> Probabilistic analysis studies algorithm behavior when randomness is involved, either in the input or inside the algorithm itself.

### Foundations

- The hiring problem
- Indicator random variables
- Expected running time
- Probabilistic analysis

### Randomized Algorithms

- Randomized algorithms
- Randomized quicksort
- Further uses of indicator random variables

---

## 11. Graph Algorithms

> Graph algorithms solve problems involving relationships, networks, dependencies, and paths.

### Graph Foundations

- Graph terminology
- Directed and undirected graphs
- Weighted and unweighted graphs
- Graph representations
  - Adjacency matrix
  - Adjacency list
  - Edge list

### Elementary Graph Algorithms

- Breadth-first search
- Depth-first search
- Topological sort
- Strongly connected components

### Minimum Spanning Trees

- Growing a minimum spanning tree
- Kruskal's algorithm
- Prim's algorithm

### Single-Source Shortest Paths

- Bellman-Ford algorithm
- Shortest paths in DAGs
- Dijkstra's algorithm
- 0-1 BFS
- A* search
- Difference constraints and shortest paths
- Proofs of shortest-path properties

### All-Pairs Shortest Paths

- Shortest paths and matrix multiplication
- Floyd-Warshall algorithm
- Johnson's algorithm for sparse graphs

### Connectivity Algorithms

- Bridges
- Articulation points
- Tarjan's algorithm
- Kosaraju's algorithm
- Lowest common ancestor

### Maximum Flow

- Flow networks
- Ford-Fulkerson method
- Edmonds-Karp algorithm
- Dinic's algorithm
- Maximum bipartite matching

### Matchings in Bipartite Graphs

- Maximum bipartite matching
- Stable marriage problem
- Hungarian algorithm

### NP-Complete Graph Problems

- Clique problem
- 3-colorability problem
- Vertex cover problem
- Hamiltonian cycle problem
- Distinct representatives

---

## 12. String Algorithms

### String Matching Basics

- Naive string matching
- Rabin-Karp
- Knuth-Morris-Pratt, also known as KMP
- Z algorithm
- Boyer-Moore

### Multiple Pattern Matching

- Aho-Corasick

### String Preprocessing Structures

- Prefix function
- Suffix arrays
- Suffix trees

---

## 13. Number-Theoretic Algorithms

### Core Algorithms

- Euclidean algorithm
- Extended Euclidean algorithm
- Modular arithmetic
- Fast modular exponentiation

### Prime Numbers

- Prime testing
- Sieve algorithms

### Advanced Topics

- Chinese remainder theorem
- Applications in cryptography

---

## 14. NP-Completeness

### Complexity Classes

- P
- NP
- NP-hard
- NP-complete

### Reductions

- Polynomial reductions
- SAT
- 3-SAT

### Classic NP-Complete Problems

- Clique
- Vertex cover
- Hamiltonian cycle

### Intractability

- Approximation
- Approximation and intractability

---

## 15. Selected Advanced Topics

### Algorithmic Techniques

- Parallel algorithms
- Online algorithms
- Approximation algorithms

### Mathematical and Numerical Algorithms

- Matrix operations
- Linear programming
- Polynomials and the FFT

### Machine Learning

- Machine-learning algorithms

---

## 16. Hardware-Aware Algorithms

> Hardware-aware algorithms consider memory, CPU behavior, and measurement tools when optimizing performance.

### Memory and Cache

- Memory hierarchy
- Cache-aware algorithms
- Cache-friendly data structures
- Locality of reference

### CPU and Parallelism

- SIMD concepts
- Parallel execution

### Measurement and Optimization

- Profiling
- Benchmarking
- Performance engineering in Rust
- Cargo bench
- Criterion
- Flamegraph
