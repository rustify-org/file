# file

Rust 的文件操作

## Rust 文件操作

- std::fs:: File
- Rust 使用结构体 File 来描述/操作一个文件
- File 的所有方法都会返回一个 Result 枚举
- 注意这种设计

## 创建文件

- std::fs::File::create 只写模式打开一个文件
  - 如果文件存在清空
  - 如果文件不存在新建
- 同样返回一个文件句柄
- (std::fs::OpenOptions 可设置文件为追加模式)

## 删除文件

- std::fs::remove_file
- 从文件系统中删除某个文件

## 文件内容的操作

- std::io::Read
- std::io::Writes

## 读取文件的三种方式

- Read a file as aVec
- Read a file as a whole as a String
- Read a file line by line

## 写入文件

- 新建方式
  - Write a &str
  - Write a byte string
- 追加方式
  - std::fs::OpenOptions::new().append(true).open('df.txt')
