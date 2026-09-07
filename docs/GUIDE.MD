# Algorithms and Data Structures Roadmap

This guide is organized as a learning path. Start with the foundations,
then move into core algorithmic techniques, data structures, graph algorithms, and advanced topics.

---

## [1. Mathematical Foundations](./Mathematical%20Foundations/introduction.md)

### 1.1. Proof Techniques

- Mathematical induction
- Proof by contradiction
- Proof by construction
- Loop invariants
- Summations
  - Arithmetic series
  - Geometric series
  - Harmonic series
  - Summation rules

### 1.2. Sets, Relations, and Functions

- Sets
- Relations
- Equivalence relations
- Functions

### 1.3. Counting and Probability

- Counting principles
- Permutations
- Combinations
- Inclusion-exclusion principle
- Conditional probability
- Expected value
- Random variables

### 1.4. Matrices

- Matrix operations
- Matrix multiplication
- Matrix properties

### 1.5. Number Theory Basics

- Prime numbers
- GCD and LCM
- Euclidean algorithm
- Modular arithmetic
- Fast exponentiation

---

## [2. Introduction to Algorithms](./Introduction%20to%20Algorithms/introduction.md)

### 2.1. What Algorithms Are?

- Algorithms as problem-solving procedures
- Algorithms as technology
- Correctness
- Efficiency

### 2.2. Computational and Asymptotic Complexity

- Best-case analysis
- Average-case analysis
- Worst-case analysis

### 2.3. Asymptotic Notation

- Big O notation
- Big Theta notation
- Big Omega notation
- Little o notation
- Little omega notation

### 2.4. Amortized Complexity

- Aggregate analysis
- Accounting method
- Potential method

### 2.5. Standard Notation and Common Functions

- Logarithmic functions
- Polynomial functions
- Exponential functions
- Factorials
- Common growth rates

### 2.6. Complexity Classes Overview

- P
- NP
- NP-hard
- NP-complete

---

## [3. Divide and Conquer](./Divide%20and%20Conquer/introduction.md)

> Divide and conquer is a powerful strategy for designing efficient
> algorithms by splitting a problem into smaller subproblems, solving them recursively, and combining the results.

### 3.1. Core Ideas

- Divide step
- Conquer step
- Combine step
- Recursive algorithm structure

### 3.2. Classic Algorithms

- Binary search
- Merge sort
- Matrix multiplication
- Strassen's algorithm for matrix multiplication

### 3.3. Solving Recurrences

- Substitution method
- Recursion-tree method
- Master method
- Continuous master theorem
- Akra-Bazzi recurrences

---

## [4. Sorting and Order Statistics](./Sorting%20and%20Order%20Statistics/introduction.md)

> Sorting algorithms are a gateway to understanding algorithm design,
> tradeoffs, and performance analysis.

### 4.1. Elementary Sorting Algorithms

- Insertion sort
- Selection sort
- Bubble sort
- Comb sort

### 4.2. Intermediate Sorting Algorithms

- Shell sort
- Heap sort

### 4.3. Divide-and-Conquer Sorting

- Merge sort
- Quick sort

### 4.4. Linear-Time Sorting

- Counting sort
- Radix sort
- Bucket sort
- Sorting in linear time

### 4.5. Order Statistics

- Selection problem
- Quickselect
- Median of medians

---

## [5. Data Structures](./Data%20Structures/introduction.md)

> Data structures organize information so programs can access, update, and process it efficiently.

### 5.1. Linear Data Structures

- Arrays
- Dynamic arrays
- Stack
- Queue
- Deque
- Linked lists
  - Singly linked list
  - Doubly linked list
  - Circular linked list

### 5.2. Hash-Based Structures

- Hash tables
  - Chaining
  - Open addressing
  - Rehashing

### 5.3. Heaps

- Binary heap
- Min heap
- Max heap
- Priority queue

### 5.4. Trees

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

### 5.5. Trie-Based Structures

- Trie
- Compressed trie

### 5.6. Range Query Structures

- Segment tree
- Lazy propagation
- Fenwick tree, also known as binary indexed tree
- Sparse table

### 5.7. Augmented Data Structures

- Order-statistic trees
- Interval trees
- Augmenting existing structures safely

### 5.8. Disjoint Set Data Structures

- Union-find
- Path compression
- Union by rank
- Union by size

---

## [6. Hashing](./Hashing/introduction.md)

> Hashing is important in programming, databases, storage systems, and security.

### 6.1. Core Concepts

- Hash functions
- Hash table operations
- Load factor
- Collision handling

### 6.2. Collision Resolution

- Chaining
- Open addressing
- Deletion

### 6.3. Advanced Hashing

- Perfect hash functions
- Universal hashing
- Rehashing
- Rolling hash
- Hash functions for extendible files
- Hashing with buckets

---

## [7. Greedy Algorithms](./Greedy%20Algorithms/introduction.md)

> Greedy algorithms build a solution piece by piece by choosing the locally best option at each step. They are often fast and simple, but they require proof that local choices lead to a globally optimal solution.

### 7.1. Core Ideas

- Greedy choice property
- Optimal substructure
- Exchange arguments
- Matroids

### 7.2. Classic Problems

- Activity selection problem
- Fractional knapsack
- Huffman coding
- Scheduling problems

### 7.3. Greedy Graph Algorithms

- Kruskal's algorithm
- Prim's algorithm
- Dijkstra's algorithm

---

## [8. Dynamic Programming](./Dynamic%20Programming/introduction.md)

### 8.1. Core Ideas

- Optimal substructure
- Overlapping subproblems
- Memoization
- Tabulation
- State definition
- Transition recurrence

### 8.2. Classical Problems

- Fibonacci
- Rod cutting
- Coin change
- Knapsack
- Matrix chain multiplication
- Longest common subsequence
- Longest increasing subsequence
- Edit distance

### 8.3. Advanced Dynamic Programming

- Bitmask DP
- Tree DP
- Graph DP

---

## [9. Backtracking and Search](./Backtracking%20and%20Search/introduction.md)

### 9.1. Core Ideas

- State space search
- Constraint checking
- Pruning
- Branch and bound

### 9.2. Classic Problems

- Subset generation
- Permutations
- Combinations
- N-Queens
- Sudoku solver

---

## [10. Probabilistic Analysis and Randomized Algorithms](./Probabilistic%20Analysis%20and%20Randomized%20Algorithms/introduction.md)

> Probabilistic analysis studies algorithm behavior when randomness is involved, either in the input or inside the algorithm itself.

### 10.1. Foundations

- The hiring problem
- Indicator random variables
- Expected running time
- Probabilistic analysis

### 10.2. Randomized Algorithms

- Randomized algorithms
- Randomized quicksort
- Further uses of indicator random variables

---

## [11. Graph Algorithms](./Graph%20Algorithms/introduction.md)

> Graph algorithms solve problems involving relationships, networks, dependencies, and paths.

### 11.1. Graph Foundations

- Graph terminology
- Directed and undirected graphs
- Weighted and unweighted graphs
- Graph representations
  - Adjacency matrix
  - Adjacency list
  - Edge list

### 11.2. Elementary Graph Algorithms

- Breadth-first search
- Depth-first search
- Topological sort
- Strongly connected components

### 11.3. Minimum Spanning Trees

- Growing a minimum spanning tree
- Kruskal's algorithm
- Prim's algorithm

### 11.3. Single-Source Shortest Paths

- Bellman-Ford algorithm
- Shortest paths in DAGs
- Dijkstra's algorithm
- 0-1 BFS
- A* search
- Difference constraints and shortest paths
- Proofs of shortest-path properties

### 11.4. All-Pairs Shortest Paths

- Shortest paths and matrix multiplication
- Floyd-Warshall algorithm
- Johnson's algorithm for sparse graphs

### 11.5. Connectivity Algorithms

- Bridges
- Articulation points
- Tarjan's algorithm
- Kosaraju's algorithm
- Lowest common ancestor

### 11.6. Maximum Flow

- Flow networks
- Ford-Fulkerson method
- Edmonds-Karp algorithm
- Dinic's algorithm
- Maximum bipartite matching

### 11.7. Matchings in Bipartite Graphs

- Maximum bipartite matching
- Stable marriage problem
- Hungarian algorithm

### 11.8. NP-Complete Graph Problems

- Clique problem
- 3-colorability problem
- Vertex cover problem
- Hamiltonian cycle problem
- Distinct representatives

---

## [12. String Algorithms](./String%20Algorithms/introduction.md)

### 12.1. string Matching Basics

- Naive string matching
- Rabin-Karp
- Knuth-Morris-Pratt, also known as KMP
- Z algorithm
- Boyer-Moore

### 12.2. Multiple Pattern Matching

- Aho-Corasick

### 12.3. String Preprocessing Structures

- Prefix function
- Suffix arrays
- Suffix trees

---

## [13. Number-Theoretic Algorithms](./Number-Theoretic%20Algorithms/introduction.md)

### 13.1. Core Algorithms

- Euclidean algorithm
- Extended Euclidean algorithm
- Modular arithmetic
- Fast modular exponentiation

### 13.2. Prime Numbers

- Prime testing
- Sieve algorithms

### 13.3. Advanced Topics

- Chinese remainder theorem
- Applications in cryptography

---

## [14. NP-Completeness](./NP-Completeness/introduction.md)

### 14.1. Complexity Classes

- P
- NP
- NP-hard
- NP-complete

### 14.2. Reductions

- Polynomial reductions
- SAT
- 3-SAT

### 14.3. Classic NP-Complete Problems

- Clique
- Vertex cover
- Hamiltonian cycle

### 14.4. Intractability

- Approximation
- Approximation and intractability

---

## [15. Selected Advanced Topics](./Selected%20Advanced%20Topics/introduction.md)

### 15.1. Algorithmic Techniques

- Parallel algorithms
- Online algorithms
- Approximation algorithms

### 15.2. Mathematical and Numerical Algorithms

- Matrix operations
- Linear programming
- Polynomials and the FFT

### 15.3. Machine Learning

- Machine-learning algorithms

---

## [16. Hardware-Aware Algorithms](./Hardware-Aware%20Algorithms/introduction.md)

> Hardware-aware algorithms consider memory, CPU behavior, and measurement tools when optimizing performance.

### 16.1. Memory and Cache

- Memory hierarchy
- Cache-aware algorithms
- Cache-friendly data structures
- Locality of reference

### 16.2 CPU and Parallelism

- SIMD concepts
- Parallel execution

### 16.3 Measurement and Optimization

- Profiling
- Benchmarking
- Performance engineering in Rust
- Cargo bench
- Criterion
- Flamegraph
