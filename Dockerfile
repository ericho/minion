FROM ubuntu:16.04

RUN mkdir /app

ADD ./target/release/minion /app/

WORKDIR /app

CMD ["/app/minion"]

