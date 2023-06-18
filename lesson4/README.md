### Lesson4: Off-chain Worker 
>
> 请回答链上随机数（如前面Kitties示例中）与链下随机数的区别
>
> * 在Offchain Worker中，使用Offchain Indexing特性实现从链上向Offchain Storage中写入数据
> * 使用 js sdk 从浏览器frontend获取到前面写入Offchain Storage的数据
> * 设计一个场景实例（比如获取一个外部的价格信息），实现从OCW中向链上发起带签名负载的不签名交易，并在Runtime中正确处理
>

#### 测试用例

[tests.rs](https://github.com/dylan-nm/substrate-advnce-learning/blob/main/substrate-node-template/pallets/kitties/src/tests.rs)

#### 测试截图
<img width="1089" alt="image" src="https://github.com/dylan-nm/substrate-advnce-learning/assets/41264413/192b95b5-6fc3-4671-ae48-bdac4429df94">

### 截图
<img width="1135" alt="image" src="https://github.com/dylan-nm/substrate-advnce-learning/assets/41264413/41a13805-c130-489e-9958-588d2d7b810d">


