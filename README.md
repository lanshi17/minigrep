# minigrep

一个简单的 grep 工具，用于在文件中搜索文本。

## 使用方法

### 基本用法
```bash
cargo run -- <查询词> <文件路径>
```

例如：
```bash
cargo run -- "the" src/poem.txt
```

### 控制大小写敏感性

#### 使用命令行参数
```bash
cargo run -- <查询词> <文件路径> --ignore-case
```

或者使用简短形式：
```bash
cargo run -- <查询词> <文件路径> -i
```

#### 使用环境变量
```bash
IGNORE_CASE=1 cargo run -- <查询词> <文件路径>
```

### 优先级

环境变量具有更高的优先级。如果同时设置了命令行参数和环境变量，将优先使用环境变量的设置。

例如，以下命令将进行大小写敏感的搜索（因为环境变量的优先级更高）：
```bash
IGNORE_CASE=0 cargo run -- <查询词> <文件路径> --ignore-case
```

即使设置了 `--ignore-case` 参数，只要设置了 `IGNORE_CASE=0` 环境变量，程序也会进行大小写敏感的搜索。