# Implementing A Mutex

TODO check title and section titles make sense.
TODO filename vs html
TODO add listings to each import
TODO pub code
TODO add section for other functions
TODO check each import is code to library code
TODO mention why all in one file
TODO describe algorithm and recursive or not

In this chapter we will be implementing a `Mutex` and it will be a messy complicated journey.
On this journey we will explore [`Atomics`][atomic], the [`UnsafeCell`][unsafecell] and poisoning.

The `Mutex` guards a value while allowing access to the value from multiple threads. It does this by using thread synchronization to only allow access to the variable from one thread at a time.

Modern implementations of the [`Mutex`][mu-std], such as those found in the Rust standard library, involve complex locking mechanisms and take advantage of OS specific features.
We however will not be taking advantage of these as they are beyond the scope of this chapter and irrelevant for our discussion.
Our Mutex will instead just use Atomics to synchronize which will work on all platforms for which Atomics on `u8` are implemented.
In addition to further simplify our implementation threads will spin (endlessly loop) while the Mutex is unavailable.

Simple spinlocks are an extremely poor method for locking Mutexes and come with several harmful side effects.
To learn more about this check out TODO link

This unfortunately that means that the Mutex we create here is unsuitable for real use but it will still enlighten us the important issues.

Our implementation will in the early sections will have trait bounds `!Sync`, `!Send` and `Sized`.
As we move through the chapter these bounds we be analysed and possibly removed in our final implementation.

So lets get started.


[atomic]: https://doc.rust-lang.org/std/sync/atomic/index.html
[unsafecell]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html
[mu-std]: https://doc.rust-lang.org/std/sync/struct.Mutex.html