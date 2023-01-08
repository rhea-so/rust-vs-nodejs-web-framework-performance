# 🔥 Rust Rocket vs Node.js Express Performance Test

## Source code

- [Rust Rocket](./rust-rocket)
- [Node.js Express](./nodejs-express)

## Tool

- [wrk](https://github.com/wg/wrk)

```sh
brew install wrk
```

### Machine

- MacBook Air (M1, 2020)
  - CPU: M1 7-Core GPU
  - RAM: 8GB

## Stress Test

### Test Case 1

```sh
wrk -d1m http://localhost:3000/
```

- time: 1 min
- threads: 2
- connections: 10

#### Node.js Express

1st try:

```
Running 1m test @ http://localhost:3000/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   462.16us  234.08us  16.59ms   88.31%
    Req/Sec    11.16k     0.96k   12.89k    67.30%
  1334744 requests in 1.00m, 329.68MB read
Requests/sec:  22207.82
Transfer/sec:      5.49MB
```

2nd try:

```
Running 1m test @ http://localhost:3000/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   486.64us  358.95us  21.98ms   94.99%
    Req/Sec    10.87k     1.28k   13.41k    76.04%
  1300356 requests in 1.00m, 321.19MB read
Requests/sec:  21636.63
Transfer/sec:      5.34MB
```

#### Rust Rocket

1st try:

```
Running 1m test @ http://localhost:3000/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    80.69us   28.53us   2.13ms   82.00%
    Req/Sec    54.48k     4.37k   63.80k    72.30%
  6515211 requests in 1.00m, 1.52GB read
Requests/sec: 108405.71
Transfer/sec:     25.85MB
```

2nd try:

```
Running 1m test @ http://localhost:3000/
  2 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    75.96us   26.42us   1.42ms   80.80%
    Req/Sec    57.88k     4.67k   84.19k    67.94%
  6916236 requests in 1.00m, 1.61GB read
Requests/sec: 115080.26
Transfer/sec:     27.44MB
```
