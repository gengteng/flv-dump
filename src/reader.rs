use crate::Exception;
use bytes::{Buf, Bytes, BytesMut};
use std::convert::TryFrom;
use std::path::Path;
use tokio::fs::File;
use tokio::io::BufReader;
use tokio::prelude::*;
use tokio_util::codec::{Decoder, FramedRead};

#[derive(Debug)]
pub struct Header {
    pub version: u8,
    pub type_: u8,
    pub offset: u32,
}

#[derive(Debug, Copy, Clone)]
pub enum TagType {
    Audio,        // 8
    Video,        // 9
    Script,       // 18
    Reserved(u8), // all others
}

#[derive(Debug, Clone)]
pub struct TagHeader {
    pub tag_type: TagType,
    pub data_size: u32,
    pub timestamp: i32, // UI24 + UI8 => SI32
                        // stream_id: u32, // UI24 always 0
}

#[derive(Debug)]
pub struct Tag {
    pub header: TagHeader,
    pub data: TagData,
}

#[derive(Debug)]
pub enum SoundFormat {
    LinearPCMPlatformEndian,
    ADPCM,
    MP3,
    LinearPCMLittleEndian,
    Nellymoser16,
    Nellymoser8,
    Nellymoser,
    G711ALaw,
    G711MuLaw,
    Reserved,
    AAC,
    Speex,
    MP38kHz,
    DeviceSpecific,
}

impl TryFrom<u8> for SoundFormat {
    type Error = Exception;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use SoundFormat::*;
        Ok(match (value & 0b_1111_0000) >> 4 {
            0 => LinearPCMPlatformEndian,
            1 => ADPCM,
            2 => MP3,
            3 => LinearPCMLittleEndian,
            4 => Nellymoser16,
            5 => Nellymoser8,
            6 => Nellymoser,
            7 => G711ALaw,
            8 => G711MuLaw,
            9 => Reserved,
            10 => AAC,
            11 => Speex,
            14 => MP38kHz,
            15 => DeviceSpecific,
            n => return Err(format!("error sound format: {}", n).into()),
        })
    }
}

#[derive(Debug)]
pub enum SoundRate {
    R5p5kHz = 0,
    R11kHz = 1,
    R22kHz = 2,
    R44kHz = 3,
}

impl TryFrom<u8> for SoundRate {
    type Error = Exception;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use SoundRate::*;
        Ok(match (value & 0b_0000_1100) >> 2 {
            0 => R5p5kHz,
            1 => R11kHz,
            2 => R22kHz,
            3 => R44kHz,
            n => return Err(format!("Invalid sound rate: {}", n).into()),
        })
    }
}

#[derive(Debug)]
pub enum SoundSize {
    S8Bit = 0,
    S16Bit = 1,
}

impl TryFrom<u8> for SoundSize {
    type Error = Exception;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use SoundSize::*;
        Ok(match (value & 0b_0000_0010) >> 1 {
            0 => S8Bit,
            1 => S16Bit,
            n => return Err(format!("Invalid sound size: {}", n).into()),
        })
    }
}

#[derive(Debug)]
pub enum SoundType {
    Mono = 0,
    Stereo = 1,
}

impl TryFrom<u8> for SoundType {
    type Error = Exception;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use SoundType::*;
        Ok(match value & 0b_0000_0001 {
            0 => Mono,
            1 => Stereo,
            n => return Err(format!("Invalid sound type: {}", n).into()),
        })
    }
}

#[derive(Debug)]
pub struct AudioDataHeader {
    pub sound_format: SoundFormat,
    pub sound_rate: SoundRate,
    pub sound_size: SoundSize,
    pub sound_type: SoundType,
}

impl TryFrom<u8> for AudioDataHeader {
    type Error = Exception;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let sound_format = SoundFormat::try_from(value)?;
        let sound_rate = SoundRate::try_from(value)?;
        let sound_size = SoundSize::try_from(value)?;
        let sound_type = SoundType::try_from(value)?;

        Ok(Self {
            sound_format,
            sound_rate,
            sound_size,
            sound_type,
        })
    }
}

#[derive(Debug)]
pub struct AudioData {
    pub header: AudioDataHeader,
    pub data: Bytes,
}

#[derive(Debug)]
pub enum VideoFrameType {
    KeyFrame,
    InterFrame,
    DisposableInterFrame,
    GeneratedKeyFrame,
    VideoInfoOrCommandFrame,
}

impl TryFrom<u8> for VideoFrameType {
    type Error = Exception;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use VideoFrameType::*;
        Ok(match (value & 0xf0) >> 4 {
            1 => KeyFrame,
            2 => InterFrame,
            3 => DisposableInterFrame,
            4 => GeneratedKeyFrame,
            5 => VideoInfoOrCommandFrame,
            n => return Err(format!("Invalid video frame type: {}", n).into()),
        })
    }
}

#[derive(Debug)]
pub enum CodecId {
    JPEG,
    SorensonH263,
    ScreenVideo,
    On2VP6,
    On2VP6WithAlpha,
    ScreenVideoVersion2,
    AVC,
}

impl TryFrom<u8> for CodecId {
    type Error = Exception;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use CodecId::*;
        Ok(match value & 0xf {
            1 => JPEG,
            2 => SorensonH263,
            3 => ScreenVideo,
            4 => On2VP6,
            5 => On2VP6WithAlpha,
            6 => ScreenVideoVersion2,
            7 => AVC,
            n => return Err(format!("Invalid codec id {}", n).into()),
        })
    }
}

#[derive(Debug)]
pub struct VideoDataHeader {
    pub frame_type: VideoFrameType,
    pub codec_id: CodecId,
}

impl TryFrom<u8> for VideoDataHeader {
    type Error = Exception;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let frame_type = VideoFrameType::try_from(value)?;
        let codec_id = CodecId::try_from(value)?;

        Ok(Self {
            frame_type,
            codec_id,
        })
    }
}

#[derive(Debug)]
pub struct VideoData {
    pub header: VideoDataHeader,
    pub data: Bytes,
}

#[derive(Debug)]
pub struct ScriptData {
    raw: Bytes,
}

#[derive(Debug)]
pub enum TagData {
    Audio(AudioData),
    Video(VideoData),
    Script(ScriptData),
    Reserved(Bytes),
}

#[derive(Debug)]
pub enum Field {
    PreTagSize(u32),
    Tag(Tag),
}

#[derive(Debug)]
pub enum CodecStatus {
    PreTagSize,
    Tag,
}

impl Default for CodecStatus {
    fn default() -> Self {
        Self::PreTagSize
    }
}

#[derive(Debug)]
pub struct BodyDecoder {
    status: CodecStatus,
}

impl Default for BodyDecoder {
    fn default() -> Self {
        Self {
            status: CodecStatus::default(),
        }
    }
}

impl Decoder for BodyDecoder {
    type Item = Field;
    type Error = Exception;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match &self.status {
            CodecStatus::PreTagSize => {
                if src.len() >= Self::PRE_TAG_SIZE_SIZE {
                    self.status = CodecStatus::Tag;
                    let pre_tag_size = src.get_u32();
                    Ok(Some(Field::PreTagSize(pre_tag_size)))
                } else {
                    Ok(None)
                }
            }
            CodecStatus::Tag => {
                if src.len() >= Self::TAG_HEADER_SIZE {
                    match &src[..Self::TAG_HEADER_SIZE] {
                        [tt, s1, s2, s3, t1, t2, t3, t0, 0, 0, 0] => {
                            let tag_type = match tt {
                                8 => TagType::Audio,
                                9 => TagType::Video,
                                18 => TagType::Script,
                                n => TagType::Reserved(*n),
                            };

                            // UI24 big endian
                            let data_size = u32::from_be_bytes([0, *s1, *s2, *s3]);

                            // t0: Extension of the timestamp field to form a SI32 value.
                            // This field represents the upper 8 bits, while the previous timestamp
                            // field represents the lower 24 bits of the time in milliseconds.
                            //
                            // t1~t3: time in milliseconds which the data in this tag applies.
                            // This value is relative to the first tag in the FLV file, which always
                            // has a timestamp of 0.
                            let timestamp = i32::from_be_bytes([*t0, *t1, *t2, *t3]);

                            let header = TagHeader {
                                tag_type,
                                data_size,
                                timestamp,
                            };

                            if src.len() >= data_size as usize + Self::TAG_HEADER_SIZE {
                                src.advance(Self::TAG_HEADER_SIZE);
                                let mut data_bytes = src.split_to(data_size as usize);

                                self.status = CodecStatus::PreTagSize;
                                match header.tag_type {
                                    TagType::Audio => Ok(Some(Field::Tag(Tag {
                                        header,
                                        data: TagData::Audio(AudioData {
                                            header: AudioDataHeader::try_from(data_bytes.get_u8())?,
                                            data: data_bytes.freeze(),
                                        }),
                                    }))),
                                    TagType::Video => Ok(Some(Field::Tag(Tag {
                                        header,
                                        data: TagData::Video(VideoData {
                                            header: VideoDataHeader::try_from(data_bytes.get_u8())?,
                                            data: data_bytes.freeze(),
                                        }),
                                    }))),
                                    TagType::Script => Ok(Some(Field::Tag(Tag {
                                        header,
                                        data: TagData::Script(ScriptData {
                                            raw: data_bytes.freeze(),
                                        }),
                                    }))),
                                    TagType::Reserved(_) => Ok(Some(Field::Tag(Tag {
                                        header,
                                        data: TagData::Reserved(data_bytes.freeze()),
                                    }))),
                                }
                            } else {
                                Ok(None)
                            }
                        }
                        n => Err(format!("Invalid tag header: {:?}", n).into()),
                    }
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl BodyDecoder {
    const PRE_TAG_SIZE_SIZE: usize = 32 / 8;
    const TAG_HEADER_SIZE: usize = (8 + 24 + 24 + 8 + 24) / 8;
}

pub async fn open_flv<P: AsRef<Path>>(
    path: P,
) -> Result<(u64, Header, FramedRead<BufReader<File>, BodyDecoder>), Exception> {
    let file = File::open(path).await?;

    let file_size = file.metadata().await?.len();

    let reader = BufReader::new(file);

    let mut reader = reader;
    let mut buf = [0u8; 9];
    let _len = reader.read_exact(&mut buf).await?;

    assert_eq!(_len, 9);

    let header = match buf {
        [b'F', b'L', b'V', version, type_, o1, o2, o3, o4] => {
            let offset = u32::from_be_bytes([o1, o2, o3, o4]);
            Header {
                version,
                type_,
                offset,
            }
        }
        _ => return Err("invalid flv file".into()),
    };

    let reader = FramedRead::new(reader, BodyDecoder::default());
    Ok((file_size, header, reader))
}
