# Anatomy of a rust program

In order to understand the more complex aspects of rust we must first analyse the different parts of a compiled rust program. By evaluating the capabilites and limitations of the compiled program we will be able to understand how the complex aspects of rust work and why they are needed.

A rust program can be thought about in five sections: TODO check Execution, Constant, Statics, Stack, Heap

In this chapter we will explore each section why we have them and why we don't have any other sections. Once you understand the intracancies it will allow you to understand the motivation behind issues such as lifetimes, unsafe code, compiler optimizations and other topics.
