FROM ubuntu:26.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential
RUN apt-get update && apt-get upgrade -y && apt-get install -y curl
RUN apt-get update && apt-get upgrade -y && apt-get install -y git
RUN curl -1sLf 'https://dl.cloudsmith.io/public/evilmartians/lefthook/setup.deb.sh' | bash
RUN apt-get update && apt-get upgrade -y && apt-get install -y lefthook
RUN apt-get update && apt-get upgrade -y && apt-get install -y llvm
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-arm
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-riscv
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-x86
RUN apt-get update && apt-get upgrade -y && apt-get install -y tmux
RUN apt-get update && apt-get upgrade -y && apt-get install -y tzdata
RUN apt-get update && apt-get upgrade -y && apt-get install -y vim
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . $HOME/.cargo/env && rustup target add aarch64-unknown-none
RUN . $HOME/.cargo/env && rustup target add aarch64-unknown-uefi
RUN . $HOME/.cargo/env && rustup target add riscv64gc-unknown-none-elf
RUN . $HOME/.cargo/env && rustup target add x86_64-unknown-none
RUN . $HOME/.cargo/env && rustup target add x86_64-unknown-uefi
RUN git config --global alias.graph 'log --graph --date-order -C -M --pretty=format:"<%h> %ad [%an] %Cgreen%d%Creset %s" --all --date=short'
WORKDIR /root/
RUN echo 'export GPG_TTY=$(tty)' >> .bashrc
ARG TIMEZONE
RUN cp $TIMEZONE /etc/localtime
ARG DOMAIN
ARG DEVELOPER
ARG PRODUCT
RUN git clone https://$DOMAIN/$DEVELOPER/$PRODUCT.git
WORKDIR $PRODUCT
ARG BRANCH
RUN git checkout $BRANCH
RUN lefthook install
RUN ln -s $(realpath .docker/tmux/conf) ../.tmux.conf
RUN ln -s $(realpath .docker/vim/rc) ../.vimrc
