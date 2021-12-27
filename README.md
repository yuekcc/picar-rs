# picar-rs

本项目是 rust 实现的 [picar](https://github.com/yuekcc/picar)。

picar 是自家用照片整理工具。 picar 会按照拍照日期将文件整理到不同的目录，比如： 

```
IMG_20151106_212111.jpg => 201511/PREFIX_20150401_111111.jpg
```

## 功能

- [x] 读取照片文件的 Exif 数据
- [x] 按照 Exif 数据记录的时间日期，重命名文件
- [x] 支持命令行参数
    - [x] `--rename-only`
    - [ ] `--prefix`
    - [ ] `-videos`
- [ ] 打包脚本

## LICENSE

[MIT](LICENSE)
