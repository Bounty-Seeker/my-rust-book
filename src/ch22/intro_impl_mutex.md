# Implementing A Mutex

TODO check title and section titles make sense.
TODO check each import is code to library code

In this chapter we will be implementing a ```Mutex``` and it will be a messy complicated journey. On this journey we will explore [```Atomics```][atomic], [```UnsafeCell```][unsafecell] and how we can deal with panics.

The ```Mutex``` guards a value while being accessed from multiple threads. It does this by only allowing mutable access from one thread at a time.

Modern implementations of the [```Mutex```][mu-std], such as those found in the Rust standard library, takes advantage of OS specific features. We however will not be taking advantage of these as they are beyond the scope of this chapter and irrelevant for our discussion. As such we will instead just use Atomics which will work on all platforms for which Atomics on u8 are implemented. In addition to this our implementation we spin (endlessly loop) while the Mutex is unavailable.

Our implementation will in the early sections will have trait bounds ```!Sync```, ```!Send``` and ```Sized```. As we move through the chapter these bounds we be analysed and possibly removed in our final implementation.

So lets get started.


[atomic]: https://doc.rust-lang.org/std/sync/atomic/index.html
[unsafecell]: https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html
[mu-std]: https://doc.rust-lang.org/std/sync/struct.Mutex.html