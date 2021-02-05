# Implementing Mutex Guard

The first issue being rather simple. Our lock functions lock the mutex and return a mutable reference to our data which should work fine once we implement our lock_mech. The problem is that we never unlock the lock_mech once we are done making changes to the inner data.
The solution is to simply return a new struct called a Mutex_Guard TODO rusty from our locking functions. If we implement traits Deref and DerefMut on the Mutex_Guard the user will be able to treat like a reference to the inner data. By also implenting the drop trait to unlock the mutex when were are done and the lifetimes ensure we are able to do so safely.
So lets implement it.

TODO Mutex_Guard struct { &mut Mutex} !sync, !send, sized, drop, deref, derefmut, change old functions

Full code at this point:
