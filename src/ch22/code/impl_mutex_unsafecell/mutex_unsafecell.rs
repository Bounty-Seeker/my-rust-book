
// ANCHOR: here
pub struct Mutex<T : Sized> {
    lock_mech : LockMech,
    data : UnsafeCell<T>,
}
// ANCHOR_END: here

struct LockMech;

unsafe impl<T> !Send, !Sync for Mutex<T>

//ANCHOR: here2
impl<T:Sized> Mutex<T> {

    /// Create a mutex for some data.
    fn new(data:T) -> Mutex<T>  {
        Mutex{
            lock_mech : LockMech::new(),
            data : UnsafeCell::new(data),
        }
    }

    /// Tries to lock, spins until we get access to data.
    fn lock(&'a self) -> MutexGuard<'a, T> {
        self.lock_mech.lock();
        MutexGuard::new(self)
    }

    /// Tries to lock but returns with None if unable to get immediate access 
    fn try_lock(&'a self) -> Option<MutexGuard<'a, T>> {
        if self.lock_mech.try_lock() {
            Some(MutexGuard::new(self))
        }
        else {
            None
        }
    }

    /// Consume the mutex and return the inner T.
    fn into_inner(self) -> T {
        self.data.into_inner()
    }
}
//ANCHOR_END: here2

impl LockMech {

    /// Create a LockMech.
    fn new() -> LockMech  {
        todo!()
    }

    /// Tries to lock, spins until we get access to data.
    fn lock(&self) {
        todo!()
    }

    /// Tries to lock but returns with False if unable to
    /// get immediate access. If it can get the lock we return
    /// True.
    fn try_lock(&self) -> bool {
        todo!()
    }

    /// Unlocks the lock.
    fn unlock(&self) {
        todo!()
    }
}

struct MutexGuard<'a, T:Sized> {
    mu : &'a Mutex<T>,
}

unsafe impl<'a, T> !Send, !Sync for MutexGuard<'a,T>

impl<'a, T:Sized> MutexGuard<'a,T> {

    fn new(mu : &'a Mutex<T>) -> MutexGuard<'a, T> {
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

//ANCHOR: here3
impl<'a, T:Sized> Deref for MutexGuard<'a,T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        & (*self.mu.data.get())
    }
}

impl<'a, T:Sized> DerefMut for MutexGuard<'a,T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut (*self.mu.data.get())
    }
}
//ANCHOR_END: here3

























