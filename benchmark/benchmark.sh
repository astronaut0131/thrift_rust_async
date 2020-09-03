#!/bin/sh

# set environment $ASYNC_STD_THREAD_COUNT t0 $CPU_NUM
CPU_NUM=$(($(cat /proc/cpuinfo |grep "processor"|wc -l) * 2));
export ASYNC_STD_THREAD_COUNT=$CPU_NUM;
echo "SET ASYNC_STD_THREAD_COUNT = $ASYNC_STD_THREAD_COUNT";

## config parameter
RUN_CLIENT=true;
RUN_SERVER=true;
RUN_SYNC=true;
RUN_ASYNC=true;
RUN_ASYNC_TOKIO=false;
THREAD_NUM=200;
LOOP_NUM=1000;
ADDR=127.0.0.1:9090;
##

cargo run --color=always --release --package benchmark --bin benchmark \
 $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO $THREAD_NUM $LOOP_NUM $ADDR
