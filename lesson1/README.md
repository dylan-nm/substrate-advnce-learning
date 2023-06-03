### Lesson1: Proof of Existence
>编写存证模块的单元测试代码，包括：
>* 创建存证的测试用例
>* 撤销存证的测试用例
>* 转移存证的测试用例
>（建议用polkadot-v0.9.40这个版本，有问题的话去github上搜issue）提交的Github链接必须包含：⚠️代码运行的截图图片+⚠️全部代码

```shell
git clone -b polkadot-v0.9.40 --depth 1 https://github.com/substrate-developer-hub/substrate-node-template.git
cargo build --release
# 测试poe
cargo test -p pallet-poe
```

#### 测试用例

[test.rs](https://github.com/dylan-nm/substrate-advnce-learning/blob/main/substrate-node-template/pallets/poe/src/tests.rs)

#### 测试截图

<img width="1078" alt="image" src="https://github.com/dylan-nm/substrate-advnce-learning/assets/41264413/2768dd44-bfd7-41f2-abb0-05e358e19722">


