# Locking our mutex

We have now reached the stage at which we will need to implement our actual locking mechanism in the LockMech struct.
As mentioned at the beginning of the chapter, modern OSs offer features to make the locking mechanism much more efficient. Unfortunately in order to implement such a lock we would be required to get involved with the various APIs each OS and hardware exposes. This however is not the purpose of this chapter, which is to understand the safety concerns when created our own types. As such our Mutex will utilise a simple atomic which has been exposed by Rust's std. If you want to examine how we create a mutex using these features you can check the implementations in the std library TODO link

## Atomics

We shall use the AtomicBool TODO rusty type as the core of our lock. When no one has a MutexGuard the bool will be False and when one exists it will be True.

TODO make LockMech

For the new TODO rusty method  we will simply require the new TODO method of  AtomicBool TODO with value False.

TODO new method

The unlock function is also relatively simple as we will simply use the store TODO rusty method from the AtomicBool. We will talk about the ordering in a moment.

TODO unlock function

Now we need to do the lock functions. The lock functions we need to be more complex as the methods must read the bool and if the bool is False we set it to true then alert us to this action but if the bool is True then we do nothing and alert ourselves to the fact. All of this must be done atomically.
This  can be done with compare_and_swap TODO rusty method with current set to False and new set to True.

TODO lock functions

TODO orderings

We this with have now finished the locking mechanism for the Mutex. This means the core functionality of the Mutex is complete. We now need to implement the appropriate traits for our Mutex.

Full code at this point:











