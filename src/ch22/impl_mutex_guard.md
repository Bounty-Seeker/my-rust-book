# Implementing Mutex Guard

The first issue being rather simple. Our lock functions lock the `Mutex` and return a mutable reference to our data which should work fine once we implement our `LockMech`. The problem is however that we never unlock the lock once we are done making changes to the inner data.

This problem is relatively common and has a well known solution is to simply return a new struct called a `MutexGuard` from our locking functions. If we implement traits `Deref` and `DerefMut` on the `MutexGuard` the user will be able to treat it like a reference to the inner data.
By also implementing the `drop` trait to unlock the `Mutex` when we are done.

So lets implement it.

We create the `MutexGuard` and its associated methods:

```rust
{{#rustdoc_include ./code/impl_mutex_guard/mutex_guard_intro.rs:here}}
```

And we alter the methods on the `Mutex` to return a `MutexGuard`.

```rust
{{#rustdoc_include ./code/impl_mutex_guard/mutex_guard_intro.rs:here2}}
```

That was relatively simple so lets move on to the next problem.