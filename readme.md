# Compare version

[Official Documentation](https://docs.ltpp.vip/COMPARE_VERSION/)

A Rust library for comparing semantic versioning strings and checking version compatibility.

## Features

- **Version Comparison**: Compare two semantic versioning strings to determine their order (greater, less, equal).
- **Version Range Matching**: Check if a specific version matches a specified range using `^` and `~` notation.
- **Pre-release Support**: Handle pre-release versions with proper comparison logic.
- **Error Handling**: Comprehensive error types to handle version parsing and range issues gracefully.

## Installation

To use this crate, you can run cmd:

```shell
cargo add compare_version
```

## Examples

```rust
use compare_version::*;
let result = compare_versions("1.2.3", "1.2.4");
assert_eq!(result, Ok(VersionComparison::Less));
let matches = matches_version_range("1.2.3", "^1.2.0");
assert_eq!(matches, Ok(true));
let matches = matches_version_range("1.2.3", "~1.2.4");
assert_eq!(matches, Ok(false));
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
