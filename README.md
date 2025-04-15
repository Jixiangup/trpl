# TRPL

`TRPL`为`The Rust Programming Language`的缩写 这个项目会写一些关于我在接触使用以及学习`Rust`的时候遇到的一些很有意思的Demo把它们整理为每一个不同的`crate`加深对`Rust`的理解

## Usage

- 下载项目

```bash
# 克隆项目
git clone https://github.com/Jixiangup/trpl.git

# 进入项目目录
cd trpl

# 打包
cargo build --release

# 运行
./target/release/trpl crawler -u https://www.rust-lang.org -s title
```

## Crawler 爬虫

`crawler`是一个简单的爬虫工具，使用`scraper`库来发送HTTP请求，并使用`select`库来解析HTML文档。它可以从指定的URL中提取指定的元素，并将其打印到控制台。

### Usage

```bash
./target/release/trpl crawler -u https://www.rust-lang.org -s title
```

### Options

- `-u`或`--url`：指定要爬取的URL。
- `-s`或`--selector`：指定要提取的元素的CSS选择器。