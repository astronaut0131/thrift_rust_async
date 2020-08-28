## 进展
1. 基本实现了frame transport, buffered transport, buffered protocol的异步调用
2. 实现了异步server
3. 手工改动了一个thrift生成文件，可以实现client-server调用

## 一些文件解释
1. original_tutorial.rs 是thrift自动生成的同步rs文件，用来做对比
2. tutorial.thrift 这是thrift idl文件
3. src/tutorial.rs 是手工修改后的异步rs文件
4. src/main.rs 用来做基本测试（test里面的测试多线程似乎不会及时输出）
5. src/client.rs 测试用的客户端，包括了transport, protocol和完整client测试

其他包与同步实现基本相同，只不过将其实现改为异步

## 运行方法
根目录下运行
```
cargo run --package thrift --bin thrift
```