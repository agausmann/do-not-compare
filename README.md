# do-not-compare

A wrapper type that inhibits comparison and hashing of the inner value.

## Motivation

This pattern is an experimental solution for scenarios where you have key-value
pairs that you want to compare or sort in some way, but you only want the key to
be compared and not the associated value (or if the associated value is just not
comparable).

The typical solution is to define a wrapper struct containing both the key and
value, and write custom implementations for traits like `PartialEq`, `Hash`,
`PartialOrd`, and `Ord` that only check the key:

```rust
struct KeyValuePair {
    key: MyKeyType,
    value: MyValueType,
}

impl PartialEq for KeyValuePair {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for KeyValuePair {}

impl PartialOrd for KeyValuePair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

/* et cetera ... */
```

This is not very ergonomic; there are derive macros for all of these
traits in the standard library! If only we could use them... 

The problem is, if the struct has even one incomparable field, even if you do
not care about comparing that field, the derive macros will not work; they will
produce code that does not compile.

However, with `do-not-compare`, you _can_ use the derive macros:

```rust
use do_not_compare::DoNotCompare;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct KeyValuePair {
    key: MyKeyType,
    value: DoNotCompare<MyValueType>,
}
```

This should behave identically to the previous example, but it is a lot more
concise!

## Implementation

`DoNotCompare` is pretty simple. It wraps the inner value and tries to be as
transparent as possible, providing most of the standard owned & borrowed
conversion traits. Notably, this does not include `Borrow` and `BorrowMut`
traits, due to violating the following requirement (from the docs of
[`Borrow`][borrow]):

> `Eq`, `Ord` and `Hash` must be equivalent for borrowed and owned values:
> `x.borrow() == y.borrow()` should give the same result as `x == y`.

`DoNotCompare` provides its own comparison implementations that do not require
the inner type to be comparable. All of the comparisons always report that
values of `DoNotCompare` are equal: `PartialEq::eq` returns `true`, and
`Ord::cmp` returns `Equal`. It also provides a `Hash` implementation that is a
no-op. Effectively, `DoNotCompare<T>` behaves like `PhantomData<T>` for
comparisons and hashing, except it actually does contain a value of type `T` but
ignores it.

## Drawbacks

- **Does not work with generics** - In the `KeyValuePair` examples,
  `DoNotCompare` will not be very useful if `KeyValuePair` is generic over the
  value type. If you defined it as, say, `struct KeyValuePair<K, V>` with a
  `DoNotCompare<V>` field, then derive macros for comparison traits will still
  place bounds on _both_ `K` and `V` which is too restrictive.

[borrow]: https://doc.rust-lang.org/stable/core/borrow/trait.Borrow.html