# SpinLock

A spinlock protects shared data so only one piece of code can use it at a time.

```text
Code tries to lock WRITER
        |
        v
Is the lock available?
  Yes -> take the lock and use WRITER
  No  -> repeatedly check until its owner releases it
```

The repeated checking is called **spinning**. The waiting CPU keeps running the
check loop; it does not sleep or give its time to another thread.

The `spin` crate provides synchronization types such as `spin::Mutex`. In this
OS, a spinlock protects the global VGA writer so two pieces of code do not try
to change the screen buffer at the same time.

## Why `lazy_static!` is used

`WRITER` is global, but setting it up includes accessing VGA memory at
`0xb8000`. `lazy_static!` creates it when it is first used, instead of requiring
the value to be fully created at compile time.

## `Volatile`

VGA memory is hardware memory, so every screen read and write must really
happen. `Volatile<ScreenChar>` prevents the compiler from removing or
reordering those operations as normal RAM optimizations.

## `dyn Fn()` in the test runner

`Fn()` is Rust's trait for something you can call with no arguments and no
return value. A normal function can implement it:

```rust
fn first_test() {}
fn second_test() {}

first_test();
```

`dyn` means Rust uses the trait at runtime instead of one specific concrete
function type. This lets one list contain different test functions:

```rust
let tests: [&dyn Fn(); 2] = [&first_test, &second_test];
```

The test runner uses `&[&dyn Fn()]`: a borrowed list (`&[...]`) of references
to callable tests (`&dyn Fn()`). Rust puts every `#[test_case]` function in
this list, and the runner calls them one by one.
