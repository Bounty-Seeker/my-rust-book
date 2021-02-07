
pub Struct Mutex<T> : Sized {
    lock_mech : LockMech,
    data : UnsafeCell<T>,
}



Unsafe impl<T> !Send, !Sync for Mutex<T>

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
        self.data
    }
}



    Struct LockMech {
        locked : AtomicBool,
    }

impl LockMech {

    /// Create a LockMech.
    fn new() -> LockMech {
        LockMech {
            locked : AtomicBool::new(False),
        }
    }


    /// Tries to lock, spins until we get access to data.
    fn lock(&self) {
        while !self.try_lock()
    }

    /// Tries to lock but returns with False if unable to
    /// get immediate access. If it can get the lock we return
    /// True.
    fn try_lock(&self) -> bool {
        self.locked.compare_and_swap(&self, False, True, order: Ordering)
    }

    /// Unlocks the lock.
    fn unlock(&self) {
        self.locked.set(False, Ordering)
    }
}



Struct MutexGuard<'a, T:Sized> {
    mu : &'a Mutex<T>,
}

Unsafe impl<'a, T> !Send, !Sync for MutexGuard<'a,T>


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


























