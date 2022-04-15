# asm

This is a "hello world" project to play with Rust inline assembly.
Inline assembly was introduced with Rust 1.59 release (2022-02-24)
and this is an attempt to learn more about it.

Inspiration from this little project also comes from @iMilnb (Twitter)
and the C From Scratch video series and mainly this
show https://www.youtube.com/watch?v=F-Ow6-uH6Mc

The goal of the project is to show:
* How to call the `write` and `uname` syscall directly using assembly.
* How to use raw pointers to share data.
* Usage of unsafe in such cases.
* Implementation of the From trait to convert data from the C style
  structure to the Rust one. This part is shamefully more or less
  copied from the uname crate.

The project uses x86_64 assembly and is targeting a Linux x86_64
operating system. It means that **it will not be able to run** on
another platform or CPU.

Of course, tracing the program with `stace` shows
the syscalls explicitly requested with inline assembly :
```
execve("./target/debug/asm", ["./target/debug/asm"], 0x7fff86007bb0 /* 90 vars */) = 0
...
...
write(1, "Hello from: ", 12Hello from: )            = 12
write(1, "\360\237\246\211 Uggla !!!\n", 15🦉 Uggla !!!
) = 15
uname({sysname="Linux", nodename="ugglalaptop", ...}) = 0
...
...
exit_group(0)                           = ?
+++ exited with 0 +++
```
