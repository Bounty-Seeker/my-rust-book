# Initial Layout

In this section we will create an initial skeleton for our Mutex.

A little thought leads us to this basic structure:

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_struct_init.rs}}
```

The data field shall hold our data while the `LockMech` struct will handle access control. It will, as previously discussed only allow one thread access to the data at any one time and spin while waiting. The `LockMech` struct is also the element for which we could use conditional compilation to take advantage of each platforms features and is how the std actually works.

Our Mutex will also need some methods:

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_initial_layout.rs:here}}
```
This gives us the core functionality expected from a Mutex. This should be enough for our purpose and can easily be expanded to give further functionality.

TODO  Sized, pub

We can also create the following methods for our `LockMech` struct.

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_initial_layout2.rs:here2}}
```

Now using these functions we can now complete the bodies of our associated methods on the Mutex struct.

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_initial_layout2.rs:here}}
```

Our Mutex is already looking good and we have only just begun. However we already have several problems to address that we should fix before go further.
