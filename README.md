# rs
​	

# 使用thrift生成rust代码



## 准备工作

##### 1.下载Thrift编译器：https://thrift.apache.org/

##### 2.在Cargo.toml中添加以下crate

```
thrift = "x.y.z" # x.y.z 是thrift编译器的版本
ordered-float = "0.3.0"
try_from = "0.2.0"
```

##### 3.在 lib.rs 或者main.rs中添加以下crate

```
extern crate ordered_float;
extern crate thrift;
extern crate try_from;
```

##### 4.给IDL（比如：Tutorial.thrift）生成rust源码

```
thrift -out my_rust_program/src --gen rs -r Tutorial.thrift
```

##### 5.使用生成的源码

```
// add extern crates here, or in your lib.rs
extern crate ordered_float;
extern crate thrift;
extern crate try_from;

// generated Rust module
mod tutorial;

use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::protocol::{TInputProtocol, TOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport};
use thrift::transport::{TIoChannel, TTcpChannel};

use tutorial::{CalculatorSyncClient, TCalculatorSyncClient};
use tutorial::{Operation, Work};

fn main() {
    match run() {
        Ok(()) => println!("client ran successfully"),
        Err(e) => {
            println!("client failed with {:?}", e);
            std::process::exit(1);
        }
    }
}

fn run() -> thrift::Result<()> {
    //
    // build client
    //

    println!("connect to server on 127.0.0.1:9090");
     let mut c = TTcpChannel::new();
    c.open("127.0.0.1:9090")?;

    let (i_chan, o_chan) = c.split()?;
    
    let i_prot = TCompactInputProtocol::new(
        TFramedReadTransport::new(i_chan)
    );
    let o_prot = TCompactOutputProtocol::new(
        TFramedWriteTransport::new(o_chan)
    );

    let mut client = CalculatorSyncClient::new(i_prot, o_prot);

    //
    // alright! - let's make some calls
    //

    // two-way, void return
    client.ping()?;

    // two-way with some return
    let res = client.calculate(
        72,
        Work::new(7, 8, Operation::Multiply, None)
    )?;
    println!("multiplied 7 and 8, got {}", res);
       // two-way and returns a Thrift-defined exception
    let res = client.calculate(
        77,
        Work::new(2, 0, Operation::Divide, None)
    );
    match res {
        Ok(v) => panic!("shouldn't have succeeded with result {}", v),
        Err(e) => println!("divide by zero failed with {:?}", e),
    }

    // one-way
    client.zip()?;

    // done!
    Ok(())
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

------

