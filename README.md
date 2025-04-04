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
cargo add zip unrar sevenz-rust
```

## 编译
```sh
cargo build --release
```
可执行文件位于 `target/release/ExtractTool.exe`。

## 使用方法
```sh
ExtractTool.exe <压缩文件路径> <输出目录>
```

### 示例
- **解压 ZIP 文件**
  ```sh
  ExtractTool.exe example.zip output_folder
  ```

- **解压 RAR 文件**
  ```sh
  ExtractTool.exe example.rar output_folder
  ```

- **解压 7Z 文件**
  ```sh
  ExtractTool.exe example.7z output_folder
  ```

## 兼容性
- 适用于 Windows 。

## 许可证
MIT License

Copyright ©️2025 Colzry