export ASYNC_STD_THREAD_COUNT=128

## config parameter
RUN_CLIENT=true
RUN_SERVER=true;
RUN_SYNC=true;
RUN_ASYNC=true;
RUN_ASYNC_TOKIO=true;
THREAD_NUM=200
LOOP_NUM=100
ADDR=127.0.0.1:9090;
##

cargo run --color=always --release --package benchmark --bin benchmark \
 $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO $THREAD_NUM $LOOP_NUM $ADDR