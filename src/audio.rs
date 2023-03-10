use std::{f64::consts::PI, sync::atomic::AtomicUsize, thread};

use pa::{Stream, NonBlocking, Output};
use portaudio as pa;


const CHANNELS: i32 = 1;
const NUM_MILLISECONDS: i32 = 15200;
pub const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;

pub fn val_to_freq(val: f32) -> f32 {
    // 120. + 120. * val
    120. + 90. * val
 }
 

pub static mut FREQ: AtomicUsize = AtomicUsize::new(0);


pub fn play_sine(sample_rate: f64) -> Result<Stream<NonBlocking, Output<f32>>, pa::Error> {
    // Initialise sinusoidal wavetable.
    let mut sine = [0.0; TABLE_SIZE];
    for i in 0..TABLE_SIZE {
        sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;;
    }
    // transient distortion

    

    let mut left_phase = 0;

    let pa = pa::PortAudio::new()?;

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, sample_rate, FRAMES_PER_BUFFER)?;
    // we won't output out of range samples so don't bother clipping them.
    settings.flags = pa::stream_flags::CLIP_OFF;

    let mut f = 500.;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        for frame in 0..frames {
            buffer[frame] = sine[left_phase];
            
            let phase_delta = (TABLE_SIZE as f32 * f / sample_rate as f32).round();

            left_phase += phase_delta as usize;
            
            if left_phase >= TABLE_SIZE {
                left_phase -= TABLE_SIZE;
            }
        }
        f = unsafe {&FREQ}.load(std::sync::atomic::Ordering::SeqCst) as f32;
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    /*loop {
        thread::sleep(std::time::Duration::from_secs_f32(0.5));
        unsafe {&FREQ}.fetch_add(100, std::sync::atomic::Ordering::SeqCst);
        // unsafe {FREQ}.store(1, order)
    }*/

    Ok(stream)
}

#[test]
fn test() {
    
    play_sine(SAMPLE_RATE).unwrap();
    
    /*play_sine(SAMPLE_RATE - 1000.).unwrap();
    play_sine(SAMPLE_RATE - 2000.).unwrap();
    play_sine(SAMPLE_RATE - 3000.).unwrap();
    play_sine(SAMPLE_RATE - 4000.).unwrap();
    play_sine(SAMPLE_RATE - 5000.).unwrap();
    play_sine(SAMPLE_RATE - 6000.).unwrap();*/
}