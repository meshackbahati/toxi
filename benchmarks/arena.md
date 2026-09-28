# HTTP Arena Comparison

Published Alpha Round figures from http-arena.com against the two of
the six present there, Rocket and Salvo. Same machine, same harness,
same round.

## Toxi vs Salvo — Toxi leads all six shared profiles

| Profile | Toxi | Salvo |
| ------- | ---: | ----: |
| baseline | 293,837 | 282,378 |
| async | 151,542 | 133,927 |
| json-comp 4096 | 194,761 | 117,978 |
| json-comp 16384 | 194,897 | 123,869 |
| json-tls | 497,756 | 260,856 |
| 8gbit | 49,081 | 48,852 |

## Toxi vs Rocket — split decision

| Profile | Toxi | Rocket |
| ------- | ---: | -----: |
| baseline | 293,837 | 668,747 |
| pipelined | 2,723,159 | 1,266,201 |
| async | 151,542 | 393,572 |
| json-comp 4096 | 194,761 | 387,157 |
| json-comp 16384 | 194,897 | 387,669 |
| json-tls | 497,756 | 533,045 |
| 8gbit | 49,081 | 48,705 |

Toxi takes pipelined by 2.2× and ties 8gbit; Rocket takes baseline,
async, and json-comp by about 2–2.6×, json-tls by 7%.
