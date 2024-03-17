# is-prime

Trait to check if a number is prime.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
is-prime = "0.1"
```

Or run the following command:

```sh
cargo add is-prime
```

## Usage

```rust
use is_prime::IsPrime;

assert!(2i8.is_prime());
assert!(3usize.is_prime());
assert!(5u128.is_prime());
assert!(7i64.is_prime());

assert!(!4i16.is_prime());
assert!(!6u16.is_prime());
assert!(!8u32.is_prime());
assert!(!9i128.is_prime());
```

## Supported Number Types

- `u8`
- `u16`
- `u32`
- `u64`
- `u128`
- `usize`
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `isize`

## License

This project is licensed under the [MIT license](LICENSE).
