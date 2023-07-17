# 第五章 上线


我们已经有了一个运行良好的新闻通讯API原型，现在是时候将其上线了。

我们将学习如何将我们的Rust应用程序打包为Docker容器，以便在[DigitalOcean](https://docs.digitalocean.com/products/app-platform/)的App平台上部署(译者， 这里我将选择别的一个平台进行部署，而不是DigitalOcean)。

在本章的最后，我们将拥有一个持续部署（CD）流水线：每次提交到主分支都会自动触发部署最新版本的应用程序给我们的用户。


## 5.1 我们必须谈论部署

每个人都喜欢谈论尽可能频繁地将软件部署到生产环境的重要性（我也是其中之一！）。

> "尽早获得客户反馈！"
> "经常发布并对产品进行迭代！"

但是没有人告诉你如何做到这一点。

随机选择一本关于Web开发或某个框架介绍的书。
大多数书对部署的主题只会用一段话来简单提及。
少数书籍可能会有一章来讨论部署，通常是在书的结尾，你可能根本没有机会真正阅读到。

只有少数书籍在合理的情况下，给予了它应有的重视。

为什么呢？

因为部署（至今）仍然是一项复杂的任务。

有很多供应商，大多数并不直观易用，而且被认为是最先进或最佳实践的事物往往变化迅速{1Kubernetes is six years old, Docker itself is just seven years old!}。

这就是为什么大多数作者避开这个话题的原因：它需要很多页的篇幅，而且很痛苦地写下来后才意识到，一两年后它已经过时了。

尽管如此，部署是软件工程师日常工作中重要的关注点——例如，谈论数据库架构迁移、领域验证和API演进时都很难不考虑你的部署流程。

在一本名为《从零到生产》的书中，我们简单不能忽视这个主题。


## 5.2 选择工具

本章的目的是让你亲身体验每次提交到主分支时的部署过程。

这就是为什么我们在第五章早早地谈论部署的原因：为了让你有机会在本书的其余部分实践这项技能，就像你在一个真实的商业项目中所做的那样。

实际上，我们特别关注的是持续部署工程实践对我们的设计选择和开发习惯的影响。

同时，构建完美的持续部署流水线并不是本书的重点，它需要一本独立的书籍，甚至可能需要整个公司的介入。

我们必须务实并在内在的有用性（即学习一个在行业中受到重视的工具）和开发者体验之间取得平衡。

即使我们花时间来打造“最佳”设置，由于你组织的具体限制，你仍然有可能选择不同的工具和不同的供应商。

重要的是其潜在的理念，并让你尝试将持续部署作为一种实践。

### 5.2.1 虚拟化：Docker

我们的本地开发环境和生产环境有着截然不同的目的。

浏览器、集成开发环境、音乐播放列表等可以共存于我们的本地机器上。它是一个多功能的工作站。
相比之下，生产环境的焦点更加狭窄：运行我们的软件，使其对用户可用。任何与此目标无关的东西，最好情况下都是资源浪费，最坏情况下是安全风险。

这种差异在历史上使得部署变得非常麻烦，引发了“在我的机器上可以运行”的抱怨，现在已经成为一个名言。

仅仅将源代码复制到我们的生产服务器是不够的。我们的软件很可能对底层操作系统提供的功能有所假设（例如，原生Windows应用程序无法在Linux上运行），对同一台机器上其他软件的可用性有所依赖（例如，某个版本的Python解释器），或者对其配置有所依赖（例如，我是否具备root权限）。

即使我们从两个相同的环境开始，随着版本的漂移和微妙的不一致性的出现，随着时间的推移，我们也会遇到问题。

确保我们的软件正确运行的最简单方法是严格控制其执行环境。

这是虚拟化技术的基本理念：如果你可以将一个包含应用程序的自包含环境部署到生产环境中，而不是仅仅部署代码，那将会非常好！

对于开发人员来说，这减少了周五晚上的意外；对于负责生产基础设施的人员来说，它提供了一个一致的抽象层。

如果环境本身可以作为代码来指定，以确保可重现性，那就更好了。

虚拟化的好处在于它已经存在，并且已经成为主流技术已经有将近十年的时间了。

对于技术中的大多数事物来说，根据你的需求，你有几个选择：虚拟机、容器（如Docker）和其他一些选项（如Firecracker）。

我们将选择主流且无处不在的选项 - Docker容器。

### 5.2.2 托管：DigitalOcean

[AWS](https://aws.amazon.com/)、[Google Cloud](https://cloud.google.com/?hl=zh-cn)、[Azure](https://azure.microsoft.com/en-gb/)、[Digital Ocean](https://www.digitalocean.com/)、[Clever Cloud](https://www.clever-cloud.com/)、[Heroku](https://www.heroku.com/)、[Qovery](https://www.qovery.com/)...
可以选择托管软件的供应商的列表还在继续增长。

人们已经成功地通过推荐最适合您特定需求和用例的云平台来创造了一个业务 -
这不是我的工作（尚未）或本书的目的。

我们正在寻找的是易于使用的东西（出色的开发者体验，最小的不必要复杂性）和相对成熟的解决方案。
在2020年11月，这两个要求的交集似乎是Digital Ocean，尤其是他们新推出的App Platform提案。

> 免责声明：Digital Ocean并没有支付我在这里推广他们的服务。

## 5.3 为我们的应用程序编写 Dockerfile


[DigitalOcean的App Platform](https://www.digitalocean.com/docs/app-platform/languages-frameworks/docker/)原生支持部署容器化应用程序。
这将是我们的第一个任务：我们需要编写一个Dockerfile来构建和执行我们的应用程序作为一个Docker容器。

### 5.3.1 Dockerfile

Dockerfile是您的应用程序环境的配方。

它们以层次结构组织：您从一个基础镜像开始（通常是一个包含编程语言工具链的操作系统），然后按顺序执行一系列命令（如COPY、RUN等），以构建所需的环境。

让我们来看一个最简单的用于Rust项目的Dockerfile示例：

```dockerfile
# We use the latest Rust stable release as base image
FROM rust:1.59.0
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]
```

将其保存为名为 Dockerfile 的文件，放在我们的 git 存储库的根目录中。

```bash
zero2prod/
    .github/
    migrations/
    scripts/
    src/
    tests/
    .gitignore
    Cargo.lock
    Cargo.toml
    configuration.yaml
    Dockerfile
```

使用 Docker CLI 执行这些命令来获取镜像的过程称为构建。

```bash
# Build a docker image tagged as "zero2prod" according to the recipe
# specified in `Dockerfile`
docker build --tag zero2prod --file Dockerfile .
```

在命令末尾的点号 (.) 表示构建上下文，它指定了构建过程中的文件路径。在这种情况下，点号表示当前目录作为构建上下文，Docker 将会在当前目录中查找 Dockerfile 和其他需要复制到镜像中的文件。

### 5.3.2  Build Context

docker build 通过使用一个构建上下文来生成镜像。你可以将正在构建的Docker镜像想象为一个完全隔离的环境。镜像与你的本地机器之间唯一的联系点是像COPY或ADD这样的命令。构建上下文决定了在Docker容器内部，COPY和其他相关命令可以看到你本地机器上的哪些文件。

通过使用`.`，我们告诉Docker将当前目录作为这个镜像的构建上下文。因此，`COPY . app`命令将会将当前目录中的所有文件（包括我们的源代码！）复制到Docker镜像的`app`目录中。

使用`.`作为构建上下文意味着Docker不会允许`COPY`命令看到父目录中的文件，也不会允许它看到你本地机器上的任意路径中的文件。

根据你的需求，你可以使用不同的路径甚至是URL作为构建上下文。

### 5.3.3  Sqlx offline mode

这里我们使用的是sqlx的0.7版本，这里offline已经是默认支持的feature了，所以不需要再指定。
