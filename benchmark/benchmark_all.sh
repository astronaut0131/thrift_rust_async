export ASYNC_STD_THREAD_COUNT=128

## config parameter
RUN_CLIENT=true;
RUN_SERVER=true;
RUN_SYNC=false;
RUN_ASYNC=true;
RUN_ASYNC_TOKIO=false;
THREAD_NUM=10
LOOP_NUM=10
ADDR=10.108.21.57:9090;
PROG=../target/release/benchmark
##

# cargo run --color=always --release --package benchmark --bin benchmark \
# $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO $THREAD_NUM $LOOP_NUM $ADDR
cargo build --color=always --release --package benchmark --bin benchmark

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 16 1000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 16 2000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 16 5000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 16 10000 $ADDR


#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 32 1000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 32 2000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 32 5000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 32 10000 $ADDR


#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 64 1000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 64 2000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 64 5000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 64 10000 $ADDR


#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 128 1000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 128 2000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 128 5000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 128 10000 $ADDR


#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 256 1000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 256 2000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 256 5000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 256 10000 $ADDR


#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 512 1000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 512 2000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 512 5000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 512 10000 $ADDR


#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 1024 1000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 1024 2000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 1024 5000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 1024 10000 $ADDR


#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 2048 1000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 2048 2000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 2048 5000 $ADDR

#cargo run --color=always --release --package benchmark --bin benchmark \
 $PROG $RUN_CLIENT $RUN_SERVER $RUN_SYNC $RUN_ASYNC $RUN_ASYNC_TOKIO 2048 10000 $ADDR
