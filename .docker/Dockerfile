FROM ubuntu:26.04
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential
RUN apt-get update && apt-get upgrade -y && apt-get install -y curl
RUN apt-get update && apt-get upgrade -y && apt-get install -y gcc-aarch64-linux-gnu
RUN apt-get update && apt-get upgrade -y && apt-get install -y gdb
RUN apt-get update && apt-get upgrade -y && apt-get install -y git
RUN apt-get update && apt-get upgrade -y && apt-get install -y iasl
RUN apt-get update && apt-get upgrade -y && apt-get install -y libglib2.0-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y libpixman-1-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y libssl-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y llvm
RUN apt-get update && apt-get upgrade -y && apt-get install -y nasm
RUN apt-get update && apt-get upgrade -y && apt-get install -y ninja-build
RUN apt-get update && apt-get upgrade -y && apt-get install -y python3-venv
RUN apt-get update && apt-get upgrade -y && apt-get install -y systemd
RUN apt-get update && apt-get upgrade -y && apt-get install -y tmux
RUN apt-get update && apt-get upgrade -y && apt-get install -y tzdata
RUN apt-get update && apt-get upgrade -y && apt-get install -y uuid-dev
RUN apt-get update && apt-get upgrade -y && apt-get install -y vim

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . $HOME/.cargo/env && cargo install cargo-expand

# Install QEMU
WORKDIR /root/
RUN git clone --branch v11.0.0 --depth 1 --recursive --shallow-submodules --single-branch https://gitlab.com/qemu-project/qemu.git
WORKDIR qemu
RUN ./configure --target-list=aarch64-softmmu,riscv64-softmmu,x86_64-softmmu CFLAGS="-O0 -g -fno-inline" CXXFLAGS="-O0 -g -fno-inline"
RUN make
RUN make install
WORKDIR ..

# Install EDK2
RUN git clone --branch edk2-stable202605 --depth 1 --recursive --shallow-submodules --single-branch https://github.com/tianocore/edk2.git
WORKDIR edk2
RUN make -C BaseTools
RUN . ./edksetup.sh && build -a X64 -b DEBUG -p OvmfPkg/OvmfPkgX64.dsc -t GCC
WORKDIR ..

# Install TF-A
RUN git clone --branch v2.15.0 --depth 1 --recursive --shallow-submodules --single-branch https://github.com/ARM-software/arm-trusted-firmware.git
WORKDIR arm-trusted-firmware
RUN make CROSS_COMPILE=aarch64-linux-gnu- PLAT=qemu DEBUG=1
RUN make PLAT=qemu DEBUG=1 fiptool
WORKDIR ..

# Git settings
RUN git config --global alias.graph 'log --graph --date-order -C -M --pretty=format:"<%h> %ad [%an] %Cgreen%d%Creset %s" --all --date=short'
RUN echo 'export GPG_TTY=$(tty)' >> .bashrc

# Set VNC port
ENV VNC_PORT=5900
EXPOSE $VNC_PORT

# Set timezone
ARG TIMEZONE
RUN ln -fs $TIMEZONE /etc/localtime

# Clone this repository
ARG DOMAIN
ARG DEVELOPER
ARG PRODUCT
RUN git clone https://$DOMAIN/$DEVELOPER/$PRODUCT.git
WORKDIR $PRODUCT
ARG BRANCH
RUN git checkout $BRANCH

# Install Lefthook
RUN curl -1sLf 'https://dl.cloudsmith.io/public/evilmartians/lefthook/setup.deb.sh' | bash
RUN apt-get update && apt-get upgrade -y && apt-get install -y lefthook
RUN lefthook install

# Development environment settings
RUN ln -s $(realpath .docker/tmux/conf) ../.tmux.conf
RUN ln -s $(realpath .docker/vim/rc) ../.vimrc
