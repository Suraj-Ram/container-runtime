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
`cargo build` and `./target/debug/container-runtime [ARGS]`

#### Download Alpine root fs

```
mkdir -p ~/rootfs
cd ~/rootfs
curl -O https://dl-cdn.alpinelinux.org/alpine/v3.20/releases/x86_64/alpine-minirootfs-3.20.0-x86_64.tar.gz
mkdir alpine
tar -xzf alpine-minirootfs-3.20.0-x86_64.tar.gz -C alpine
```
