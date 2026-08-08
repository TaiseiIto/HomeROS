# HomeROS
Homemade Rust Operating System

## How to run it
Install the following softwares.

* Docker
* Git
* Rust

You should be a member of group `docker` to use docker without sudo.
And run the commands below.

```
/somewhere$ git clone https://github.com/TaiseiIto/HomeROS.git
/somewhere$ cd HomeROS
```

## Cargo commands

### Build HomeROS

```
/somewhere/HomeROS$ cargo xtask build
```

### Run HomeROS on QEMU

```
/somewhere/HomeROS$ cargo xtask run aarch64
```

or

```
/somewhere/HomeROS$ cargo xtask run riscv64
```

or

```
/somewhere/HomeROS$ cargo xtask run x64
```

Then you can connect to `localhost:5900` with VNC and operate HomeROS.

### Disassemble

```
/somewhere/HomeROS$ cargo xtask disassemble --package boot --arch aarch64
```

or

```
/somewhere/HomeROS$ cargo xtask disassemble --package boot --arch riscv64
```

or

```
/somewhere/HomeROS$ cargo xtask disassemble --package boot --arch x64
```

### Build development environment

```
/somewhere/HomeROS$ cargo xtask environment
```

This command builds development environment as a docker container and enter the container.

### Delete development environment

```
/somewhere/HomeROS$ cargo xtask environment delete
```

### Rebuild development environment

```
/somewhere/HomeROS$ cargo xtask environment rebuild
```

### Privilege development environment

If you have push permission to this repository, this command make you able to push commits in the development environment.

```
/somewhere/HomeROS$ cargo xtask environment privilege --gpg-key /path/to/gpgkey --ssh-key /path/to/sshkey
```

### Lint

```
/somewhere/HomeROS$ cargo xtask lint
```
