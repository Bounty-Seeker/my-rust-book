//ANCHOR: here

#![feature(negative_impls)]

pub struct Mutex<T : Sized> {
    lock_mech : LockMech,
    data : T,
}

struct LockMech;

impl<T> !Send for Mutex<T> {}
impl<T> !Sync for Mutex<T> {}

impl<T:Sized> Mutex<T> {

    /// Create a mutex for some data.
    fn new(data:T) -> Mutex<T>  {
        todo!()
    }

    /// Tries to lock, spins until we get access to data.
    fn lock(&mut self) -> &mut T {
        todo!()
    }

    /// Tries to lock but returns with None if unable to get immediate access 
    fn try_lock(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Consume the mutex and return the inner T.
    fn into_inner(self) -> T {
        todo!()
    }
}
//ANCHOR_END: here