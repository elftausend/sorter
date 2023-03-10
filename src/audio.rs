use std::f64::consts::PI;

use portaudio as pa;


const CHANNELS: i32 = 1;
const NUM_MILLISECONDS: i32 = 100;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;


fn play_sine(sample_rate: f64) -> Result<(), pa::Error> {
    // Initialise sinusoidal wavetable.
    let mut sine = [0.0; TABLE_SIZE];
    for i in 0..TABLE_SIZE {

        let i_val = if i <= 10 {
            i as f64 * 1.
        } else {
            i as f64
        };

        let mut sine_val = (i_val / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
        sine[i] = sine_val;
    }
    // transient distortion


    let mut left_phase = 0;

    let pa = pa::PortAudio::new()?;

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, sample_rate, FRAMES_PER_BUFFER)?;
    // we won't output out of range samples so don't bother clipping them.
    settings.flags = pa::stream_flags::CLIP_OFF;

    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            buffer[idx] = sine[left_phase];
            // buffer[idx + 1] = sine[right_phase];
            
            left_phase += 1;
            if left_phase >= TABLE_SIZE {
                left_phase -= TABLE_SIZE;
            }
            
            idx += 1;
        }
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    pa.sleep(NUM_MILLISECONDS);

    stream.stop()?;
    stream.close()?;

    Ok(())
}

#[test]
fn test() {
    play_sine(SAMPLE_RATE).unwrap();
    play_sine(SAMPLE_RATE - 1000.).unwrap();
    play_sine(SAMPLE_RATE - 2000.).unwrap();
    play_sine(SAMPLE_RATE - 3000.).unwrap();
    play_sine(SAMPLE_RATE - 4000.).unwrap();
    play_sine(SAMPLE_RATE - 5000.).unwrap();
    play_sine(SAMPLE_RATE - 6000.).unwrap();
}