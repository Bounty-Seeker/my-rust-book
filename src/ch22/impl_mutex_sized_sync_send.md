# Implementing Sync, Send and ?Sized

We have implemented the core functionality of our Mutex. It has now come time to examine the sync, send and ?Sized traits.

## Sync and Send

We can't just give blanket implementations of Send and Sync for Mutex\<T> with any T. This is clear when you consider the case when T is a Rc\<U>. The Mutex would allow us to create an Rc in two different threads.

TODO code
```rust
{{#rustdoc_include ./code/impl_mutex_sized_sync_send/mutex_sync_send.rs:mutex_send_sync}}
```

If we have the case where T is Sync but not Send then we still can't implement Send and Sync for our Mutex as the Mutex effectively moves the inner T across thread boundaries with just an &Mutex. TODO explain better example

This however explains why we can implement Sync and Send when T is Send. TODO more explain

This however not the fully settled, we need to consider the same question for the MutexGuard. We only need to consider the case when T is Send as if we consider all the other cases when T is not Send we could create the MutexGuard in the same thread as the Mutex, to only then transfer the MutexGuard to a different thread is effectively the same as making the Mutex Sync and Send.

TODO Check this with example trait bounds as maybe auto derived and not in docs. Tables

```rust
{{#rustdoc_include ./code/impl_mutex_sized_sync_send/mutex_sync_send.rs:mutex_guard_send_sync}}
```

As such we need to be careful in how we apply each to MutexGuard. If MutexGuard 



## ?Sized













