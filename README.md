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
/somewhere/HomeROS$ cargo benv
~/HomeROS#
```

## Cargo commands

### Build development environment

```
/somewhere/HomeROS$ cargo benv
```

This command builds development environment as a docker container and enter the container.

### Deleve development environment

```
/somewhere/HomeROS$ cargo denv
```

### Rebuild development environment

```
/somewhere/HomeROS$ cargo renv
```

### Privilege 

```
/somewhere/HomeROS$ cargo penv --gpg-key /path/to/gpgkey --ssh-key /path/to/sshkey
```

If you have push permission to this repository, this command make you able to push commits in the development environment.

### Lint

```
~/HomeROS# cargo lint
```

