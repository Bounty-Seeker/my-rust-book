# Implementing A Mutex

TODO check title and section titles make sense.
TODO check each import is code to library code

In this chapter we will be implementing a Mutex and it will be a messy complicated journey. On this journey we will explore atomics TODO link, Unsafe cell TODO link and how we can deal with panics.

The mutex guards a value while being accessed from multiple threads. It does this by only allowing mutable access from one thread at a time.

Modern implementations of the mutex, such as those found in the Rust standard library TODO link, takes advantage of OS specific features. We however will nit be taking advantage of these as they are beyond the scope of this chapter and irrelevant for our discussion. As such we will instead just use atomics. Due to this our implementation we spin (endlessly loop) while the mutex is unavailable.

Our implementation will in early sections of this chapter will have trait bounds !Sync, !Send and Sized TODO make rusty. As we move through the chapter these bounds we be analysed and possibly removed in our final implementation.

So lets get started.