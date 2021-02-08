pub struct Mutex<T : Sized> {
    lock_mech : LockMech,
    data : T
}

struct LockMech;