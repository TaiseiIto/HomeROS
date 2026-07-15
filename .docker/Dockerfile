FROM ubuntu:26.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get upgrade -y && apt-get install -y git
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-arm
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-misc
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-x86
RUN apt-get update && apt-get upgrade -y && apt-get install -y tmux
RUN apt-get update && apt-get upgrade -y && apt-get install -y tzdata
RUN apt-get update && apt-get upgrade -y && apt-get install -y vim
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
WORKDIR /root/
ARG TIMEZONE
RUN cp $TIMEZONE /etc/localtime
