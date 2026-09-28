# Responses

Medians, 100 samples.

| Case | Median | Mean |
| ---- | ------ | ---- |
| JSON small struct | 0.24 µs | 0.25 µs |
| JSON medium value | 0.64 µs | 0.67 µs |
| JSON 1000 rows | 263.7 µs | 260 µs |
| text 1 KB | 0.21 µs | 0.25 µs |
| HTML page | 0.34 µs | 0.34 µs |
| empty ok | 0.12 µs | 0.13 µs |
| error to 404 | 1.46 µs | 1.55 µs |

![response construction](graphs/responses.png)
