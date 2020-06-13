use crate::reader::{
    open_flv, AudioData, AudioDataHeader, Field, Header, Tag, TagData, TagHeader, VideoData,
    VideoDataHeader,
};
use std::error::Error;
use tokio::stream::StreamExt;

mod reader;

type Exception = Box<dyn Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Exception> {
    let path = std::env::args()
        .nth(1)
        .unwrap_or("./resources/test.flv".into());

    let (
        file_size,
        Header {
            version,
            type_,
            offset,
        },
        mut decoder,
    ) = open_flv(&path).await?;

    println!("=====================================");
    println!("File: {}", path);
    println!("FileSize: {}", file_size);
    println!("Version: {}", version);
    println!("Type: {}", type_);
    println!("DataOffset: {}", offset);

    let mut pre_tag_size_index = 0;
    let mut tag_index = 1;

    while let Some(result) = decoder.next().await {
        match result {
            Ok(field) => match field {
                Field::PreTagSize(size) => {
                    println!("=====================================");
                    println!("PreviousTagSize{}: {}", pre_tag_size_index, size);
                    pre_tag_size_index += 1;
                }
                Field::Tag(Tag {
                    header:
                        TagHeader {
                            tag_type,
                            data_size,
                            timestamp,
                        },
                    data,
                }) => {
                    println!("=====================================");
                    println!("TagIndex: {}", tag_index);
                    println!("TagType: {:?}", tag_type);
                    println!("DataSize: {:?}", data_size);
                    println!("Timestamp: {:?}", timestamp);
                    match data {
                        TagData::Audio(AudioData {
                            header:
                                AudioDataHeader {
                                    sound_format,
                                    sound_rate,
                                    sound_size,
                                    sound_type,
                                },
                            data,
                        }) => {
                            println!("SoundFormat: {:?}", sound_format);
                            println!("SoundRate: {:?}", sound_rate);
                            println!("SoundSize: {:?}", sound_size);
                            println!("SoundType: {:?}", sound_type);
                            println!("Data: {:?}", data);
                        }
                        TagData::Video(VideoData {
                            header:
                                VideoDataHeader {
                                    frame_type,
                                    codec_id,
                                },
                            data,
                        }) => {
                            println!("FrameType: {:?}", frame_type);
                            println!("CodecId: {:?}", codec_id);
                            println!("Data: {:?}", data);
                        }
                        TagData::Script(_) => {
                            // TODO: parse the raw script data
                            println!("RawScriptData: {:?}", data);
                        }
                        TagData::Reserved(data) => {
                            println!("Data: {:?}", data);
                        }
                    }
                    tag_index += 1;
                }
            },
            Err(e) => return Err(e),
        }
    }

    println!("=====================================");

    Ok(())
}
