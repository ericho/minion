# minion (WIP)

A little monitoring program for your system. Intented to be small and efficient.

## Building

Clone this repository and run:

```
cargo build --release
```

## Running

Check the help for more details:

```
$ minion --help

minion 0.1.0
Erich Cordoba <erich.cm@yandex.com>

USAGE:
    minion --aggregator <aggr>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --aggregator <aggr>  
```

example:

```
minion --aggregator 192.168.100.14:1234"
```

Currently there's no an aggregator defined, so the 1234 port should be opened by another tool, like `nc`. 

## TODO

- [ ] Use only one `TcpStream::connect` for the entire program instead of one per sample rate.
- [ ] Enable the aggregator mode. This will let use the same binary for monitoring and aggregation.
- [ ] Define a configuration type to be used by every component of the program.
- [ ] Define a sample rate per sensor.

