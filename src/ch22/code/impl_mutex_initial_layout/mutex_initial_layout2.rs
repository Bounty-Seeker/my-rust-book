
pub Struct Mutex<T> : Sized {
    lock_mech : LockMech,
    data : T,
}

Struct LockMech

Unsafe impl<T> !Send, !Sync for Mutex<T>

impl<T:Sized> Mutex<T> {

    /// Create a mutex for some data.
    fn new(data:T) -> Mutex<T>  {
        Mutex{
            lock_mech : LockMech::new(),
            data,
        }
    }

    /// Tries to lock, spins until we get access to data.
    fn lock(&mut self) -> &mut T {
        self.lock_mech.lock();
        &mut data,
    }

    /// Tries to lock but returns with None if unable to get immediate access 
    fn try_lock(&mut self) -> Option<&mut T> {
        if self.lock_mech.try_lock() {
            Some(&mut self.data)
        }
        else {
            None
        }
    }

    /// Consume the mutex and return the inner T.
    fn into_inner(Self) -> T {
        self.data
    }
}

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
    fn unlock(&mut Self) {
        todo!()
    }
}
