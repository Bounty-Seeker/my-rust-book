# Implementing our Mutex using an UnsafeCell

The second issue is much more complicated.
Our Mutex is going to be accessed from multiple places possibly at the same time as such we are going to have problems with using `&mut self` at each call site.

If we try and give out a `&mut self` to each thread, the compiler will correctly stop us from having multiple mutable references to the same element.

In order to solve this we will change all the previous functions to only require a `&self` instead.
The compiler will let us give out multiple shared references quite happily.
This will give us:

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell_intro.rs:here}}
```
TODO set doesn't compiler on import in mdbook md text when we test

This however also gives us a bunch of compiler errors in our other methods:

TODO compiler error
```console
{{#rustdoc_include ./code/impl_mutex_unsafecell/compiler_error1.txt}}
```
So we need to make the same change in the functions of our internal structs.

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell_intro2.rs:here}}
```

However this once more creates another new problem.
The compiler nicely stops us from making any changes to our internal structs as we no longer have a mutable reference to them.
But we can't get a mutable reference as our `MutexGuard` only has a shared reference to our `Mutex`.

```console
{{#rustdoc_include ./code/impl_mutex_unsafecell/compiler_error2.txt}}
```

So we need some way of getting mutable access from a shared reference.

Luckily there is a solution and that is the [`UnsafeCell<T>`][unsafecell].
The `UnsafeCell<T>` is the core compiler primitive when it comes to interior mutability.
All types implementing interior mutability are using the UnsafeCell at the lowest level.
It may look complicated to begin with but all it is a wrapper around a value with the additional aspect that it disables several compiler optimizations.
The docs for the [UnsafeCell][unsafecell] give a fuller picture while all we need to understand is that the `UnsafeCell<T>` lets us take a `&UnsafeCell<T>` to a `*mut T` which we can then turn into a `&mut T` via unsafe code. TODO mention other ways UnsafeCell works such as send sync behavior.
It also gives us the correct behavior from the drop checker and the correct variance, which we explain later. TODO which is.

So lets change our `Mutex<T>`'s data field to `UnsafeCell<T>` to take advantage of this behavior.

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here}}
```
And we also make changes to the appropriate functions:

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here2}}
```
Also we need to alter the deref functions for the `MutexGuard`:

```rust
{{#rustdoc_include ./code/impl_mutex_unsafecell/mutex_unsafecell.rs:here3}}
```

With this we can give safe mutable access to the inner data via our Mutex.

## Variance
TODO MutexGuard 'a and Mutex

For the `T` in `Mutex<T>` and `MutexGuard<'a,T>` we need it to be invariant.
This can be seen as we can get a `&mut T` from the `Mutex<T>` and `MutexGuard<'a,T>` so it must at least have the same variance, which is invariant, or stricter.
But as invariance is the strictest possible variance it must be invariant. This is already given by the `UnsafeCell<T>` so we need to make no changes.

Now we consider the `'a` in the `MutexGuard<'a,T>`.
This lifetime is linked to the lifetime of the MutexGuard and not the lifetime of the references it gives out.
It is sound to give a longer lived `MutexGuard<'a,T>` when we expect a shorter lived `MutexGuard<'a,T>`, which is covariance.
It is unsound the other way however.
Luckily `'a` is already covariant which it gets from its `&'a Mutex<T>`, so we again need to do no more.


## Dropcheck
TODO MutexGuard 'a and Mutex, check

The Mutex owns `T` so we need to let the compiler know this. The `UnsafeCell<T>` takes care of this for us.

The `MutexGuard<'a,T>` however only has a reference to the `T`.
Once again we need to do nothing as the compiler can work this out due to `&'a Mutex<T>` in the `MutexGuard<'a,T>`. TODO check

Now with all that out of the way we can get back and complete our locking mechanism.

[unsafecell]:https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html