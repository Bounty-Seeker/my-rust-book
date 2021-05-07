# Review

TODO unwind safe, ref unwind safe, unpin TODO Talk about unsafe code how little there is.

We now have a fully working Mutex. It can allow safe access to values from multiple threads. TODO expand

We have this code: TODO code

While this could be used with safely TODO check there are still some issues to address, namely mem::forget and poisoning. TODO rust check name

The mem::forget TODO rusty check name is simple to reason about. If any of the MutexGuard the Mutex will simply never unlock and just leak memory.

TODO Poisoning of itself why can't
The issue with poisoning is much more interesting and will lead to informative discussion. So lets implement guards against it.




