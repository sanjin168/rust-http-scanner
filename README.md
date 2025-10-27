# Rust HTTP Scanner

一个高性能的 HTTP 服务命令行扫描工具，使用 Rust 编写。支持从单个目标或文件批量扫描 URL，并提供多种输出格式。

> tokio + clap + reqwest

## 功能特性

- 🚀 **高性能**：基于多线程设计，支持并发扫描
- 📁 **多输入源**：支持单个 URL 或文件批量输入
- 🔄 **重定向控制**：可配置的重定向跟随和最大重定向次数
- ⏱️ **超时设置**：自定义请求超时时间
- 🔌 **代理支持**：可通过代理服务器进行扫描
- 📊 **多格式输出**：支持 TXT、CSV、JSON 格式输出
- 🎨 **用户友好**：彩色命令行输出和进度显示

## 安装

### 从源码编译

```bash
# 克隆仓库
git clone https://github.com/sanjin168/rust-http-scanner.git
cd rust-http-scanner

# 编译发布版本
cargo build --release

# 可执行文件位于 target/release/rust-http-scanner
```

## 使用方法

### 扫描单个目标

```bash
rust-http-scanner --target https://example.com
```

### 从文件批量扫描

```bash
rust-http-scanner --file targets.txt
```

## 高级选项

### 设置超时和线程数：

```bash
rust-http-scanner --target https://example.com --timeout 10 --threads 128
```

### 禁用重定向跟随：

```bash
rust-http-scanner --target https://example.com --follow-redirect false
```

### 使用代理：

```bash
rust-http-scanner --target https://example.com --proxy http://127.0.0.1:8080
```

## 输出格式 (待办)

### TXT 格式（默认）：

```bash
rust-http-scanner --file targets.txt --output txt
```

### CSV 格式：

```bash
rust-http-scanner --file targets.txt --output csv
```

### JSON 格式：

```bash
rust-http-scanner --file targets.txt --output json
```
