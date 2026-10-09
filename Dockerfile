FROM debian:trixie

# Install cURL
RUN apt update
RUN apt install -y --no-install-recommends curl ca-certificates xz-utils
RUN rm -rf /var/lib/apt/lists/*

# Install Oxish

RUN mkdir -pv /root/Downloads
WORKDIR /root/Downloads

RUN curl -LO https://github.com/djc/oxish/releases/download/0.1.0/oxish-x86_64-unknown-linux-gnu.tar.xz
RUN tar xvf oxish-x86_64-unknown-linux-gnu.tar.xz

RUN install -vm755 oxish-x86_64-unknown-linux-gnu/oxish-server  /usr/local/bin
RUN install -vm755 oxish-x86_64-unknown-linux-gnu/oxish-session /usr/local/bin

WORKDIR /
RUN rm -rv /root/Downloads

# Configure Oxish
RUN oxish-server --generate-host-key

# Setup builder user
RUN useradd -m builder -s /bin/bash

RUN mkdir -pv /home/builder/.ssh
RUN printf '%s\n' 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDV2Q4h6zhEwosGAYCDoFfFPT2ydfiswItQjLIY5Xnn9 builder' > /home/builder/.ssh/authorized_keys

RUN chown builder:builder /home/builder/.ssh/authorized_keys
RUN chmod 600 /home/builder/.ssh/authorized_keys

RUN mkdir -pv /home/builder/work

ENTRYPOINT ["/usr/local/bin/oxish-server", "--port", "22"]
