# 作业

## 学员提交作业流程

**第一次写作业时**，

首先Fork自己所在组的代码仓库（本仓库）到自己的Github用户里，

![fork](./fork.png)

接着在本地，克隆刚刚Fork过来的代码仓库，Git命令为:

```
git clone git@github.com:<your-github-account>/<your-team>.git
```

替换`your-github-account`为自己Github账户名，`<your-team>`为自己所在组。

然后在本地，添加自己所在组的代码仓库链接，方便后面更新代码，Git命令为：

```
git remote add upstream git@github.com:SubstrateCourse/<your-team>.git
```

替换`<your-team>`为自己所在组。通过`git remote -v`，你应该能看到新添加的远端仓库链接。

*每次写作业之前*，

在本地，首先切换到以自己名字命名的分支， Git命令为：

```
git checkout <your-name>
```

替换`your-name`为自己的名字，如果没有，联系助教进行添加。

之后，


## 讲师、助教职责

本仓库的master分支由讲师提交；

本仓库由学员名字命名的分支，由学员通过发送PR提交，助教review之后merge到相应的分支。

每节课之前，master分支会提交该课的作业基础代码，由助教将新增的代码同步到学员分支。

*Flow:*

![flow](./course_flow.png)

