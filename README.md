这是我的第一个Rust小项目，将一串字符串信息隐藏进png图片，原理是新增一个对图片显示没有影响的chunk，解析该chunk就可以得到隐藏的信息。
参考链接：[https://picklenerd.github.io/pngme_book/introduction.html](https://picklenerd.github.io/pngme_book/introduction.html)
# Introduction
该小项目根据[PNG文件结构](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)自定义Png的创建与解析，PNG文件结构其实很简单，由一个固定头signature和多个chunk组合而成，如下图所示
![png_struct.png](https://cdn.nlark.com/yuque/0/2022/png/23135940/1657509586833-6a76cf53-a304-4bde-a41a-c43a54c2613b.png#clientId=u6933692d-b9d1-4&crop=0&crop=0&crop=1&crop=1&from=ui&id=u22c35d5a&margin=%5Bobject%20Object%5D&name=png_struct.png&originHeight=320&originWidth=991&originalType=binary&ratio=1&rotation=0&showTitle=false&size=40536&status=done&style=none&taskId=u17af43fd-0046-4801-9816-2f40a719c51&title=)
具体细节还有很多，包括chunk_type的定义，chunk_type由各自字母组成，大小写的不同代表了chunk的不同特性。
将
# Data Structure
## ChunkType
```rust
pub struct ChunkType(u8, u8, u8, u8);
```
## Chunk
```rust
pub struct Chunk {
    data_length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
```
## Png
```rust
pub struct Png {
    signature: [u8; 8],
    pub(crate) chunks: Vec<Chunk>,
}
```
# 使用示例
pngme encode <file_path> <chunk_type> <message> <out_file_path>
pngme decode <file_path> <chunk_type>
pngme remove <file_path> <chunk_type>
pngme print <file_path>
![image.png](https://cdn.nlark.com/yuque/0/2022/png/23135940/1657510398726-6567250f-0007-4daf-9004-fe2b223927ca.png#clientId=u6933692d-b9d1-4&crop=0&crop=0&crop=1&crop=1&from=paste&height=343&id=u1914f7aa&margin=%5Bobject%20Object%5D&name=image.png&originHeight=686&originWidth=1748&originalType=binary&ratio=1&rotation=0&showTitle=false&size=1036080&status=done&style=none&taskId=u2fff4be3-2efe-4dc2-bb0c-8a9798573e0&title=&width=874)
