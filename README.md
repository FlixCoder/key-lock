# KeyLock

[![crates.io page](https://img.shields.io/crates/v/key-lock.svg)](https://crates.io/crates/key-lock)
[![docs.rs page](https://docs.rs/key-lock/badge.svg)](https://docs.rs/key-lock/)
![license: MIT](https://img.shields.io/crates/l/key-lock.svg)

Simple library for mutual exclusion based on keys. Lock and wait for execution by key.

## Usage

Import the project using:

```bash
cargo add key-lock
```

## Example

```rust
use key_lock::KeyLock;

#[tokio::main]
async fn main() {
    // Initialize new lock.
    let lock = KeyLock::new();
    // Lock A, continues immediately.
    let _a = lock.lock("a").await;
    // Lock B, continues immediately.
    let _b = lock.lock("b").await;
    // Try to lock A, but it is already locked. Normal locking would block here.
    assert!(lock.try_lock("a").await.is_err());
}
```

## Minimum supported Rust version

Currently, I am always using the latest stable Rust version and do not put in effort to keep the MSRV. Please open an issue in case you need a different policy, I might consider changing the policy.

## License

Licensed under the MIT license. All contributors agree to license under this license.
