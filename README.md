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

<img width="611" alt="image" src="https://user-images.githubusercontent.com/25793226/211211882-d88ae01d-a912-44d2-9974-0f919cf05a40.png">

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

### Test Case 2

```sh
wrk -d1m -t10 -c50 http://localhost:3000/
```

- time: 1 min
- threads: 10
- connections: 50

#### Node.js Express

1st try:

```
Running 1m test @ http://localhost:3000/
  10 threads and 50 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.27ms  615.09us  39.37ms   88.41%
    Req/Sec     2.22k   162.95     4.65k    83.20%
  1328737 requests in 1.00m, 328.20MB read
Requests/sec:  22123.45
Transfer/sec:      5.46MB
```

2nd try:

```
Running 1m test @ http://localhost:3000/
  10 threads and 50 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.29ms  476.25us  11.10ms   87.09%
    Req/Sec     2.20k   143.19     3.58k    77.15%
  1313219 requests in 1.00m, 324.37MB read
Requests/sec:  21867.75
Transfer/sec:      5.40MB
```

#### Rust Rocket

1st try:

```
Running 1m test @ http://localhost:3000/
  10 threads and 50 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     0.93ms    5.08ms 186.05ms   98.06%
    Req/Sec    13.31k     1.84k   72.72k    84.49%
  7948209 requests in 1.00m, 1.85GB read
Requests/sec: 132248.44
Transfer/sec:     31.53MB
```

2nd try:

```
Running 1m test @ http://localhost:3000/
  10 threads and 50 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   840.35us    4.29ms 154.68ms   98.25%
    Req/Sec    11.95k     1.81k   27.75k    80.23%
  7135836 requests in 1.00m, 1.66GB read
Requests/sec: 118829.48
Transfer/sec:     28.33MB
```

### Test Case 3

```sh
wrk -d1m -t10 -c1000 http://localhost:3000/
```

- time: 1 min
- threads: 10
- connections: 1000

### Node.js Express

1st try:

```
Running 1m test @ http://localhost:3000/
  10 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    50.13ms    5.72ms 169.15ms   87.38%
    Req/Sec     2.00k   179.98     2.38k    84.41%
  1194619 requests in 1.00m, 295.07MB read
  Socket errors: connect 0, read 5133, write 1, timeout 0
Requests/sec:  19876.74
Transfer/sec:      4.91MB
```

2nd try:

```
Running 1m test @ http://localhost:3000/
  10 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    52.15ms    6.62ms 149.70ms   87.90%
    Req/Sec     1.92k   235.80     2.69k    80.44%
  1148468 requests in 1.00m, 283.67MB read
  Socket errors: connect 0, read 3456, write 4, timeout 0
Requests/sec:  19108.54
Transfer/sec:      4.72MB
```

### Rust Rocket

1st try:

```
Running 1m test @ http://localhost:3000/
  10 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.83ms   23.85ms 277.63ms   90.75%
    Req/Sec    12.77k     1.90k   30.22k    79.16%
  7608955 requests in 1.00m, 1.77GB read
  Socket errors: connect 0, read 3258, write 1, timeout 0
Requests/sec: 126608.28
Transfer/sec:     30.19MB
```

2nd try:

```
Running 1m test @ http://localhost:3000/
  10 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    14.69ms   22.05ms 253.00ms   90.26%
    Req/Sec    12.06k     1.68k   33.53k    78.74%
  7190400 requests in 1.00m, 1.67GB read
  Socket errors: connect 0, read 3328, write 0, timeout 0
Requests/sec: 119672.08
Transfer/sec:     28.53MB
```
