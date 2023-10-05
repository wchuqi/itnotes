# 前言 

Rust 是一门系统编程语言。

```
所谓系统编程，指的是编写： 
操作系统
各种设备驱动 
文件系统 
数据库 
运行在廉价设备或必须极端可靠设备上的代码 
加解密程序 
媒体编解码器（读写音频、视频和图片文件的软件） 
媒体处理器（如语音识别或图片编辑软件） 
内存管理程序（如实现垃圾收集器） 
文本渲染程序（将文本和字体转换为像素） 
高级编程语言（如 JavaScript 或 Python） 
网络程序 
虚拟化及软件容器 
科学模拟程序 
游戏 

简言之，系统编程是一种资源受限的编程。这种编程需要对每个字节和每个 CPU 时钟周期精打细算。
```

Rust 是由 Mozilla 和社区贡献者共同开发的一种新的系统编程语言。

**所有权（ownership）、转移 （move）和借用（borrow）机制**

# 入门

下载和安装Rust

rustup

```shell
是一个管理 Rust 安装包的工具，类似 Ruby 的 RVM 或者 Node 的 NVM.
$ rustup update  # 升级版本

$ cargo --version
cargo 1.34.0 (6789d8a0a 2019-04-01)
$ rustc --version
rustc 1.34.1 (fc50f328b 2019-04-24)
$ rustdoc --version
rustdoc 1.34.1 (fc50f328b 2019-04-24)
```

```
cargo 是 Rust 的编译管理器、包管理器以及通用工具。可以使用 Cargo 来创建新项目、构建和运行程序，以及管理代码所依赖的外部库。

rustc 是 Rust 编译器。通常是通过 Cargo 来调用编译器，但有时候也需要直接调用它。

rustdoc 是 Rust 文档工具。如果在代码注释中以适当格式写了文档，那么 rustdoc 可以基于它们生成格式化的 HTML。与rustc 类似，通常也让 Cargo 来帮助运行 rustdoc。
```

```shell
# 创建一个新的包目录 hello，其中 --bin 标记告诉 Cargo 将其作为一个可执行文件，而不是一个库。
$ cargo new --bin hello
 Created binary (application) `hello` project
# 命令行里加上 --cvs none，则创建出的项目不带 .gitignore 文件和 .git 目录

# cargo run 命令都可以构建并运行这个程序
$ cargo run
 Compiling hello v0.1.0 (file:///home/jimb/rust/hello)
 Finished dev [unoptimized + debuginfo] target(s) in 0.27
secs
 Running `/home/jimb/rust/hello/target/debug/hello`
Hello, world!
# Cargo 调用了 Rust 编译器 rustc，然后又运行了生成的可执行文件。Cargo 把可执行文件放到了包顶部的 target 子目录中。

# Cargo 清理生成的文件
$ cargo clean
$ ../target/debug/hello
bash: ../target/debug/hello: No such file or directory

```

4 个空格的缩进是 Rust 风格。

类型推断

```rust
let n = 1;
let n:u64 = 1;
```

用花括号括起来的任何代码块都可以看作一 个表达式。

Rust 语言本身内置了简单测试机制。

```rust
$ cargo test
Executing task: C:\Users\Administrator\.cargo\bin\cargo.exe test --package rust_test1 --bin rust_test1 -- test1 --exact --nocapture 

    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src\main.rs (target\debug\deps\rust_test1-596a29cf2bd8234b.exe)

running 1 test   
test test1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

测试函数可以写在源代码中的任何地方，只要紧跟着它要测试的代码即可。这样，cargo test 会自动把它们收集起来并全部运行。

命令行参数

`&x` 是借用对 x 的引 用，而 `*r` 是引用 r 所指向的值。

`cargo run` 命令允许向程序传递参数。

Rust 标准库。

在安装 Rust 的同时，rustup 命令也自动在你的计算机上安装了一份文档。

可以运行如下命令在浏览器中查阅标准库的文档： 

`$ rustup doc --std` 

在线文档地址见 Rust 官方网站。



可以从 crates.io 自由下载第三方依赖。

要在代码中使用 crates.io 上的某个包，cargo 命令可以搞定一切：下载正 确的版本、构建代码、必要时升级。Rust 包，无论是一个库还是可执行文件，都叫 crate（意思是”木板集装箱“）。Cargo（货船）和 crates.io 都是因此而得名的。

过按照约定，如果模块的名字叫 prelude，那就说明它所导出的特性是使用该包的任何用户都可能要用到的一些通用特性。

所有 Rust 函数都是线程安全的。



# 基本类型

Rust 在设计时没有考虑解析器或即时（JIT，Just In Time）编译器，而是选择了事先编译。

换句话说， Rust 程序在执行之前会完全被转换为机器码。

Rust 的类型有助于事前编译器为程序要操作的值选择更好的机器级表示，这些表示形式的性能是可预期的，因而程序员可以完全利用机器的能力。

Rust的类型

| 类型                                   | 说明                                                       | 值                                                           |
| -------------------------------------- | ---------------------------------------------------------- | ------------------------------------------------------------ |
| `i8、i16、i32、i64、u8、u16、u32、u64` | 给定位宽的有符号和无符号整数                               | `42、-5i8、0x400u16、0o100i16、20_922_789_888_000u64 字节字面量）` |
| `isize、usize`                         | 与机器 字（32 位或 64 位）同样大的有符号和无符号整数       | `137、-0b0101_0010isize、0xffff_fc00usizez`                  |
| `f32、f64`                             | 单精度、双精度 IEEE 浮点数值                               | `1.61803、3.14f32、6.0221e23f64`                             |
| `bool`                                 | 布尔值                                                     | `true、false`                                                |
| `char`                                 | Unicode 字符， 32 位宽                                     | `'*'、'\n'、'字'、'\x7f'、'\u{CA0}'`                         |
| `(char, u8, i32)`                      | 元组， 允许混合类型                                        | `('%', 0x7f, -1)`                                            |
| `()`                                   | “基元” （空） 元组                                         | `()`                                                         |
| `struct S { x: f32, y: f32 }`          | 命名字段结构体                                             | `S { x: 120.0, y: 209.0}`                                    |
| `struct T(i32, char); `                | 类元组 结构体                                              | `T {120, 'X'}`                                               |
| `struct E;`                            | 类基元 结构体，无字段                                      | `E`                                                          |
| `enum Attend { OnTime, Late(u32)}`     | 枚举，或代数数据类型                                       | `Attend::Late(5)、Attend::OnTime`                            |
| `Box`                                  | Box：拥有指向堆中值的指针                                  | `Box::new(Late(15))`                                         |
| `&i32、&mut i32`                       | 共享和可修改的引用：非所有型指针， 生命期不能超过引用 的值 | `&s.y、&mut v`                                               |
| `String`                               | UTF-8 字符串，动态分配大小                                 | `"ラーメン: ramen".to_string()`                              |
| `&str`                                 | 对 str 的引用：对 UTF-8 文本的非所有型指针                 | `"そば: soba"、&s[0..12]`                                    |
| `[f64; 4]、[u8; 256]`                  | 数组， 固定长度，元素类型 都相同                           | `[1.0, 0.0, 0.0, 1.0]、[b' '; 256]`                          |
| `Vec<f64>`                             | 向量， 可变长度，元素类型 都相同                           | `vec![0.367, 2.718, 7.389]`                                  |
| `&[u8]、&mut [u8]`                     | 对切片，即数组或向量某 一部分的引用，包含指针和长度        | `&v[10..20]、&mut a[..]`                                     |
| `&Any、&mut Read`                      | 特型对象，对任何实现了一组给定方法的值的引用               | `value as &Any、&mut file as &mut Read`                      |
| `fn(&str, usize) -> isize`             | 函数指针                                                   | `i32::saturating_add`                                        |
| （闭包类型没有书面形式）               | 闭包                                                       | `|a, b| a* a + b * b`                                        |





