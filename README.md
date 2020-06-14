# flv-dump

使用方法：

1. 安装 [Rust 开发环境](https://www.rust-lang.org/zh-CN/tools/install) ；
2. `git clone https://github.com/live2o3/flv-dump.git` ；
3. cd flv-dump && cargo build --release && cd target/release ；
4. `./flv-dump <FLV文件路径> > dump.txt` 。

生成的 dump.txt 格式如下：

```
=====================================
File: ../../resources/test.flv
FileSize: 2674235
Version: 1
Type: 5
DataOffset: 9
=====================================
PreviousTagSize0: 0
=====================================
TagIndex: 1
TagType: Script
DataSize: 366
Timestamp: 0
RawScriptData: Script(ScriptData { raw: b"<此处省略真实数据>" })
=====================================
PreviousTagSize1: 377
=====================================
TagIndex: 2
TagType: Video
DataSize: 46
Timestamp: 0
FrameType: KeyFrame
CodecId: AVC
Data: b"<此处省略真实数据>"
=====================================
PreviousTagSize2: 57
=====================================
TagIndex: 3
TagType: Video
DataSize: 20208
Timestamp: 0
FrameType: KeyFrame
CodecId: AVC
Data: b"<此处省略真实数据>"
=====================================
PreviousTagSize3: 20219
<...>
```
