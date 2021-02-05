# Summary

[Rust in detail]()
[What is this book](Prefix/what_is_this_book.md)


- [Anatomy of a Rust Program](ch01/intro_anatomy.md)
  - [Overview](ch01/overview.md)
  - [Run](ch01/exec.md)
  - [Constant](ch01/constant.md)
  - [Statics](ch01/statics.md)
  - [Stack](ch01/stack.md)
  - [Heap](ch01/heap.md)
  - [Registers]()
  - [Complications, closures , red zone, scratch pad]()
  - [&str vs String]()
  - [Function calls, ABIs](ch01/function_calls.md)
  - [hardware vs compiler](ch01/hardware_vs_compiler.md)


- [Lifetimes](ch02/intro_lifetimes.md)
  - [Rules]()
  - [How rustc does borrow checking]()
  - [Lifetimes in type signatures]()
  - [initialzing]()
  - [splitting borrows]()
  - [Phantom data]()


- [Unsafe](ch03/intro_unsafe.md)
  - [Undefined behaviour]()
  - [valid data]()
  - [Unsafe functions]()
  - [Implications]()
  - [Traits]()
  - [ptrs vs references]()
  - [Examples]()
  - [FFIs]()
  - [Allocator]()
  - [unbound lifetime]()
  - [unititalized Maemory radje blog]()
  - [transmute]()
  - [Unsafe tips]()
  - [Casts]()


- [Types](ch04/intro_types.md)
  - [repr]()
  - [Intresting Type stuff]()
  - [Chalk]()
  - [!]()
  - [Function Pointers vs Function Items]()
  - [Print type sizes]()
  - [extern types]()
  - [ZSTs]()
  - [Empty Types]()
  - [Thin wrapper]()
  
- [Type sizedness](ch05/intro_type_sizedness.md)
  - [&[]]()
  - [Pointer and size]()
  - [dyn type]()
  - [DSTs]()
  - [Heap vs Stack]()
  - [?sized]()
 
  
- [Concurrency](ch06/intro_concur.md)

- [async](ch07/intro_async.md)
  - [Pin]()
  
- [FFIs](ch08/intro_FFIs.md)
  - [C types]()
  - [Helpful tools]()

- [miri](ch09/intro_miri.md)

- [constant eval](ch10/intro_const_eval.md)
  - [miri]()
  - [Floating point inaccuracies]()

- [Iterators](ch11/intro_iterators.md)
  - [traits]()
  - [intro_iter() vs iter() bs iter_mut()]()
  - [Other iterator traits]()
  - [Stream iterator]()
  
- [Generics](ch12/intro_generics.md)
  - [Const Generics]()
  - [Generic Associated Types]()
  
- [Higher kindeness](ch13/intro_high_kind.md)
  - [Intro]()
  - [Generic Types]()
  - [Lifetimes]()
  - [Traits]()
  - [HRTBs]()
  - [Const Generics]()
  - [Generic Associated Types]()
  
- [Cows](ch14/intro_cows.md)

- [Heap](ch15/intro_heap.md)
  - [Allocator]()

- [Implement Box](ch16/intro_impl_box.md)

- [Implement Vec](ch17/intro_impl_vec.md)


- [Interior Mutability](ch18/intro_inter_mut.md)
  - [Exterior vs Interior mutability]()
  - [Unsafe_cell]()
  
  
- [Implement Cell](ch19/intro_impl_cell.md)

- [Implement Rc](ch20/intro_impl_rc.md)

- [Implement Refcell](ch21/intro_impl_refcell.md)

- [Implement Mutex](ch22/intro_impl_mutex.md)
  - [Creating an Initial Layout](ch22/impl_mutex_initial_layout.md)
  - [Implementing MutexGuard](ch22/impl_mutex_guard.md)
  - [The UnsafeCell type](ch22/impl_mutex_unsafe_cell.md)
  - [Implementing locking for our Mutex](ch22/impl_mutex_locking.md)
  - [Should our mutex be ?Sized, Sync, Send](ch22/impl_mutex_sized_sync_send.md)
  - [A fully working Mutex](ch22/impl_mutex_review_no_poison.md)
  - [Poisoning in our Mutex](ch22/impl_mutex_poison.md)
  - [Managing what happens when poisoning occurs](ch22/impl_mutex_poison_error.md)
  - [A working Mutex With poisoning](ch22/impl_mutex_final.md)
  - [Manually Drop]()

- [Rust Issues]()
  - [How to read]()

- [Future Rust Improvements]()

- [Undefined behavior]()

- [Conventions]()

- [Macros]()

- [std intresting stuff]()

- [Beneath std]()
  - [Setup to program]()
  - [PHil OS stuff raised]()
  - [ABIs]()
  - [Red_zone]()
  - [inline]()
  
- [rustc tour]()
  - [lang items]()
  - [librification]()
  - [NLL vs polonius]()
  - [datalog language used for polonius]()
  - [trait type checker]()
  - [llvm]()
  
- [Formal proof checked stuff]()
  - [Stacked borrows]()

- [Closure]()
  - [syntatic sugar]()
  - [move]()
  - [Fn, FnMut, FnOnce]()
  - [threads]()

- [Intresting traits]()
  - [From and Into]()
  - [Borrow]()
  - [Borrow_mut]()
  - [As_ref]()
 
- [Intresting Crates]()
  - [Nom]()
  - [Serde]()
  
- [Blogs]()
  
- [Coding Challenges]()

- [Appendix]()
 

