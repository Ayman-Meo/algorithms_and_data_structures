# INTRODUCTION TO ALGORITHMS AND DATA STRUCTURES IN RUST

This repository contains a collection of Rust implementations of various algorithms and data structures, organized using 
**Cargo Workspaces** for optimal compilation performance and clean boundaries.

> ⚠️ **Important Notice on Language Adaptation:**  When studying algorithms and data structures from traditional 
> resources (which are primarily written in C++, Java, or Pseudo-Code), you must adapt the
> implementation logic to fit your target language's paradigm. 
> 
> For instance, translating structures like linked lists, trees, or graphs directly 
> from C++ to Rust introduces challenges due to Rust's strict **ownership, borrowing,
> and lifetime rules**. To implement these safely and idiomatically, you will need to leverage concepts such 
> as smart pointers (`Box`, `Rc`, `RefCell`) 
> or use arena-based allocation models instead of raw,
> unmanaged pointers. Always focus on adapting the core mathematical concept rather than doing a line-by-line translation.

## Tools
- Rust Stable Toolchain V- 1.96.0+

## Resources
- [The Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [All C++ Implementation](https://github.com/TheAlgorithms/C-Plus-Plus)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [GeeksforGeeks - Data Structures & Algorithms](https://www.geeksforgeeks.org/data-structures/)
- [Algorithms for Competitive Programming](https://cp-algorithms.com/)
- [The Algorithms - Rust Implementation](https://github.com/TheAlgorithms/Rust)
- [Algorithms for Modern Hardware](https://en.algorithmica.org/hpc/profiling/instrumentation/)
- Books:
  - "Introduction to Algorithms 4th edition" by Thomas H. Cormen, Charles E. Leiserson, Ronald L. Rivest, and Clifford Stein

## Contributing
Contributions to this repository are welcome! If you have an implementation of an algorithm or data structure that is not already included, please feel free to submit a pull request. Make sure your code is formatted with `cargo fmt` and passes all tests via `cargo test`.

## License
This repository is licensed under the Mit License. See the [LICENSE](LICENSE) file for more details.

## Contact
If you have any questions or suggestions regarding the code in this repository, please feel free to contact me via [email](mailto:eng.ayman.meo@gmail.com) or visit my [GitHub](https://github.com/Ayman-Meo).

---

## Overview Of Project Structure
```text
.
│   Cargo.toml         # Root workspace configuration
│   LICENSE            # The license of the program
│   
├───app/               # Main CLI application binary
│   └───Cargo.toml
│           
├───docs/              # Markdown files for explanations and notices
│   └───GUIDE.MD
│
├───lib/
│   ├───data_structures/      # Data structures implementation
│   │   └───Cargo.toml
│   │           
│   ├───graph/                # Graph algorithms implementation 
│   │   └───Cargo.toml
│   │           
│   ├───hashing/              # Hashing algorithms implementation 
│   │   └───Cargo.toml
│   │           
│   ├───sort/                 # Sorting algorithms implementation 
│   │   └───Cargo.toml
│   │           
│   └───string_matching/      # String matching algorithms implementation 
│       └───Cargo.toml
│           
└───utilities/         # Logging, array comparisons, benchmarking helpers, etc.
    └───Cargo.toml
```
---

## **[Learning Path (⚠️🚨🚨 Click here)](./docs/GUIDE.MD)**

### Overview 
- Mathematical Foundations
- Introduction to Algorithms
- Divide and Conquer
- Sorting and Order Statistics
- Data Structures
- Hashing
- Greedy Algorithms
- Dynamic Programming
- Backtracking and Search
- Probabilistic Analysis and Randomized Algorithms
- Graph Algorithms
- String Algorithms
- Number-Theoretic Algorithms
- Selected Topics from CLRS
- NP-Completeness
- Hardware-Aware Algorithms


## RUN THE APP

**run in terminal opened in root direct of project:**
``` bash
  cargo build
  cargo run -p app
```

