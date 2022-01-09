use crate::core::error::DataError;
use crate::core::sound_id::SoundId;
use crate::drawing::sff::data::{FileReader, DataReader, BufferReader};

use super::structs::{FileHeader, SubHeader, WavHeader, WavSound};
use gdnative::api::file::File;
use gdnative::api::audio_stream_sample::AudioStreamSample;
use gdnative::prelude::*;

pub fn read_sounds(path: &str) -> Result<Vec<WavSound>, DataError> {
    let file = File::new();
    let open_result = file.open(path, File::READ);

    if let Err(detail) = open_result {
        return Result::Err(DataError::new(format!(
            "Error opening snd file: {}",
            detail
        )));
    }

    let mut reader = FileReader::new(&file);
    let head = FileHeader::read(&mut reader);

    if head.signature != "ElecbyteSnd" {
        file.close();
        return Result::Err(DataError::new(format!(
            "Snd invalid signature: {}",
            head.signature
        )));
    }

    file.seek(head.subheader_offset as i64);

    let mut result = Vec::new();

    for _ in 0..4096 {
        if reader.eof() {
            break;
        }

        let subheader = SubHeader::read(&mut reader);

        if subheader.length == 0 {
            break;
        }

        let tmp_arr = reader.get_buffer(subheader.length as usize);
        let mut tmp_arr_reader = BufferReader::new(&tmp_arr);
        let wav_header = WavHeader::read(&mut tmp_arr_reader);

        let stream = AudioStreamSample::new();
        stream.set_data(to_signed(&tmp_arr));
        stream.set_mix_rate(wav_header.sample_rate as i64);
        stream.set_stereo(wav_header.num_channels == 2);
        match wav_header.bits_per_sample {
            8 => stream.set_format(AudioStreamSample::FORMAT_8_BITS),
            16 => stream.set_format(AudioStreamSample::FORMAT_16_BITS),
            _ => {
                godot_warn!("invalid bits_per_sample: {}", wav_header.bits_per_sample);
            }
        };

        result.push(WavSound {
            soundid: SoundId::new(subheader.groupno as i16, subheader.soundno as i16),
            stream: stream.into_shared()
        });

        if subheader.next > 0 && subheader.next < file.get_len() as u32 {
            file.seek(subheader.next as i64);
        } else {
            break;
        }
    }

    file.close();

    Result::Ok(result)
}


fn to_signed(source: &[u8]) -> ByteArray {
    source
        .iter()
        .map(|value| ((*value as i16) - 128) as u8)
        .collect()
}
