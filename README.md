# arce
A *minimal* blog generator for photographers powered by `Rust`.
Demo: [icera's gallery](http://iceeera.com/pics)

Current Version: `0.1.1`

*The theme is inspired by camarts*

#### Features:
- 三个页面, 精选(单页),所有照片(自动分页),关于(从markdown文件渲染)
- 简洁美观且响应式的UI，意味着在手机和电脑上都能获得较为良好的体验
- 自动从Exif中抓取光圈快门ISO、拍摄时间等信息

## 使用方法
#### 1. 下载安装
从这个github仓库的release中下载对应版本的压缩包并且解压



#### 2. 基础配置
- 你需要修改目录中的`config.yaml`文件。推荐使用VSCode, notepad++等软件打开，如果都没有的话也可以使用Windows自带的记事本。
在每一项的 `:`后修改内容为自己想要的内容。请不要修改`:`前的内容！
- 你需要修改主目录中的`about.md`，可以使用markdown语法，将内容修改为你的自我介绍即可。


#### 3. 导入图片
*你可以先依照4中的步骤运行一次程序，生成对应文件夹*
将精选照片复制到`public/gallery/selected`目录下
将其余照片复制到`public/gallery/all`目录下
*由于图片加载一般较为缓慢，请控制图片的大小. 推荐在 `500kB` 以下*


#### 4. 运行程序
- Windows下，双击arce_blog.exe。看到`Main Completed`即说明网页已经生成好了
- Linux下...你都会用linux了还要我来教你？


运行完毕后,进入`public`文件夹，里面的`index.html, about.html`即为生成的静态网页



#### 5. 部署网页
- 如果你已经有了自己的服务器、域名，那相信这一步也不需要我来教您。直接把public文件夹中的内容一股脑丢到服务器上(例如我的是nginx目录下的html文件夹)即可
- 如果你没有自己搭建网站的经历，甚至不知道什么叫“部署”。那么我建议你
    1. 参阅[网络教程，例如这篇文章](https://zhuanlan.zhihu.com/p/448782779)申请一个`Github Pages`
    2. 申请完毕后，直接在网页上选择`Add file`, `Upload file`,将public文件夹内的文件全部上传上去即可 （**public文件夹本身不要传上去！**)
    *这种方式一次最多上传100个文件，如果照片多可能需要分步操作*
    3. 点击网页底部的`Commit Changes`, 然后你就可以通过`你的用户名.github.io`访问自己的照片博客了.


## 如果你想开发这个项目...
~~不会吧不会吧不会吧, 写成*山的代码竟然有人愿意看~~
Clone后使用`cargo  build`即可

### 依赖
- Rust
- Bootstrap4
- jQuery
- Tera
- Serde
- kamadak-exif
- pulldown-cmark

*Thanks for the contribution of LauYeeYu*
