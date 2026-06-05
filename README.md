An Ubuntu VM is needed for development as the `clone` syscall is only supported on Linux.


Using OrbStack, a VM can be created and used as follows from this project directory.
```
orb create ubuntu rust-dev
```

```
orb -m rust-dev
```

```
<follow the steps to install Rust using rustup>
```

This opens a shell into the Ubuntu VM, then we can build and run it using 
`cargo build` and `./target/debug/container-runtime -- [ARGS]`
