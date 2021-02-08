# Unsafe cell

The second issue is much more complicated. Our Mutex is going to be accessed from multiple places possibly at the same time as such we are going to have problems with using &mut Mutex<T> at each call site. In order to solve this we will change all the previous functions to only require a &T. This will give us:

TODO change func signatures
```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell_intro.rs:here}}
```




This will however create a new problem. The compiler nicely stops us from making any changes to our structs as we no longer have a mutable reference. We need someway of getting mutable access from a shared reference.

TODO compiler error
```console
{{#rustdoc_include ./code/impl_mutex_unsafecell/compiler_error1.txt}}
```

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell_intro2.rs:here}}
```

```console
{{#rustdoc_include ./code/impl_mutex_unsafecell/compiler_error2.txt}}
```

Luckily there is a solution and that is the UnsafeCell<T> TODO link. The UnsafeCell<T> is the core compiler primitive when it comes to interior mutability. All types implementing interior mutability are using the UnsafeCell st the lowest level.
It may look complicated to begin with but all it is a wrapper around a value with the additional aspect that it disables several compiler optimizations. The docs for the UnsafeCell TODO link give a fuller picture while all we need to understand is that the UnsafeCell gives us the correct behavior from the drop checker and the correct variance TODO which is.



So lets change our Mutex's data field to UnsafeCell to take advantage of this behavior.

TODO change Mutex struct

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here}}
```

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here2}}
```

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here3}}
```

Now using the TODO new, into_inner and get methods on the UnsafeCell to fix our Mutex:

TODO show fixed methods

The unsafe code in the above code, is safe because only one MutexGuard will have access to the data at any point thanks to our locking mechanism.

Full code at this point:


