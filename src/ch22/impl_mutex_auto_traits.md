# Implementing Auto Traits
We have implemented the core functionality of our ```Mutex```. We can safely lock and access the inner data. As such it has now come time to examine the ```Sync```, ```Send``` and ```?Sized``` traits.

## Sync and Send

We will first consider the ```Sync``` and ```Send``` trait
### Mutex
With a little thought, we see that we can't just give blanket implementations of ```Send``` and ```Sync``` for ```Mutex<T>``` with any ```T```. This is becomes clear when you consider the case when ```T``` is a ```Rc<U>```. The ```Mutex``` would allow us to create an ```Rc``` in two different threads.

Looking at the access the ```Mutex<T>```and the ```&Mutex<T>``` give us the right idea.

The ```Mutex<T>``` effectively owns the type ```T``` and gives mutable access to ```T```. This leads us to the fact that if ```T``` is ```Send``` then so is the ```Mutex<T>```.
However if ```T``` is just ```Sync``` then this is not enough as we can see if we consider the case where we wrap such a ```T``` in a ```Mutex<T>```, transfer that across the boundary then use the ```into_inner``` method to get the inner data. This would allow us to illegal take any `T:Sync + !Send` across boundaries despite the fact that `T` is not `Send`.

The ```&Mutex<T>``` gives mutable and shared access, via a ```MutexGuard```, to ```T```. This tells us that just knowing that ```T``` is ```Sync``` is not enough to deduce that ```&Mutex<T>``` can cross thread boundaries. Instead, by the same reasoning, knowing that ```T``` is ```Send``` is enough for ```&Mutex<T>``` to be safe to cross thread boundaries. i.e. ```Mutex<T>``` is ```Sync``` if and only if ```T``` is ```Send```.

The above reasoning is summarized in the table below TODO table.

And we can alter are code to reflect this.

TODO code, table
```rust
{{#rustdoc_include ./code/impl_mutex_sized_sync_send/mutex_sync_send.rs:mutex_send_sync}}
```

TODO atomicbool thread safe? explain in all cases
### MutexGuard
The issue of `Sync` and `Send` however is not quite don, we need to consider the same question for the MutexGuard.

While considering whether the ```MutexGuard<'a,T>``` itself can be moved across thread boundaries we only need to consider when `T` is `Send` as it quickly follows from our earlier reasoning in the ```&Mutex``` case.
This is because moving a ```&Mutex<T>``` across the boundary then creating a ```MutexGuard<'a, T>``` is equivalent to creating a ```MutexGuard<'a,T>``` in the original thread and moving that across thread boundaries.
Now in the case that `T` is `Send` it is enough to deduce that it is safe to move ```MutexGuard<'a, T>``` across thread boundaries. This is because moving the ```MutexGuard<'a,T>``` across the boundaries is logically equivalent to moving `T` across the boundaries and moving the `T` back to the original thread when we drop the ```MutexGuard<'a,T>``` which is ensured to be safe when `T` is `Send`.

Now let us consider whether ```&MutexGuard<'a,T>``` is safe to move across thread boundaries. The methods on ```&MutexGuard<'a,T>``` can only give us a shared reference to `T`. As such ```&MutexGuard<'a,T>``` is safe to move across thread boundaries. In other words ```MutexGuard<'a,T>``` is `Sync` if and only if `T` is `Sync`.

This is again expressed in the table below:

TODO table

Now let us change the code to reflect this:

TODO Check this with example trait bounds as maybe auto derived and not in docs. Tables

```rust
{{#rustdoc_include ./code/impl_mutex_sized_sync_send/mutex_sync_send.rs:mutex_guard_send_sync}}
```


## Sized

### Mutex

### MutexGuard

## Catch Unwind and Ref Catch Unwind

### Mutex

### MutexGuard

## Pin

### Mutex

### MutexGuard