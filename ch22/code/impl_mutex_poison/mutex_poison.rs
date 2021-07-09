compiler_error!(ensure sized , past are changed )
pub struct Mutex<T> : Sized {
    lock_mech : LockMech,
    data : UnsafeCell<T>,
    poison : AtomicBool,
}

//ANCHOR: mutex_send_sync
Unsafe impl<T : !Send> !Send, !Sync for Mutex<T>
Unsafe impl<T : Send> Send, Sync for Mutex<T>
//ANCHOR_END: mutex_send_sync

impl<T:Sized> Mutex<T> {

    /// Create a mutex for some data.
    fn new(data:T) -> Mutex<T>  {
        Mutex{
            lock_mech : LockMech::new(),
            data : UnsafeCell::new(data),
            poison : AtomicBool::new(False),
        }
    }

    /// Tries to lock, spins until we get access to data.
    fn lock(&'a self) -> MutexGuard<'a, T> {
        self.lock_mech.lock();
        if self.poison.get() == True {
            panic!()
        }
        else {
            MutexGuard::new(self)
        }
    }

    /// Tries to lock but returns with None if unable to get immediate access 
    fn try_lock(&'a self) -> Option<MutexGuard<'a, T>> {
        if self.lock_mech.try_lock() {
            if self.poison.get() == True {
                panic!()
            }
            else {
                Some(MutexGuard::new(self))
            }
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

    struct LockMech {
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


//ANCHOR: mut_guard
struct MutexGuard<'a, T:Sized> {
    mu : &'a Mutex<T>,
    guard: bool,
}
//ANCHOR_END: mut_guard

//ANCHOR: mutex_guard_send_sync
//Question over send and poisoning
Unsafe impl<'a, T> !Send for MutexGuard<'a,T>
Unsafe impl<'a, T : !Sync> !Sync for MutexGuard<'a,T>
Unsafe impl<'a, T : Sync> Sync for MutexGuard<'a,T>
//ANCHOR_END: mutex_guard_send_sync

//ANCHOR: mut_gu_new
impl<'a, T:Sized> MutexGuard<'a,T> {
    fn new(mu : &'a Mutex<T>) -> MutexGuard<'a, T> {
        MutexGuard {
            mu,
            guard: thread::panicking(),
        }
    }
}
//ANCHOR_END: mut_gu_new

//ANCHOR: mut_gu_drop
impl<'a, T:Sized> Drop for MutexGuard<'a,T> {
    fn drop(&mut self) {
        if thread::panicking() && !self.guard
        {
            self.mu.poison.set(True);
        }
        self.mu.lock_mech.unlock();
    }
}
//ANCHOR_END: mut_gu_drop


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


























