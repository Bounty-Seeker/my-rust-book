#![feature(negative_impls)]

// ANCHOR: here
use std::ops::{Drop, Deref, DerefMut};

// --snip--

// ANCHOR_END: here

pub struct Mutex<T : Sized> {
    lock_mech : LockMech,
    data : T,
}

struct LockMech;

impl<T> !Send for Mutex<T> {}
impl<T> !Sync for Mutex<T> {}

//ANCHOR:here2
impl<T:Sized> Mutex<T> {

//  --snip--

//ANCHOR_END:here2
    /// Create a mutex for some data.
    fn new(data:T) -> Mutex<T>  {
        Mutex{
            lock_mech : LockMech::new(),
            data : data,
        }
    }

// ANCHOR: here2
    /// Tries to lock, spins until we get access to data.
    fn lock<'a>(&'a mut self) -> MutexGuard<'a, T> {
        self.lock_mech.lock();
        MutexGuard::new(self)
    }

    /// Tries to lock but returns with None if unable to get immediate access 
    fn try_lock<'a>(&'a mut self) -> Option<MutexGuard<'a, T>> {
        if self.lock_mech.try_lock() {
            Some(MutexGuard::new(self))
        }
        else {
            None
        }
    }

//  --snip--

// ANCHOR_END: here2

    /// Consume the mutex and return the inner T.
    fn into_inner(self) -> T {
        self.data
    }
// ANCHOR: here2
}
//ANCHOR_END: here2

impl LockMech {

    /// Create a LockMech.
    fn new() -> LockMech  {
        todo!()
    }

    /// Tries to lock, spins until we get access to data.
    fn lock(&mut self) {
        todo!()
    }

    /// Tries to lock but returns with False if unable to
    /// get immediate access. If it can get the lock we return
    /// True.
    fn try_lock(&mut self) -> bool {
        todo!()
    }

    /// Unlocks the lock.
    fn unlock(&mut self) {
        todo!()
    }
}


//ANCHOR: here
pub struct MutexGuard<'a, T:Sized> {
    mu : &'a mut Mutex<T>,
}

impl<'a, T> !Send for MutexGuard<'a,T> {}
impl<'a, T> !Sync for MutexGuard<'a,T> {}

impl<'a, T:Sized> MutexGuard<'a,T> {
    fn new(mu : &'a mut Mutex<T>) -> MutexGuard<'a, T> {
        MutexGuard {
            mu
        }
    }
}

impl<'a, T:Sized> Drop for MutexGuard<'a,T> {
    fn drop(&mut self) {
        self.mu.lock_mech.unlock();
    }
}

impl<'a, T:Sized> Deref for MutexGuard<'a,T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        & self.mu.data
    }
}

impl<'a, T:Sized> DerefMut for MutexGuard<'a,T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mu.data
    }
}
//ANCHOR_END: here