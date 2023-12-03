# beyla-rust

## basic
basic instrumentation of Grafana beyla to Rust actix-web

## total_request_times
Grafana beyla instrumentation and jeager instrumentation to Rust actix-web

Run jeager:

```bash
docker run -d -p4318:4318 -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```

Request server with traceparent
```bash
curl  http://localhost:8080/hello -H "traceparent: 00-1af92f3577b34da6a3ce929d0e0e0000-00f067aa0ba902a8-01"
```