# Rust Pattern Problems

Welcome to the **Rust Pattern Problems** repository! 🎉 This project demonstrates solutions to popular pattern problems in **Rust**, showcasing **control flow** constructs and Rust's **module system**. Perfect for beginners looking to explore Rust through hands-on practice.

## 🚀 What is this?

This repository includes:
- Solutions to pattern problems from the [Must-Do Pattern Problems](https://takeuforward.org/strivers-a2z-dsa-course/must-do-pattern-problems-before-starting-dsa/).
- Practical use of Rust’s **module system** to organize and structure the codebase.
- Examples of Rust’s **control flow constructs** (`for`, `while`, `if`).

Patterns include:
- **Star patterns** (triangles, pyramids, etc.)
- **Number patterns**
- **Alphabet patterns**
- **Creative patterns**

## 📂 Folder Structure

```plaintext
📦 rust-pattern-problems
├── src
│   ├── main.rs                      # Entry point for the program
│   ├── pattern_problems/            # Each pattern is a separate module
│   │   ├── pattern_1.rs             
│   │   ├── pattern_2.rs             
│   │   ├── pattern_3.rs             
│   │   ├── pattern_4.rs             
│   │   └── ...                      
├── README.md                        # You are here!
├── Cargo.toml                       # Rust package file
```

## 🔧 How to Use

1. Clone the repository:
   ```bash
   git clone https://github.com/devasaya2003/RUST-Pattern-Problems.git
   cd RUST-Pattern-Problems
   ```

2. Run the project:
   ```bash
   cargo run
   ```

3. Explore `src/main.rs` to see how modules and patterns are organized.

## 📝 Code Explanation

### `main.rs`

The `main.rs` file acts as the program's entry point. Here’s how it’s structured:

1. **Attributes:**
   ```rust
   #![allow(unused_variables)]
   #![allow(dead_code)]
   ```
   These attributes suppress warnings for unused variables or functions, useful during exploration and debugging.

2. **Modules:**
   ```rust
   mod pattern_problems {
       pub mod pattern_1;
       pub mod pattern_2;
       pub mod pattern_3;
       // ... other patterns
   }
   ```
   The `pattern_problems` module imports each pattern as a submodule.

3. **Pattern Execution:**
   ```rust
   fn main() {
       let n: u32 = 4;
       pattern_problems::pattern_22::pattern22(n);
   }
   ```
   The `main` function demonstrates how to call a specific pattern function (e.g., `pattern22` from `pattern_22.rs`).

## 🎯 Why this Repository?

- **Practice Rust’s Control Flow:** Learn to use `for` and conditional statements in real-world scenarios.
- **Understand Modular Programming:** See how Rust organizes large codebases using modules.
- **Solve Fun Problems:** Practice and visualize creative pattern solutions in the terminal.

## 🤝 Contributing

Want to add new patterns or improve the codebase? Contributions are welcome! Fork this repository, make changes, and submit a pull request.

## ⭐ Acknowledgments

Inspired by the [Striver's A2Z DSA Course](https://takeuforward.org/strivers-a2z-dsa-course/).

If you find this project helpful, give it a ⭐ and share it with others!