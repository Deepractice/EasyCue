#!/bin/bash

# EasyCue 启动脚本

echo "🚀 启动 EasyCue..."

# 加载 Rust 环境
source "$HOME/.cargo/env"

# 进入项目目录
cd "$(dirname "$0")"

# 编译并运行
cd src-tauri
cargo run

echo "✅ EasyCue 已退出"