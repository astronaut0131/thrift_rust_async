# rs

## 本项目层级解释

```
├── go_client 				# 测试用golang客户端 
├── Cargo.lock
├── Cargo.toml
├── README.md
├── async_thrift			# 采用async-std实现的异步thrift 
│   ├── Cargo.toml	
│   └── src				# 源码
│       ├── autogen.rs			
│       ├── errors.rs
│       ├── lib.rs
│       ├── protocol			# 协议层， 支持了binary protocol
│       │   ├── async_binary.rs
│       │   └── mod.rs
│       ├── server			# 异步服务端
│       │   ├── asynced.rs
│       │   └── mod.rs
│       └── transport			# 传输层 提供了 buffered 以及 framed两种传输方式
│           ├── async_buffered.rs
│           ├── async_framed.rs
│           ├── async_socket.rs
│           └── mod.rs
├── async_thrift_tokio			# 采用tokio实现的异步thrift 
│   ├── Cargo.toml
│   └── src
│       ├── autogen.rs
│       ├── errors.rs
│       ├── lib.rs
│       ├── protocol
│       │   ├── async_binary.rs
│       │   └── mod.rs
│       ├── server
│       │   ├── asynced.rs
│       │   └── mod.rs
│       └── transport
│           ├── async_buffered.rs
│           ├── async_framed.rs
│           ├── async_socket.rs
│           └── mod.rs
├── async_thrift_uring			# 采用io-uring实现的异步thrift 
│   ├── Cargo.toml
│   └── src
│       ├── autogen.rs
│       ├── errors.rs
│       ├── lib.rs
│       ├── protocol
│       │   ├── async_binary.rs
│       │   └── mod.rs
│       ├── server
│       │   ├── asynced.rs
│       │   └── mod.rs
│       └── transport
│           ├── async_buffered.rs
│           ├── async_framed.rs
│           ├── async_socket.rs
│           └── mod.rs
├── benchmark				# 性能测试包 
│   ├── Cargo.toml
│   ├── README.md
│   ├── benchmark.sh
│   ├── benchmark_all.sh
│   └── src
│       ├── async_thrift_test		# async-std版性能测试
│       │   ├── bytes.thrift
│       │   ├── client.rs
│       │   ├── echo.rs
│       │   ├── mod.rs
│       │   ├── server.rs
│       │   └── tutorial.rs
│       ├── async_thrift_test_tokio	# tokio版性能测试
│       │   ├── client.rs
│       │   ├── mod.rs
│       │   ├── server.rs
│       │   └── tutorial.rs
│       ├── main.rs			# 测试主函数
│       ├── sync_thrift_test		# 同步版性能测试
│       │   ├── client.rs
│       │   ├── mod.rs
│       │   ├── server.rs
│       │   └── tutorial.rs
│       └── util.rs			# 测试打印依赖文件
└── benchmark_result.md
```
​	

# 使用thrift生成rust代码

## 准备工作

##### 1.下载并编译async_thrift编译器：https://github.com/guanjialin/compiler

##### 2.在Cargo.toml中添加以下crate

```
async_thrift = "x.y.z" # x.y.z 是async thrift编译器的版本
ordered-float = "0.3.0"
try_from = "0.2.0"
```

##### 3.在 lib.rs 或者main.rs中添加以下crate

```
extern crate ordered_float;
extern crate async_thrift;
extern crate try_from;
```

##### 4.编写IDL文件(with_struct.thrift)
```
namespace cl tutorial
namespace cpp tutorial
namespace d tutorial
namespace dart tutorial
namespace java tutorial
namespace php tutorial
namespace perl tutorial
namespace haxe tutorial
namespace netstd tutorial

struct Input {
  1: i32 num1 = 0,
  2: i32 num2,
  3: optional string comment,
}

struct Output{
	1: i32 res,
  	2: optional string comment,
}

service Calculator {
   Output add(1:Input param),
}
```

##### 5.给IDL（比如：Tutorial.thrift）生成rust源码

```
thrift -out my_rust_program/src --gen rs with_struct.thrift
```

##### 6.使用生成的源码(server部分)
```
use async_thrift::server;
use async_std::task;
use async_std::io::Error;
use async_thrift::transport::async_framed::{TAsyncFramedReadTransportFactory, TAsyncFramedWriteTransportFactory};
use async_thrift::protocol::async_binary::{TAsyncBinaryInputProtocolFactory, TAsyncBinaryOutputProtocolFactory};
use async_std::net::ToSocketAddrs;
use async_trait::async_trait;
use crate::async_thrift_test::with_struct::{CalculatorSyncProcessor, CalculatorSyncHandler, Input, Output};
use async_thrift::transport::async_buffered::{TAsyncBufferedReadTransportFactory, TAsyncBufferedWriteTransport, TAsyncBufferedWriteTransportFactory};
use async_std::fs::File;
use futures::{AsyncReadExt, AsyncWriteExt};

pub async fn run_server(addr: &str) {
    let processor = CalculatorSyncProcessor::new(PartHandler {});
    let r_trans_factory = TAsyncBufferedReadTransportFactory::new();
    let w_trans_factory = TAsyncBufferedWriteTransportFactory::new();
    let i_proto_factory = TAsyncBinaryInputProtocolFactory::new();
    let o_proto_factory = TAsyncBinaryOutputProtocolFactory::new();
    let mut s = server::asynced::TAsyncServer::new(r_trans_factory, i_proto_factory, w_trans_factory, o_proto_factory, processor);

    s.listen(addr).await;
}

struct PartHandler {}

#[async_trait]
impl CalculatorSyncHandler for PartHandler {
    async fn handle_add(&self, param: Input) -> async_thrift::Result<Output> {
        Ok(Output { res: Some(param.num1.unwrap() + param.num2.unwrap()), comment: None })
    }
}

```

##### 7.使用生成的源码(client部分)

```
use std::time::{SystemTime, UNIX_EPOCH};
use async_std::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    task,
};
use async_std::io::Error;

pub type Result<T> = std::result::Result<T, Error>;


use async_thrift::transport::async_socket::TAsyncTcpChannel;
use async_thrift::transport::async_framed::{TAsyncFramedWriteTransport, TAsyncFramedReadTransport};
use async_thrift::transport::{AsyncWrite, TAsyncIoChannel, AsyncReadHalf, AsyncWriteHalf};
use async_thrift::protocol::{TFieldIdentifier, TType};
use async_thrift::protocol::async_binary::{TAsyncBinaryOutputProtocol, TAsyncBinaryInputProtocol};
use async_thrift::protocol::TAsyncOutputProtocol;
use async_thrift::transport::async_buffered::{TAsyncBufferedReadTransport, TAsyncBufferedWriteTransport};
use time::Duration;
use futures::AsyncWriteExt;
use crate::async_thrift_test::with_struct::{CalculatorSyncClient, Input, TCalculatorSyncClient};
use thrift::transport::TTcpChannel;

pub async fn run_client(addr: impl ToSocketAddrs, loop_num: i32) -> async_thrift::Result<(Box<Vec<i64>>)> {
    // time
    // let start = time::now();

    let mut stream = TcpStream::connect(addr).await?;

    let mut c = TAsyncTcpChannel::with_stream(stream);

    let (i_chan, o_chan) = c.split().unwrap();

    let i_prot = TAsyncBinaryInputProtocol::new(
        TAsyncBufferedReadTransport::new(i_chan), true,
    );
    let o_prot = TAsyncBinaryOutputProtocol::new(
        TAsyncBufferedWriteTransport::new(o_chan), true,
    );

    let mut client = CalculatorSyncClient::new(i_prot, o_prot);

    let mut time_array = Vec::with_capacity(loop_num as usize);

    let mut sum = 0;
    for i in 0..loop_num {
        let before = time::now();
        let r = client.add(
            Input{
                num1: Some(1),
                num2: Some(2),
                comment: None
            }
        ).await?;
        let end = time::now();
        sum += r.res.unwrap();
        time_array.push((end - before).num_nanoseconds().unwrap());
    }

    c.close();
    
    Ok((Box::new(time_array)))
}
```



## 代码生成器

### Thrift文件和模块生成

Thrift编译器根据你的Thrift文件生成一个同名的Rust模块，举个例子：

如果你的文件是ThriftTest.thrift，编译器会生成一个thrift_test.rs

如果要使用生成的文件，在lib.rs 或者main.rs文件中加入以下声明：

（每个生成的文件都要加）mod... 和use...



### 结果和错误

Thrift的runtime库定义了`thrift::Result` 和 `thrift::Error` 类型，这两种类型在生成的源码里都可以使用，`thrift::Error`.的定义转换std::io::Error`, `str` and `String



### Thrift类型和对应的Rust类型

Thrift定义了一些类型，编译器会把他们转换成相应的Rust类型

- Primitives (bool, i8, i16, i32, i64, double, string, binary)
- Typedefs
- Enums
- Containers
- Structs
- Unions
- Exceptions
- Services
- Constants (primitives, containers, structs)

另外，除非有特别的说明，Thrift的include会被转换成use ,生成的源码中的声明，参数，trait，和类型都在对应的名空间。

以下的小节将介绍Thrift类型和对应的Rust类型

#### Primitives：基本类型

Thrift的基本类型有直接对应的Rust类型

- bool: `bool`
- i8: `i8`
- i16: `i16`
- i32: `i32`
- i64: `i64`
- double: `OrderedFloat<f64>`
- string: `String`
- binary: `Vec<u8>`



#### Typedefs：自定义类型

Typedef会被转换成 pub type

```
typedef i64 UserId

typedef map<string, UserId> MapType
```

```
pub type UserId = i64;

pub type MapType = BTreeMap<String, Bonk>;
```

#### Enums：枚举类型

Thrift的枚举类型和Rust的枚举一一对应

```
enum Numberz
{
    ONE = 1,
    TWO,
    THREE,
    FIVE = 5,
    SIX,
    EIGHT = 8
}
```

```
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Numberz {
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FIVE = 5,
    SIX = 6,
    EIGHT = 8,
}

impl TryFrom<i32> for Numberz {
    // ...
}
```



#### Containers：容器

Thrift有三种容器类型：list  ,set 和map,他们在Rust中对应Vec`, `BTreeSet` 和BTreeMap

任何的Thrift类型（包括结构体，枚举，自定义类型）都能做完list/set的元素，map的关键字和键值

List

```
list <i32> numbers
numbers: Vec<i32>
```

Set

```
set <i32> numbers
numbers: BTreeSet<i32>
```

Map

```
map <string, i32> numbers
numbers: BTreeMap<String, i32>
```

#### 	

#### Structs：结构体

Thrift结构体的每个域和Rust结构体的域一一对应

```
struct CrazyNesting {
    1: string string_field,
    2: optional set<Insanity> set_field,
    3: required list<
         map<set<i32>, map<i32,set<list<map<Insanity,string>>>>>
       >
    4: binary binary_field
}
```

```
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CrazyNesting {
    pub string_field: Option<String>,
    pub set_field: Option<BTreeSet<Insanity>>,
    pub list_field: Vec<
        BTreeMap<
            BTreeSet<i32>,
            BTreeMap<i32, BTreeSet<Vec<BTreeMap<Insanity, String>>>>
        >
    >,
    pub binary_field: Option<Vec<u8>>,
}

impl CrazyNesting {
    pub fn read_from_in_protocol(i_prot: &mut TInputProtocol)
    ->
    thrift::Result<CrazyNesting> {
        // ...
    }
    pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol)
    ->
    thrift::Result<()> {
        // ...
    }
}
```

#### 	Optionality

Thrift有三个可选类型

1. Required
2. Optional
3. Default

Rust的代码生成器会将Required类型编译成对应的裸类型，*Optional*  和 Default域编码成`Option<TypeName>`.

```
struct Foo {
    1: required string bar  // 1. required
    2: optional string baz  // 2. optional
    3: string qux           // 3. default
}

pub struct Foo {
    bar: String,            // 1. required
    baz: Option<String>,    // 2. optional
    qux: Option<String>,    // 3. default
}
```

## 已知问题

不支持结构体常量

Map ,List 和Set常量需要一个常量结构体包含

目前 async uring 代码还没有完成，暂时无法使用




