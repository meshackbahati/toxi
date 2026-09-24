# Full HTTP Comparison

Loopback, oha 15 s, 50 connections. Identical routes in all four
servers. Success rate 1.0 everywhere. Requests per second.

| Route | Toxi | Axum | Actix | Rocket |
| ----- | ---- | ---- | ----- | ------ |
| GET /hello | 2,019 | 2,890 | 3,449 | 1,490 |
| GET /users/42 | 2,099 | 2,661 | 2,885 | 2,229 |
| POST /echo | 2,092 | 1,984 | 2,334 | 1,251 |
| GET /missing | 1,817 | 3,055 | 3,279 | 1,258 |

![throughput by route](img/http.png)

Toxi leads Rocket on all four routes and Axum on echo. Actix leads
every route. Measured on a saturated shared box; dedicated-machine
figures replace these when available.
