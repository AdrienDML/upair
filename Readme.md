# UPair.
This data structure permits to have unordered pair keys for Hash based storage for example.
## Basic requirements.
The inner Type needs to implement the `Ord` trait.

```rust
let storage = HashSet::new();
storage.insert(UPair::new(1, 2));
assert!(storage.contains(UPair::new(2, 1));
```

## Trait implemented.
- `Debug` if implemented by inner type.
- `Copy` if implemented by inner type.
- `Hash` if implemented by inner type.
- `Eq, PartialEq` if implemented by inner type.
- `Ord, PartialOrd`.
- `From<(T, T)>`.
- `From<[T; 2]>`.
- `IntoIterator`.
