FROM ubuntu:16.04

RUN mkdir /app

ADD ./target/release/minion /app/

WORKDIR /app

ENTRYPOINT ["/app/minion", "-a", "192.168.100.14:6142"]

