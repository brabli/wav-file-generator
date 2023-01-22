mod sine;

use sine::SineOscillator;
use std::{
    fs::File,
    io::{Result, Seek, SeekFrom, Write},
};

fn write_str_bytes(file: &mut File, string: &str) -> Result<()> {
    let bytes = string.as_bytes();
    file.write_all(bytes)
        .expect(format!("{string} to be written to file").as_str());
    Ok(())
}

fn write_int_bytes(file: &mut File, int: u32, num_bytes: u8) -> Result<()> {
    let bytes = int.to_ne_bytes();
    let bytes_to_write = bytes
        .get(..num_bytes as usize)
        .expect(format!("Couldn't convert {int} into {num_bytes} bytes!").as_str());

    file.write_all(&bytes_to_write)
        .expect(format!("{int} to be written to file").as_str());
    Ok(())
}

pub fn run() -> Result<()> {
    let mut osc = SineOscillator::new(440.0, 0.5);
    let mut audio_file = File::create("audio.wav").unwrap();

    write_str_bytes(&mut audio_file, "RIFF")?;
    write_str_bytes(&mut audio_file, "----")?;
    write_str_bytes(&mut audio_file, "WAVE")?;
    write_str_bytes(&mut audio_file, "fmt ")?;

    write_int_bytes(&mut audio_file, 16, 4)?; // fmt length
    write_int_bytes(&mut audio_file, 1, 2)?; // Compression code
    write_int_bytes(&mut audio_file, 1, 2)?; // Number of channels
    write_int_bytes(&mut audio_file, 44_100, 4)?; // Sample rate

    let bitrate = (sine::SAMPLE_RATE as u32 * sine::BIT_DEPTH / 8) as u32; // 88_200
    write_int_bytes(&mut audio_file, bitrate, 4)?; // Bitrate

    write_int_bytes(&mut audio_file, 2, 2)?; // Block align (bitdepth / 8 in this case)

    write_int_bytes(&mut audio_file, sine::BIT_DEPTH, 2)?; // Bit depth (significant bytes)

    write_str_bytes(&mut audio_file, "data")?;
    write_str_bytes(&mut audio_file, "----")?;

    let pre_audio_pos = audio_file.stream_position().unwrap();
    println!("Pre audio pos: {pre_audio_pos}");

    for _ in 0..(sine::SAMPLE_RATE * 2.0) as u32 {
        let sample = osc.process();
        let int_sample: i16 = (sample * sine::MAX_AMPLITUDE as f64) as i16;
        audio_file.write_all(&int_sample.to_ne_bytes()).unwrap();
    }

    let post_audio_pos = audio_file.stream_position().unwrap();
    println!("Post audio pos: {post_audio_pos}");

    audio_file.seek(SeekFrom::Start(pre_audio_pos - 4)).unwrap();
    audio_file
        .write(&(post_audio_pos as u32 - pre_audio_pos as u32).to_ne_bytes())
        .unwrap();

    let stream_len = audio_file.seek(SeekFrom::End(0)).unwrap();
    println!("Filesize: {}", stream_len);

    audio_file.seek(SeekFrom::Start(4)).unwrap();
    audio_file
        .write(&(post_audio_pos as u32 - 8).to_ne_bytes())
        .unwrap();

    println!("Current pos: {}", audio_file.stream_position().unwrap());

    Ok(())
}
