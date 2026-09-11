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
