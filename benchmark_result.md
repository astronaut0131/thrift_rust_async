## Test 1:
### config
|  Thread num   | Loop num  | Total call |
|  ----  | ----  | ---- |
| 10  | 10000 | 100_000 |

## result
#### sync: 
|  total time | time_per_call |
|  ----  | ---- | 
|  14533 ms|  145.33 us |
#### async: 
|  total time | time_per_call |
|  ----  | ---- | 
|  8987 ms| 89.87 us |


*************************************************************
## Test 2:
### config
|  Thread num   | Loop num  | Total call |
|  ----  | ----  | ---- |
| 100  | 10000 | 1_000_000 |

## result
#### sync: 
|  total time | time_per_call |
|  ----  | ---- |
| 144671 ms| 144.671 us|
#### async: 
|  total time | time_per_call |
|  ----  | ---- |
| 109596 ms|  109.596 us |

*************************************************************
## Test 3:
### config
|  Thread num   | Loop num  | Total call |
|  ----  | ----  | ---- |
| 1000  | 1000 | 1_000_000 |

## result
#### sync: 
|  total time | time_per_call |
|  ----  |  ----  |
| 91999 ms | 91.574 us |
#### async: 
|  total time | time_per_call |
|  ----  | ---- |
| 15986 ms | 15.986 us |


