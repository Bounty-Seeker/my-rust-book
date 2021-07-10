# Poisoning

 The issue is what happens if the user code panics while its making changes to the inner data it can result in the data being left in an incoherent state.
We can implement mechanisms in order to alert the user if this happens. At the moment whether to include this aspect is included in the std library, at the time of writing this, is under debate. TODO check, link, why debate
TODO minimal panic safe

## Problem and Solution
The problem with our code is what happens if the user code panics while altering the inner data. This can result in the inner data being placed in an inconsistent state. TODO example

## Implementation
Implementation of this will require us to know when the thread we are currently in is panicking. To do this we will make use of the std function thread::panicking() TODO rusty. This function returns True if the current thread is panicking and False otherwise.

If when the MutexGuard is dropped we check if the thread is panicking. If it is we set a flag on the Mutex to note such a fact. Then when future threads try to access we need to notify them that the data has been poisoned and we need to do so while maintaining our safety guarantees.

In order to implement this we will first need to add an AtomicBool to the Mutex that tracks whether the inner data is poisoned or not. TODO Why AtomicBool

TODO add AtomicBool and change setup funcs
```rust, ignore
{{#rustdoc_include ./code/impl_mutex_poison/mutex_poison_init.rs:mutex}}
```

```rust, ignore
{{#rustdoc_include ./code/impl_mutex_poison/mutex_poison_init.rs:mutex_fun}}
```


This should allow us to record if it is possible that the inner data has been poisoned.
And we need to alter the drop TODO rusty method for the MutexGuard to check.

TODO drop method
```rust, ignore
{{#rustdoc_include ./code/impl_mutex_poison/mutex_poison_init.rs:mut_guard_drop}}
```


And we need to alter the locking functions. We will discuss how we can report this to the user in the next section for now we will just panic!().

TODO locking method change
TODO does it need to be atomic
TODO how do we unset poison flag

There is still one wrinkle with are implementation.

## If threads are panicking already
The issue is that the panic may have already happened when the thread obtains the MutexGuard. This could have if some types obtain a MutexGuard in their drop trait implementation. This could lead to false positives. So in order to avoid this we will record in our MutexGuard if the thread is panicking when we obtain it, then when we drop the MutexGuard we check if the MutexGuard's flag is set we don't set the poison flag as we were already panicking.
So we set the Mutex poison flag  on the drop of the MutexGuard if the MutexGuard flag is not set and the thead is panicking.

Before we implement this we need to ask ourselves some questions: TODO Double panic, catch_unwind

TODO implement changes to MutexGuard and functions
```rust, ignore
{{#rustdoc_include ./code/impl_mutex_poison/mutex_poison.rs:mut_guard}}
```

```rust, ignore
{{#rustdoc_include ./code/impl_mutex_poison/mutex_poison.rs:mut_gu_new}}
```

```rust, ignore
{{#rustdoc_include ./code/impl_mutex_poison/mutex_poison.rs:mut_gu_drop}}
```


TODO Send questions check all correct in previous chapters

We also need the MutexGuard !Send TODO rusty in all cases. This is as we are now storing a thread specific data in the MutexGuard.

TODO make not Send
```rust, ignore
{{#rustdoc_include ./code/impl_mutex_poison/mutex_poison.rs:mutex_guard_send_sync}}
```

TODO why safe

TODO safety

We now have protected from poisoning and is now time for us to work out how to tell the user their data is possibly poisoned.















