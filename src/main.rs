use itertools::Itertools;
use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
use tinyaudio::prelude::*;

fn main() {
    // Setup the audio output.
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    // Buffer for the audio output.
    let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count];
    let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count];

    // Load the SoundFont.
    let mut sf2 = File::open("TimGM6mb.sf2").unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(params.sample_rate as i32);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    // Play a note to bend.
    synthesizer.process_midi_message(4, 0xC0, 30, 0);
    synthesizer.note_on(4, 60, 100);

    // The counter to control the bend.
    let counter = Arc::new(Mutex::new(0));

    // Start the audio output.
    let _device = run_output_device(params, {
        move |data| {
            // Set data2.
            let mut i = counter.lock().unwrap();
            let mut data2 = *i % 32;
            if data2 > 16 {
                data2 = 32 - data2;
            }
            data2 += 64;
            println!("data2: {}", data2);

            // Do bend.
            synthesizer.process_midi_message(4, 224, 0, data2);

            synthesizer.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }

            *i += 1;
        }
    })
    .unwrap();

    // Wait for 5 seconds.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
