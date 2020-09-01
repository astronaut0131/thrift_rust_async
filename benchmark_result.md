## Test 1:
###config
|  thread num   | loop num  | total call |
|  -----------  | --------  | ---------- |
|      10      |    10_000    |    100_000    |

###sync
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    15210 ms  |        6574        |    830 us   |    144 us   |     149 us    |    153 us    |   182 us   |   317  us  |   13674941  us  |

###async
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    10256 ms  |        9750        |    1007 us   |    1086 us   |     1596 us    |    1725 us    |   2049 us   |   2410  us  |   4673  us  |


*************************************************************
## Test 2:
###config
|  thread num   | loop num  | total call |
|  -----------  | --------  | ---------- |
|      100      |    10_000    |    1_000_000    |

###sync
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    152202 ms  |        6570        |    7661 us   |    144 us   |     146 us    |    150 us    |   221 us   |   351  us  |   150669072  us  |

###async
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    112540 ms  |        8885        |    11218 us   |    11028 us   |     15090 us    |    16899 us    |   22755 us   |   33760  us  |   104415  us  |

*************************************************************
## Test 3:
###config
|  thread num   | loop num  | total call |
|  -----------  | --------  | ---------- |
|      1_000      |    1_000    |    1_000_000    |

###sync
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    153073 ms  |        6532        |    76748 us   |    144 us   |     146 us    |    154 us    |   241 us   |   26965  us  |   152795352  us  |

###async
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    106610 ms  |        9379        |    105667 us   |    85565 us   |     210787 us    |    238960 us    |   310654 us   |   417691  us  |   475449  us  |


*************************************************************
## Test 4:
###config
|  thread num   | loop num  | total call |
|  -----------  | --------  | ---------- |
|      100      |    1_000    |    100_000    |

###sync
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    15257 ms  |        6554        |    7729 us   |    144 us   |     146 us    |    153 us    |   226 us   |   1209  us  |   15084316  us  |

###async
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    10365 ms  |        9647        |    10312 us   |    10227 us   |     13542 us    |    14727 us    |   17748 us   |   23016  us  |   29674  us  |

*************************************************************
## Test 5:
###config
|  thread num   | loop num  | total call |
|  -----------  | --------  | ---------- |
|      50      |    1_000    |    50_000    |

###sync
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    7605 ms  |        6574        |    3877 us   |    144 us   |     146 us    |    148 us    |   231 us   |   1090  us  |   7441923  us  |

###async
|  total time |   query per second  |  avg time   |  per 50 time |  per 90 time |  per 95 time |  per 99 time | per 99.9 time | max time |
|  ---------  |   ----------------  | ----------  | ------------ | ------------ | ------------ | ------------ | ----------- |  -------- |
|    4932 ms  |        10137        |    4906 us   |    4916 us   |     6362 us    |    6743 us    |   7835 us   |   10896  us  |   21463  us  |



## Pressure Test
#### async
|  max thread num   |
|     ---------     |
|       5000        | 

#### sync
|  max thread num   |
|     ---------     |
|       2500        | 
