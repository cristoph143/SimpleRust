# Tests

This directory contains the test suite for "Simple Rust." Here, we have both unit and integration tests that help ensure our code meets its design specifications and behaves as intended.

## Running the Tests

To run the entire test suite, execute the following command from the root of the project:

```bash
cargo test
```

This command runs all tests, including unit tests defined in your src files next to their source and integration tests located in this `tests` directory.

### Specific Tests

If you wish to run a specific test or set of tests, you can specify the test name as follows:

```bash
cargo test test_name
```

### Test Structure

- Unit tests are typically located in the same file as the module they are testing, under a `#[cfg(test)]` mod.
- Integration tests are stored in this directory and can test multiple modules in conjunction.

### Best Practices

- Write clear, descriptive test names.
- Ensure tests are isolated and independent of each other.
- Use assertions effectively to check various conditions and outputs.

For more detailed information on Rust testing practices, refer to the [Rust Book](https://doc.rust-lang.org/book/ch11-00-testing.html).
