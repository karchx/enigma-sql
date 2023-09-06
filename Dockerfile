ARG ARCH=amd64
FROM archlinux:latest

WORKDIR /karch

# Update the repositories
RUN	 pacman -Syyuu --disable-download-timeout --noconfirm

# Install base-devel
RUN	 pacman -S --disable-download-timeout --noconfirm base-devel nasm python3 vim curl rust rustup

RUN rustup default stable
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

ENV LANG=en_US.UTF-8

COPY ./*.rs /karch/

CMD ["/bin/bash"]