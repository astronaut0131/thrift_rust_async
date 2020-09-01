******************************************
*        E-01 benchmark for rust rpc     *
*             Version : 0.1.0            *
******************************************

# 目标
* 测试async和sync版本的rust thrift rpc性能，自动生成类似如下的数据结果


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


* 实现通过配置的方式独立或同时运行server和client，独立或同时测试sync和async

# 使用方法
直接运行cargo run即可在本地测试async和sync版本rpc的性能，如果需要client与service单独运行，请看参数配置章节


# 参数配置
所有参数均配置在src/main.rc的const变量中，下面解释一下每一个参数的意义

* const THREAD_NUM: i32 = 50;

客户端线程数量

* const LOOP_NUM: i32 = 1000;

每个客户端执行调用次数

* const RUN_CLIENT: bool = true;

是否运行客户端

* const RUN_SERVER: bool = true;

是否运行服务端

* const RUN_SYNC: bool = true;

是否运行SYNC rpc的部分

* const RUN_SYNC: bool = true;

是否运行ASYNC rpc的部分

* const ADDR: &str = "127.0.0.1:9090";

rpc连接的地址和端口

# 提醒事项
1. 当单纯运行server时，请指明运行ASYNC还是SYNC模块，因为server运行起来就会block等待客户端，不会同时运行两个server，
这是因为单独运行服务端时，同时测试两个服务端会互相影响，同时运行是没有意义的


# 包说明
* async_thrift_test

包含了async thrift的测试代码，client为客户端，server为服务端，其他为thrift自动生成的文件

* sync_thrift_test

包含了sync thrift的测试代码，client为客户端，server为服务端，其他为thrift自动生成的文件

* main.rs

benchmark主程序，负责启动client与server的启动，整体测试流程等

* until.rs

包含各种辅助函数，负责时间数据的统计和格式化输出

