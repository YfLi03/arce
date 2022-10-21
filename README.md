# arce
A *minimal* blog generator for photographers powered by `Rust`.

一个针对摄影师开发的静态博客生成器，兼具展示markdown文章的功能。

Demo: [icera's gallery](http://iceeera.com)

Current Version: `0.2.4`

*The theme is inspired by camarts*



## Features:
- 使用Rust编写——速度很快
- 简洁大方的主题
- 响应式UI，意味着在手机和电脑上都能获得较为良好的体验
- 自动从Exif中抓取光圈快门ISO、拍摄时间等信息；自动压缩尺寸大的图片
- 完全静态，方便部署
- 操作简单，没有编程基础应该也可以使用


## 使用方法
#### 1. 下载安装
从这个github仓库右侧的releases中，下载最新版本的压缩包。下载完成后解压。



#### 2. 基础配置
- 你需要修改目录中的`config.yaml`文件。推荐使用VSCode, notepad++等软件打开，如果没有安装这些软件的话也可以使用系统自带的记事本。
- 在每一项的 `:`后修改内容为自己想要的内容。**请不要修改`:`前的内容！** 以下为各项设置的解释：

    - tab_title: 显示在标签页上的标题
    - title: 显示在网页上的标题
    - subtitle: 显示在网页上的副标题
    - footer_info: 网页底部内容
    - beian: 备案号（可以留空）
    - compress_imgae: 值只能是`true`或者`false`。决定了是否要自动压缩`>800kb`的图片。一般情况下，建议开启这一选项^1^。

- 需要修改`source目录`下的`about.md`。请使用`markdown`语法，将内容修改为你的自我介绍。


#### 3. 导入图片与文档
如果没有对应的文件夹，请自行创建。

- 将精选照片复制到`public/gallery/selected`目录下
- 将其余照片复制到`public/gallery/all`目录下
- 将文档(md格式)复制到source/article目录下
- 请确保selected和all目录下的图片有不同的名字。同时，如果一张图片和一个文档有相同的名字（不考虑后缀），那么这张照片将会被自动链接到对应的文章上。

*注:推荐在每一篇markdown文章的头部添加如下格式的配置信息*
```
---
title: 文章标题
date: yyyy-mm-dd
---
```
*这不是强制的，但如果没有配置信息，文档文件名会被认为是文章标题，文档创建时间会被认为是文章写作时间。*


#### 4. 运行程序，生成网页文件
- Windows下，双击arce_blog.exe。
- Linux下，切换到对应目录在终端输入./arce_blog

看到`Main Completed`即说明网页已经生成好了


此时，进入`public`文件夹，里面的`index.html, about.html`等等即为生成的静态网页。受寻址机制所限，在本地打开html网页可能会遇到样式无法加载、超链接错误等问题，不必担心。



#### 5. 部署网页
- 如果你已经有了自己的服务器、域名，那这一步应该也不需要我来教您。直接把public文件夹中的内容一股脑丢到服务器中对应文件夹 (例如我的是nginx目录下的html文件夹)即可。
- 如果你没有自己搭建网站的经历，不知道什么叫“部署”。那么请依据如下教程：

    1. 参阅[网络教程，例如这篇文章](https://zhuanlan.zhihu.com/p/448782779)申请一个`Github Pages`

    2. 申请完毕后，直接在网页上选择`Add file`, `Upload file`,将public文件夹**内**的文件全部上传上去即可 。**public文件夹本身不要传上去！**
    *这种方式一次最多上传100个文件，如果照片多的话可能需要分步操作*

    3. 点击网页底部的`Commit Changes`, 然后你就可以通过`你的用户名.github.io`访问自己的照片博客了.

注:
^1^图片在压缩后会损失exif信息。正常情况下，程序会通过pics.json文件持久化保存原有的exif信息，可以正常显示在您的网页上。但若在此之后，对图片进行了重命名等操作，将会导致信息无法匹配。

## 代码说明
有待更新

### 依赖
- Rust
- Bootstrap4
- Tera
- kamadak-exif
- pulldown-cmark
- chrono
- image-rs
- serde

