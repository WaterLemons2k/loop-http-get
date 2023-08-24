# 循环 HTTP GET

每分钟发送一次 HTTP GET。

[English](README.md) | 简体中文

## 为什么

在某些网络（如热点）上，如果一段时间内没有网络请求就会断开连接。

因此，该程序会每分钟发送一次 HTTP GET 以保持网络连接。

## 开始

```sh
$ ./loop-http-get
1 GET http://rust-lang.org
```