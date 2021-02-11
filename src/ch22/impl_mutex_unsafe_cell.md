# Unsafe cell

The second issue is much more complicated.
Our Mutex is going to be accessed from multiple places possibly at the same time as such we are going to have problems with using ```&mut self``` at each call site.
The compiler will stop us from having mutable references to the same element.

In order to solve this we will change all the previous functions to only require a ```&self``` instead. This will give us:

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell_intro.rs:here}}
```
TODO set doesn't compiler on import

This gives us a bunch of compiler errors in our other methods:

TODO compiler error
```console
{{#rustdoc_include ./code/impl_mutex_unsafecell/compiler_error1.txt}}
```
So we need to also change the functions in our internal structs.

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell_intro2.rs:here}}
```

This once more creates a new problem. The compiler nicely stops us from making any changes to our internal structs as we no longer can get a mutable reference from the shared reference to our ```Mutex```.

```console
{{#rustdoc_include ./code/impl_mutex_unsafecell/compiler_error2.txt}}
```

So we need some way of getting mutable access from a shared reference.

Luckily there is a solution and that is the [```UnsafeCell<T>```][unsafecell].
The ```UnsafeCell<T>``` is the core compiler primitive when it comes to interior mutability. All types implementing interior mutability are using the UnsafeCell at the lowest level.
It may look complicated to begin with but all it is a wrapper around a value with the additional aspect that it disables several compiler optimizations.
The docs for the [```UnsafeCell```][unsafecell] give a fuller picture while all we need to understand is that the ```UnsafeCell<T>``` lets us take a ```&UnsafeCell<T>``` to a ```&mut T``` via unsafe code.
It also gives us the correct behavior from the drop checker and the correct variance, which we explain later. TODO which is.

So lets change our ```Mutex```'s data field to ```UnsafeCell``` to take advantage of this behavior.

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here}}
```
And we also make changes to the appropriate functions:

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here2}}
```
Also we need to alter the deref functions for the ```MutexGuard```:

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here3}}
```

The unsafe code in the above code, is safe because only one ```MutexGuard``` will have access to the data at any point thanks to our locking mechanism.

With this we can give mutable access to the inner data.

## Variance
TODO MutexGuard 'a and Mutex

## Dropcheck
TODO MutexGuard 'a and Mutex

[unsafecell]:https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html