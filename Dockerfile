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
	apt-get dist-upgrade -y -o Dpkg::Options::="--force