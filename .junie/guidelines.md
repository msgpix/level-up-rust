# Level Up Rust: Development Guidelines

This document provides guidelines and instructions for developing and testing the "Level Up Rust" project.

## Build/Configuration Instructions

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- No external dependencies are required for this project

### Building the Project

1. **Development Build**
   ```bash
   cargo build
   ```

2. **Release Build**
   ```bash
   cargo build --release
   ```

3. **Running the Application**
   ```bash
   cargo run
   ```

## Testing Information

### Running Tests

1. **Run All Tests**
   ```bash
   cargo test
   ```

2. **Run Specific Tests**
   ```bash
   # Run tests with names containing "empty"
   cargo test empty

   # Run tests from a specific module
   cargo test test_example::
   ```

3. **Verbose Test Output**
   ```bash
   cargo test -- --nocapture
   ```

### Adding New Tests

There are two approaches to adding tests in this project:

1. **In-file Tests**: Add tests directly in the same file as the implementation, as shown in `src/main.rs`.

   ```rust
   #[test]
   fn test_name() {
       let input = vec![1.0, 2.0, 3.0];
       let expected_output = Some(2.0);
       let actual_output = median(input);
       assert_eq!(actual_output, expected_output);
   }
   ```

2. **Separate Test Modules**: Create separate test modules for more complex testing scenarios.

   - Create a new file with your tests (e.g., `src/my_tests.rs`)
   - Add the module to `main.rs`:
     ```rust
     #[cfg(test)]
     mod my_tests;
     ```
   - In your test file, import the functions you want to test:
     ```rust
     use crate::median;

     #[cfg(test)]
     mod tests {
         use super::*;

         #[test]
         fn your_test_name() {
             // Test implementation
         }
     }
     ```

### Test Coverage

The project currently includes tests for the following scenarios:

- Empty lists
- Sorted lists (odd length)
- Unsorted lists (requiring sorting)
- Even-length lists (requiring averaging of middle elements)
- Negative numbers
- Mixed positive and negative numbers

When adding new functionality, ensure you cover:
- Edge cases
- Normal operation
- Invalid inputs

## Code Style and Development Guidelines

### Rust Coding Style

- Follow the official [Rust Style Guide](https://github.com/rust-lang/style-team/blob/master/guide/guide.md)
- Use `rustfmt` to format your code:
  ```bash
  cargo fmt
  ```
- Use `clippy` to catch common mistakes and improve code quality:
  ```bash
  cargo clippy
  ```

### Project-Specific Guidelines

1. **Error Handling**
   - Use `Option` types for functions that may not return a valid result
   - Provide clear documentation for when `None` is returned

2. **Performance Considerations**
   - For functions that operate on vectors, consider whether to take ownership, borrow, or use a mutable reference
   - The current `median` function takes ownership of the vector, which is appropriate when the vector needs to be modified (sorted)

3. **Documentation**
   - Document public functions with doc comments (`///`)
   - Include examples in documentation when appropriate

4. **Debugging**
   - Use `println!` or the `dbg!` macro for quick debugging
   - For more complex debugging, consider using the `log` crate in future development

## Project Structure

- `src/main.rs`: Contains the main implementation and in-file tests
- Additional modules should be added as needed for more complex functionality
