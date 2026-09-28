# Auth

Medians, 100 samples. HS256 JWT creation with full verify roundtrip.

| Case | Median | Mean |
| ---- | ------ | ---- |
| JWT create | 3.78 µs | 4.17 µs |
| JWT create + verify | 10.24 µs | 10.77 µs |

![jwt cost](graphs/auth.png)
