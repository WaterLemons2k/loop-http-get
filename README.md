# Loop HTTP GET

Send an HTTP GET every 60 seconds.

English | [简体中文](README.zh-CN.md)

## Why

On some networks (e.g. hotspots), it will disconnect if there are no network requests for a period of time.

Therefore, the program sends an HTTP GET every 60 seconds to keep the network alive.

## Getting Started

```sh
$ ./loop-http-get
1 GET http://rust-lang.org
```