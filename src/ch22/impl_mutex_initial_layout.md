# Initial Layout


In this section we will create an initial skeleton for our mutex.

A little thought leads us to this struct :

TODO mutex<T> struct lock : lock_mech, data : T            struct lock_mech

The data field shall hold our data while the lock_mech struct will handle our access control. It will only allow one thread access at any one time. The lock_mech struct is the structure for which we could use conditional compilation to take advantage of each platforms features. This is how the std actually works.

Our mutex will hence need some functions

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_struct_init.rs}}
```

 TODO  /// Create a mutex for some data. new(data:T) -> Mutex<T> , 
       /// Tries to lock, spins until we get access to data. lock(&mut self) -> &mut T
       /// Tries to lock but returns with error if unable to get immediate access try_lock(&mut self) -> Result<&mut T, error type>
       /// Consume the mutex and return the inner T. into_inner(Self) -> T

TODO impl !sync, !send, Sized, pub

By also creating the following associated method for our lock_mech struct we can write our initial functions.

```rust
{{#rustdoc_include ./code/impl_mutex_initial_layout/mutex_initial_layout.rs:here}}
```

TODO /// Create  Myjhdhjnew lock_mech for a mutex new_lock()-> Lock_Mech
/// Tries to lock, spins until we get access to data. lock(&mut self) -> &mut T
/// Tries to lock but returns with error if unable to get immediate access try_lock(&mut self) -> Result<&mut T, error type>
/// 

Using the first three functions we can complete the bodies of our associated methods on the Mutex struct.

TODO  Previous functions completed

This Mutex is already looking good and we have only just begun. However we already have several problems that we should get to before go further.

Full code at this point:





