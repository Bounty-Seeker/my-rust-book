# Poisoning errors

We now have implemented the code which will record if the inner data has possibly been poisoned and now need to inform the user about the possibility that the data has been poisoned

## Overview

The general plan is to change the output of our Mutex's `lock` method to return a new type TODO rename`LockResult` which is an enum with two members `MutexGuard` and `PoisonGuard` which is just a wrapper around a `MutexGuard`.
In the case of `try_lock` we return a `TryLockResult` which is an enum with three members `WillBlock`, `PoisonGuard` and a `MutexGuard`.

With this the user can pattern match on the output and respond to the poisoned value in the appropriate way

So lets implement it.

## Implementation

We can create the `PoisonGuard` as a simple wrapper quite easily








































