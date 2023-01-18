# arce
A *minimal* blog generator for photographers powered by `Rust`.

一个针对摄影师开发的**静态博客生成器**，兼具展示文章的功能。

Demo: [icera's gallery](http://iceeera.com)

Current Version: `1.0.2`

## 功能
- 监测自定文件夹内 markdown 文章，自动将待部署的文章加入数据库内
- 监测自定文件夹内的照片，自动将待部署的照片加入数据库内
- 全自动定时部署，生成静态网页并通过 scp 传输到服务器相应文件夹


## Features:
- 使用Rust编写 —— 速度很快，占用很低
- 简洁大方的网页主题，响应式 UI
- 自动化程度较高，一次配置，全程自动；无需手动将文件放入 posts 文件夹内，支持多（文件夹）文章来源
- 自动从Exif中抓取光圈快门ISO、拍摄时间等信息；自动压缩尺寸大的图片；对于 md 文章内，路径为本地的照片，会自动上传到服务器上并进行路径替换


## 代码说明
有待更新


## 鸣谢

照片部分 UI 灵感来自 *camarts*

文字部分 UI 部分借鉴了 *Typora Whitely*, 并使用了其 css。


## 主要依赖
- Bootstrap4        网页 UI
- Tera              网页渲染
- kamadak-exif      照片信息读取
- pulldown-cmark    Markdown 渲染
- r2d2_sqlite       持久化
- notify            文件夹监测

