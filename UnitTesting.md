# Unit Testing

## Assertions
When multiple asserts in a unit tests are required, the errors need to be collected and reported together.
This ensures that all failures are visible in a single test run, rather than stopping at the first failure.
Provide macros like `check_eq!` when needed.

**BAD Example**
```rust
assert_eq!(result.floor.size_front, 10);
assert_eq!(result.floor.size_back, 11);
assert_eq!(result.floor.size_left, 12);
assert_eq!(result.floor.size_right, 13);
```

**GOOD Example**
```rust
macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}
```
```rust
let mut errors = Vec::new();

check_eq!(errors, result.floor.size_front, 10);
check_eq!(errors, result.floor.size_back, 11);
check_eq!(errors, result.floor.size_left, 12);
check_eq!(errors, result.floor.size_right, 13);

assert!(
    errors.is_empty(),
    "Assertion failures:\n{}",
    errors.join("\n")
);
```

Note that this is not required when there is only a single assertion in the test.
BDD style integration testing using `rspec` also do not require this.

## Unit Test Structure
- Unit tests should be structured using the Arrange-Act-Assert (AAA) pattern for clarity and maintainability.
- The sections should be clearly marked by `// arrange` `// act` and `//assert` comments.
- The object under test should be named `subject` for consistency across tests.
- The primary result object of the action should be named `result` (if applicable).

## Integration Test Structure
- Integration tests shall be done using BDD style testing using `rspec` and follow Behavior-Driven Development (BDD) principles, using descriptive names and scenarios to enhance readability and understanding of the test cases.
- When the user requires a specific behavior from the system, it should be implemented as a scenario in `rspec` to ensure that the system meets user requirements.
