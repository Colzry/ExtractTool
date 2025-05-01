# 压缩文件解压工具

## 概述
本工具是一个基于 Rust 的命令行解压程序，支持以下格式的压缩文件：
- **ZIP** (`.zip`)
- **RAR** (`.rar`)
- **7Z** (`.7z`)

## 依赖
本项目依赖以下 Rust 库：
- [`zip`](https://crates.io/crates/zip) 处理 ZIP 文件
- [`unrar`](https://crates.io/crates/unrar) 处理 RAR 文件
- [`sevenz-rust`](https://crates.io/crates/sevenz-rust) 处理 7Z 文件

### 安装 Rust
如果未安装 Rust，请先安装：
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 安装依赖
```sh
cargo install
```

## 编译
```sh
cargo build --release
```
可执行文件位于 `target/release/GameEffectTool.exe`。

## 使用方法
### 方法一：直接运行
1. 将压缩包与程序放在同一目录下
2. 运行程序

### 方法二：指定压缩包
1. 将压缩包与程序放在同一目录下
2. 添加参数启动
```sh
GameEffectTool.exe -p <当前路径下压缩包名称> -d <解压目录>
```

## 兼容性
- 适用于 Windows 。

## 许可证
MIT License

Copyright ©️2025 Colzry