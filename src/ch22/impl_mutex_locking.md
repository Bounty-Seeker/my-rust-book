# Locking our mutex

The time has now come to implement our actual locking mechanism in the `LockMech` struct.

As mentioned at the beginning of the chapter, modern OSs offer features to make the locking mechanism much more efficient.
Unfortunately in order to implement such a lock we would be required to get involved with the various APIs each OS and the hardware they expose. This however is outside the scope of the chapter, which is to understand the safety concerns when creating our own types and not the intricacies of OS design.
If you want to examine how we can create a Mutex using these features you can check the implementations in the standard library.

Our `Mutex` will utilise an [`AtomicBool`][atmbool] which has been exposed by Rust's std. This type is only available on platforms that support atomic loads and stores of `u8`. This should cover most platforms.

## Atomics
The [`AtomicBool`][atmbool] will be the core of our lock. When no one has a `MutexGuard` the bool will be `false` and when one exists it will be `true`. So let's import the appropriate modules and alter the `LockMech` struct.

```rust, ignore
{{#rustdoc_include ./code/impl_mutex_lock/mutex_locking.rs:lock_mech}}
```

For the `fn new() -> LockMech` method we will simply create an `AtomicBool` with the initial value `false`.

```rust, ignore
{{#rustdoc_include ./code/impl_mutex_lock/mutex_locking.rs:lock_mech_new}}
```

The `fn unlock(&self)` function is also relatively simple as we will simply use the `fn store(&self, val: bool, order: Ordering)` method with `val` set to `false` from the AtomicBool library to change it from `true` to `false`.
We will talk about the ordering in a moment.
TODO change ordering

```rust, ignore
{{#rustdoc_include ./code/impl_mutex_lock/mutex_locking.rs:lock_mech_unlock}}
```

Now we need to complete the lock functions.
The lock functions we need to be more complex as the methods must read the bool and if the bool is `false` we set it to `true` then alert us to this action but if the bool is `true` then we do nothing and alert ourselves to the fact all of which must be done atomically.

For `fn lock(&self)` we will repeatedly call the `fn try_lock(&self) -> bool` function in a loop and only return when `try_lock` returns `true`.

```rust, ignore
{{#rustdoc_include ./code/impl_mutex_lock/mutex_locking.rs:lock_mech_lock}}
```

TODO add hint::spin_lock() and link

The function `std::hint::spin_loop()` is a standard library function that signals to the processor that it is running in a busy-wait spin-loop if the capability is available on the target platform and does nothing if it is not.

Now we just need to complete the `fn try_lock(&self) -> bool` function.
The `fn swap(&self, val: bool, order: Ordering) -> bool` from `AtomicBool`'s library with `val` set to `true` will give us the correct behavior.

If the Mutex is not locked then the `LockMech`'s bool is `false`, `swap` will change the value to `true` and return `false`. Hence acquiring the lock and by taking the logical not of the output of `swap` we get our desired output.

If the Mutex is locked then the `LockMech`'s bool is `true`, `swap` will do nothing to the value, so it remains `true` and the `swap` function returns `true`. So just by taking the logical not of this we can finish our `try_lock` method.

```rust, ignore
{{#rustdoc_include ./code/impl_mutex_lock/mutex_locking.rs:lock_mech_try_lock}}
```

TODO change to swap and size_hint() and spin lock

## Orderings
TODO orderings

We this with have now finished the locking mechanism for the Mutex. With this the core functionality of the Mutex is complete. We now need to implement the appropriate traits for our Mutex.


[atmbool]:https://doc.rust-lang.org/std/sync/atomic/struct.AtomicBool.html







