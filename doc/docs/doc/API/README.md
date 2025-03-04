---
title: API申请教程
keywords: API申请教程, 随问
desc: API申请教程
date: 2025-03-05
---


## 介绍

**随问目前本身不提供大模型服务，需要用户自己申请各大模型的API并将Base Url、API密钥、模型名称提供给随问，（有了这三个项，随问就可以调用对应的大模型为你提供服务啦）。**

这里介绍火山引擎的DeepSeek的API申请教程，也是作者最推荐（也是作者正在使用）的一个API，生成质量相对较好，速度相对较快。


### 第一步：注册火山引擎账号
打开[火山方舟](https://console.volcengine.com/ark)控制台页面，按照提示注册一个火山引擎账号，火山方舟的控制台如下所示：
![火山方舟](/static/images/Ark.png)


### 第二步：创建API Key
在页面左侧下方找到API Key管理，创建一个API Key，**然后复制API Key的值。**

### 第三步：开通DeepSeek服务
在页面左侧下方找到开通管理，然后在左侧找到DeepSeek V3 点击开通服务，图片为已经开通过的样子。
![火山方舟](/static/images/ActivateLLM.png)

### 第四步：得到模型名
点击DeepSeek模型，如下红框所示
![点击模型详情](/static/images/GetModelID1.png)
之后会进入如下的页面，**复制红框所示的模型名。**
![复制模型ID](/static/images/GetModelID2.png)

### 第五步：配置随问
火山引擎的Base Url为`https://ark.cn-beijing.volces.com/api/v3`
将上面的Base Url和第二步得到的Key和第四步得到的模型名填入随问的大模型配置页面里（注意：你得到的模型名可能与图中的不一样）
![配置随问](/static/images/ConfigModel.png)

### 第六步：测试模型连通性
填完之后点击保存，再点击下方的测试模型连通性按钮，若出现连接成功的提示则证明配置成功，之后就可以使用随问啦！
![测试连接](/static/images/ConnectSuccess.png)