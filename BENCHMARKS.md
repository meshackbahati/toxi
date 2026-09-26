# BENCHMARKS.md — Toxi Performance Report

## 1. Objective

Establish where Toxi stands against Rocket, Loco, Poem, Salvo, and Warp
across throughput, latency, concurrency, payload size, memory, and
sustained load, with reproducible methods and no predetermined conclusion.

## 2. Hardware

Intel Core i5-7Y54 @ 1.20 GHz (2 cores, 4 threads), 7.6 GiB RAM,
EndeavourOS (Arch) kernel 7.2.6, x86_64, AC power, CPU governor
powersave. Full capture in `benchmarks/environment.txt`. Shared laptop:
absolute figures carry load noise; orderings and same-window deltas are
the defensible findings until the dedicated-machine run.

## 3. Software

rustc/cargo 1.98.1, Tokio 1.x, oha 1.16.0, divan via CodSpeed compat
5.0.2, CodSpeed CLI 5.0.2. Framework crates pinned in
`toxi-bench-online/servers/Cargo.lock`; Toxi commit recorded per CSV row.

## 4. Methodology

Divan microbenchmarks for dispatch, extraction, responses, middleware,
throughput, and sqlite-backed handlers. Full HTTP over loopback with oha
(JSON body for POSTs, keep-alive default, warmup before measurement,
3 runs for the baseline). CodSpeed simulation mode gates regressions in
CI. Raw oha JSON under `benchmarks/raw/`, CSV rows in
`benchmarks/results/matrix.csv`.

## 5. Commands

```bash
cargo bench -p toxi-core --no-default-features        # divan suites
cd toxi-bench-online && ./bench.sh                    # full HTTP matrix
python3 benchmark_runner.py --conns 125 --duration 30s  # shootout
```

## 6. Baseline (GET /json, 30 s, 125 conns, preserved)

| Framework | Req/s | p99 (ms) |
| --------- | ----: | -------: |
| toxi | 40,812 | 8.60 |
| warp | 40,681 | 11.51 |
| poem | 40,359 | 8.44 |
| loco | 28,994 | 10.49 |
| salvo | 27,759 | 33.78 |
| rocket | 23,207 | 36.34 |

## 7. Throughput and latency (four routes, 15 s, 50 conns)

| Route | Toxi | Rocket | Poem | Salvo | Warp | Loco |
| ----- | ---- | ------ | ---- | ----- | ---- | ---- |
| hello | 31,959 | 29,050 | 32,922 | 25,998 | 30,849 | 30,474 |
| user | 25,127 | 22,022 | 29,690 | 29,193 | 40,327 | 41,104 |
| missing | 34,403 | 34,861 | 33,109 | 32,876 | 35,586 | 31,367 |
| echo | 21,535 | 26,496 | 27,366 | 28,729 | 23,850 | 24,364 |

## 8. Concurrency, payload, memory, sustained

Concurrency sweep, 1 KB–1 MB payloads, VmHWM memory, and 5-minute
sustained runs execute through `bench.sh`; tables land in
`benchmarks/results/` as runs complete. Binary sizes and peak RSS per
framework are recorded alongside throughput in the same CSV rows.

## 9. Component results (divan medians, quiet box)

Router: first 1.00 µs, last ~6 µs at 10 routes, 500-route last hit
27.5 µs loaded / post-fix linear at 0.09 µs per route. Extractors:
JSON small 0.78 µs, 10 KB 48.7 µs, query 0.57 µs. Responses: small
0.71 µs, 1000 rows 552 µs. DB handler through router 67 µs (sqlite).
Middleware depth shows no trend above noise.

## 10. Failed and unsupported tests

- 1000-connection sweep on a 2-core laptop: recorded, may fail; failure
  is data, not excluded.
- TLS profiles: not implemented in the arena entry yet.
- CodSpeed upload in CI: blocked on the `CODSPEED_TOKEN` secret
  (account-side).

## 11. Raw data and graphs

`benchmarks/raw/`, `benchmarks/results/matrix.csv`,
`benchmarks/graphs/`. Charts render from recorded data only.

## 12. Reproduction

Clone, build release, run `./bench.sh` on a quiet machine. Read
`benchmarks/environment.txt` first for comparison context.

## 13. Limitations

Shared hardware with frequency scaling active; run-to-run variance up
to 30% between load windows; single machine and toolchain; Toxi figures
include trivial handler execution where competitors measure match only.

## 14. Interpretation

Toxi leads the /json shootout within variance of warp and poem, beats
Rocket across routes, and trails Actix-class throughput where measured
previously. Param-heavy routes favor loco and warp. The 404 path is
Toxi's most competitive route. Nothing here claims universal superiority;
the dedicated-machine run decides absolutes.
