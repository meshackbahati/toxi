# Full HTTP Comparison

GET /json, oha 30 s, 125 connections, loopback, release builds.
Success rate 1.0 everywhere.

## Baseline shootout (preserved)

| Framework | Req/s | p99 (ms) |
| --------- | ----: | -------: |
| toxi | 40,812 | 8.60 |
| warp | 40,681 | 11.51 |
| poem | 40,359 | 8.44 |
| loco | 28,994 | 10.49 |
| salvo | 27,759 | 33.78 |
| rocket | 23,207 | 36.34 |

## Tuned campaign (forward + reverse order, drift-corrected mean)

| Framework | Forward | Reverse | Mean |
| --------- | ------: | ------: | ---: |
| toxi | 33,715 | 17,137 | 25,426 |
| rocket | 12,715 | 16,218 | 14,466 |
| poem | 15,699 | 12,441 | 14,070 |
| warp | 15,201 | 12,443 | 13,822 |
| salvo | 14,608 | 12,340 | 13,474 |
| loco | 11,963 | 11,881 | 11,922 |

Requests per second. Each direction ran all six back-to-back; the mean
cancels the load drift that flatters whoever runs first. Four-route
detail (hello, user, missing, echo) in the previous section.

![throughput with p99](img/six-http.png)

## Sustained load (5 min, /json, 50 conns)

| Framework | Req/s | p50 (ms) | p99 (ms) |
| --------- | ----: | -------: | -------: |
| warp | 38,850 | 0.88 | 10.72 |
| salvo | 27,115 | 1.17 | 15.30 |
| loco | 23,924 | 1.46 | 19.94 |
| toxi | 19,547 | 1.45 | 26.91 |
| poem | 9,808 | 3.44 | 53.34 |
| rocket | 6,974 | 4.39 | 79.72 |

No errors on any framework. Sprint ordering does not hold
sustained: warp and salvo pull ahead while poem and rocket fall back.
Raw output in `raw/sus-*.json`, rows in `results/sustained.csv`.
