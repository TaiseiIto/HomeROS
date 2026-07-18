FROM ubuntu:26.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get upgrade -y && apt-get install -y build-essential
RUN apt-get update && apt-get upgrade -y && apt-get install -y curl
RUN apt-get update && apt-get upgrade -y && apt-get install -y git
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-arm
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-misc
RUN apt-get update && apt-get upgrade -y && apt-get install -y qemu-system-x86
RUN apt-get update && apt-get upgrade -y && apt-get install -y tmux
RUN apt-get update && apt-get upgrade -y && apt-get install -y tzdata
RUN apt-get update && apt-get upgrade -y && apt-get install -y vim
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
WORKDIR /root/
RUN mkdir .github
RUN mkdir .ssh
ARG TIMEZONE
RUN cp $TIMEZONE /etc/localtime
ARG CACHE_BUSTER
RUN echo $CACHE_BUSTER
ARG DOMAIN
ARG DEVELOPER
ARG PRODUCT
RUN git clone https://$DOMAIN/$DEVELOPER/$PRODUCT.git
WORKDIR $PRODUCT
ARG BRANCH
RUN git checkout $BRANCH
RUN cat .tmux/conf >> ../.tmux.conf
RUN cat .vim/rc >> ../.vimrc
