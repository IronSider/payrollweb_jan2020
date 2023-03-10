# Forked from https://github.com/paritytech/substrate/blob/efd54879ecbf8aac41fe809d31c7215ea3101241/.maintain/Dockerfile
#
# Note: We don't use Alpine and its packaged Rust/Cargo because they're too often out of date,
# preventing them from being used to build canyon/Polkadot.

FROM phusion/baseimage:0.11 as builder
LABEL maintainer="xuliuchengxlc@gmail.com"
LABEL description="This is the build stage for Canyon. Here we create the binary."

ENV DEBIAN_FRONTEND=noninteractive

ARG PROFILE=release
WORKDIR /canyon

COPY . /canyon

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	export PATH="$PATH:$HOME/.cargo/bin" && \
	rustup toolchain install nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly && \
	rustup default stable && \
	cargo build "--$PROFILE"

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11
LABEL maintainer="xuliuchengxlc@gmail.com"
LABEL description="This is the 2nd stage: a very small image where we copy the canyon binary."
ARG PROFILE=release

RUN mv /usr/share/ca* /tmp && \
	rm -rf /usr/share/*  && \
	mv /tmp/ca-certificates /usr/share/ && \
	useradd -m -u 1000 -U -s /bin/sh -d /canyon canyon && \
	mkdir -p /canyon/.local/share/canyon && \
	chown -R canyon:canyon /canyon/.local && \
	ln -s /canyon/.local/share/canyon /data

COPY --from=builder /canyon/target/$PROFILE/canyon /usr/local/bin

# checks
RUN ldd /usr/local/bin/canyon && \
	/usr/local/bin/canyon --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/bin /usr/sbin /usr/share/man

USER canyon
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

CMD ["/usr/local/bin/canyon"]
