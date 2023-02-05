## Todo

- Encrypt
- Sitemap

## arce-dev

主页 摄影 文章 图库 



主页：精选文章，分页。index
index/1.html

摄影：精选照片，分页。gallery
gallery/1.html

文章：类别，不分页。 article
article-categories.html

图库：所有照片，分页。 picture
picture/1.html



类别页
category/xxx.html

About 页
about.html

文章页
article/xxx.html







### 功能

#### 文章文件夹初始化

对目录下所有文件判断是否有 publish 标签并进行相应操作

将文件夹加入监测目录



#### 文章文件夹自动监测

param:  [指定文件夹位置，发布位置]



流程：

发现变动

1. 检测后缀是否是 md，是则继续进行：



2. 发现新文章/修改文章

a. 查找 publish 标签：是

将其添加到文章列表中

b. 查找 publish 标签：否

将其从文章列表中删除，如果有的话



2. 发现删除文章

从文章列表中将其删除



3. 修改定时发布信号为【是】





#### 文章渲染

param: 文章实际地址，实际发布地址，添加日期

return:  Result<HeadlineInfo>

其中 HeadlineInfo 包括：标题，日期，简述，分类，是否首页。



文章的 YAML 


```yaml
title: 
path: （可选，否则用 slug 代替）
category: （可选，否则 未归档）
headline:  （是否首页，默认为否）
date: （默认为添加日期）
summary: （默认为空）
```



流程：

1. 打开 md， 找出 yaml ，解析并删除这一部分

2. 照片上传与照片路径替换

3. 添加标题、日期与简介信息

4. markdown 解析

5. feat: 文章加密

6. 使用 Tera 渲染 html (header 处于文章)

7. 存储到对应地址

   



#### 照片监测

param:  [指定文件夹位置]



1. 监测到新建 DEPLOY
2. 对于每一个文件，考察 config.txt 中是否有 SELECTED[XXX], IGNORE[XXX], LINK[XXX]{XXX}, TITLE[XXX]{XXX}字样
3. 进行照片上传操作
4. 根据 2 中 flag，将相应地址加入到对应数据库中。





#### 照片上传

param: 地址

return: 地址

1. 这一图片是否已经存在了？（同时查询两个 md5 )
2. 读取照片信息并且加入数据库
3. 压缩、加水印（若有需要）
4. 更改命名
5. 加入到本地图片库，作为备份
6. scp 上传



#### 主页

根据 Headline info 渲染主页



#### 摄影

根据数据库信息渲染摄影页

注意链接



#### 文章

根据 Headline info 渲染 Categories

根据  Categories 渲染 Category



#### 图库

根据数据库信息渲染图库

注意链接



#### 渲染

根据数据库信息渲染所有文章，并得到 Headline info。渲染 about.

渲染主页

渲染文章

渲染摄影

渲染图库





#### 发布

渲染

使用 scp 上传到

feat: 部分渲染与部分发布



#### 发布触发

定时发布

手动发布



流程：





#### 手动修改信息







### 通用组件与结构

#### 组件

- Head
  - 需要可以更改 Tag 的颜色

- Foot
- 分页器
  - Prev Next
  - 数字标记





#### Config



网站标题

网站副标题

版权脚注

备案号



本地照片数据地址

照片云端前缀

压缩标准

scp 服务器名称

scp 图片库路径

scp 网站路径



是否定时发布

定时发布间隔







#### 数据库

ArticleFolders:



Articles:

文章路径，发布父亲目录，添加时间



Pictures:

原始 md5 ，新 md5 ，在线路径（后缀）

是否摄影，是否图库

标题，参数，日期，相机

是否链接到文章，文章地址



HeadlinePictures:



AllPictures:







### Tech Stack

sqlite, r2d2

tera

pulldown cmark

serde

notify



## TO Check
1. hash 和 文件名有无关联
2. 增加不严格的 markdown 格式转化





