use std::{
    f64::consts::PI,
    fs::File,
    io::{Seek, SeekFrom, Write},
};

const SAMPLE_RATE: f64 = 44_100.0;
const BIT_DEPTH: u32 = 16;
const MAX_AMPLITUDE: u32 = 2_u32.pow(BIT_DEPTH - 1) - 1;

struct SineOscillator {
    frequency: f64,
    amplitude: f64,
    angle: f64,
    offset: f64,
}

impl SineOscillator {
    pub fn new(frequency: f64, amplitude: f64) -> Self {
        SineOscillator {
            frequency,
            amplitude,
            angle: 0.0,
            offset: 2.0 * PI * frequency / SAMPLE_RATE,
        }
    }

    fn process(&mut self) -> f64 {
        let sample = self.amplitude * self.angle.sin();
        self.angle += self.offset;
        sample
    }
}

pub fn run() {
    let mut osc = SineOscillator::new(440.0, 0.5);
    let mut audio_file = File::create("audio.wav").unwrap();

    audio_file
        .write("RIFF".as_bytes())
        .expect("Riff to be written");

    audio_file
        .write("----".as_bytes())
        .expect("Size to be written");

    audio_file
        .write("WAVE".as_bytes())
        .expect("Wave to be written");

    audio_file
        .write("fmt ".as_bytes())
        .expect("Fmt to be written");

    audio_file
        .write(&16_u32.to_ne_bytes())
        .expect("Length of fmt to be written");

    audio_file
        .write(&1_u16.to_ne_bytes())
        .expect("Compression code to be written");

    audio_file
        .write(&1_u16.to_ne_bytes())
        .expect("Num channels rate to be written");

    audio_file
        .write(&44_100_u32.to_ne_bytes())
        .expect("Sample rate to be written");

    audio_file
        .write(&((SAMPLE_RATE as u32 * BIT_DEPTH / 8) as u32).to_ne_bytes())
        .expect("Bitrate to be written");

    audio_file
        .write(&2_u16.to_ne_bytes())
        .expect("Block align to be written (In this case can also be bitdepth / 8)");

    audio_file
        .write(&(BIT_DEPTH as u16).to_ne_bytes())
        .expect("Bit depth (sig bytes) to be written");

    audio_file
        .write("data".as_bytes())
        .expect("Data header to be written");

    audio_file
        .write("----".as_bytes())
        .expect("Data size to be written");

    let pre_audio_pos = audio_file.stream_position().unwrap();
    println!("Pre audio pos: {pre_audio_pos}");

    for _ in 0..(SAMPLE_RATE * 2.0) as u32 {
        let sample = osc.process();
        let int_sample: i16 = (sample * MAX_AMPLITUDE as f64) as i16;
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
}
