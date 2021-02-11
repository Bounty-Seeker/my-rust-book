# Initial Layout


In this section we will create an initial skeleton for our Mutex.

A little thought leads us to these structure:

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_struct_init.rs}}
```

The data field shall hold our data while the ```LockMech``` struct will handle access control. It will only allow one thread access at any one time. The ```LockMech``` struct is the structure for which we could use conditional compilation to take advantage of each platforms features. This is how the std actually works.

Our Mutex will need some methods:

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_initial_layout.rs:here}}
```

TODO impl !sync, !send, Sized, pub

By also creating the following associated methods for our ```LockMech``` struct we can write our initial functions.

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_initial_layout2.rs:here2}}
```

Using the first three functions we can complete the bodies of our associated methods on the Mutex struct.

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_initial_layout2.rs:here}}
```

Our Mutex is already looking good and we have only just begun. However we already have several problems that we should get to before go further.
