## RCLI

rcli is a rust CLI tool

# learn a programming language

- learn x in Y minutes
  website: https://learnxinyminutes.com/

# 学习 clap crate 基本用法

- cargo add clap --features derive
  一个 crate 里有多个 features 这里是使用 clap 中一个 feature 的 derive 功能，其他用不着的功能不用下载

# 构建基本的 CLI 的结构

# 使用 duckdb 查看 csv

# 学习 csv carate 基本用法

# 学习 serde 基本用法

# 读取 csv，输出到 console

# 学习 serde-json 基本用法

# 将 csv 转换成 json

# 重构代码

# question

1. #[command(name = "rcli", version, author, about, long_about = None)] #[command()]这个宏是从哪里引入的?

# vscode 小技巧

- 选中代码 选中 rename symbol 作用域下的要改的重复东西都会被修改
- mac command + . 快速修复代码
- cargo run -- csv -i input.csv -o output.csv --header -d ','

1. cargo run 后面的 -- 是把 rust 编译并执行后的程序和后面的参数分开，即 -- 前面是 rust 程序，后面是传递给 rust 程序的参数
2. -i 是 input 的简写 input.csv 是所跟参数 后面的 -o 和 -d 类似
3. --header 通常在命令行中指代 bool

# 安装 duckdb

- windows: winget install DuckDB.cli 在终端输入 duckdb 即可使用
- mac: brew install duckdb 同上

# tokei install

cargo install --git https://github.com/XAMPPRocky/tokei.git tokei

这个工具可查看我们的代码写了多少行

## CLI 项目：生成随机密码

- 改进之前的 csv -> json 代码
- 支持多种输出格式: json, toml, yml
- 随机生成密码的需求：长度，大小写，特殊字符，强度
- 随机数生成：rand crate
- 构建一个密码生成器
- 密码强度检测: zxcvbn crate
- 重构代码

# zsh/fish nushell (Rust): completion

structopt crate 代码库 和 clap crate 类似 可以看它源码
libp2p 看源码

## clI 项目：base64 编解码

# base64 处理：base64 crate

# 对任意文件或者 stdin 的输入，输出 base 64

- 如何处理作用域中的不同返回类型？

# 对 stdin 的 base64 输入， 输出原始内容

# 单元测试

## ClI 项目： Http 文件服务器 （1）

- 简要了解 Rust 处理 web 相关的库
  - tokio
  - hyper, reqwest, axum
  - tower, tower-http
- 构建 CLI:
  rcli http serve -d . => serve 当前目录的文件
- 使用 axum 构建一个简单 web 服务器
- 使用 axum 和 tokio fs
