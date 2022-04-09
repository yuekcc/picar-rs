# picar-rs

picar 是自家用照片整理工具。 picar 会按照拍照日期将文件整理到不同的目录，比如： 

```
IMG_20151106_212111.jpg => 201511/PREFIX_20150401_111111.jpg
```

>本项目是 rust 实现的 [picar][1]。[picar][1] 使用 golang 实现并带有一个简单的 GUI；[原始版本][2] 使用 python 实现。

[1]: https://github.com/yuekcc/picar
[2]: https://github.com/yuekcc/exifrename

## 功能

- [x] 读取照片文件的 Exif 数据
- [x] 按照 Exif 数据记录的时间日期，重命名文件
- [x] 支持命令行参数
    - [x] `--rename-only` 只修改文件名
    - [x] `--prefix` 文件名前缀
    - [ ] `--videos` 处理视频文件

## 构建

**需要 rust 1.59.0+**

```shell
$ cargo build --release
$
$ # 如果安装了 just，可以直接执行
$ just release
```

## LICENSE

[MIT](LICENSE)
