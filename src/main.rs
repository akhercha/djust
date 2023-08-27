use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    name: String,
    value: u32,
}

use raylib::prelude::*;

use raylib::consts::KeyboardKey::{KEY_A, KEY_SPACE};
use raylib::core::audio::{Music, RaylibAudio};

use raylib::core::color::Color;

const MUSIC_NIGHTCORE: &str = "songs/light-it-up-nightcore.mp3";
const BUFFER_SIZE: usize = 4096 + 1024;

fn get_samples_buffer(music: &Music) -> Vec<i32> {
    const HEADER_SIZE: usize = 87;
    let buffer_size = BUFFER_SIZE + HEADER_SIZE;
    let mut my_buffer: Vec<i32> = vec![0; buffer_size];
    if music.stream.buffer.is_null() {
        panic!("Buffer is null!");
    }
    unsafe {
        my_buffer.copy_from_slice(std::slice::from_raw_parts(
            music.stream.buffer as *const i32,
            buffer_size,
        ));
    }
    my_buffer.drain(0..HEADER_SIZE);
    my_buffer
}

fn normalize(buffer: &mut Vec<i32>, max_value: i32) {
    let min_val = *buffer.iter().min().unwrap();
    let max_val = *buffer.iter().max().unwrap();

    for val in buffer.iter_mut() {
        *val = (((*val - min_val) as f64 / (max_val - min_val) as f64) * max_value as f64) as i32;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().size(860, 600).title("DJust").build();

    let mut ra: RaylibAudio = RaylibAudio::init_audio_device();
    let mut music = Music::load_music_stream(&thread, MUSIC_NIGHTCORE).unwrap();
    ra.play_music_stream(&mut music);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        {
            let my_buffer = get_samples_buffer(&music);

            if rl.is_key_pressed(KEY_A) {
                let serialized = serde_json::to_string(&my_buffer).unwrap();
                let mut file = File::create("output.json").expect("Unable to create file");
                file.write_all(serialized.as_bytes())
                    .expect("Unable to write data");
            }
        }

        // draws
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::new(12, 12, 13, 255));

            let my_buffer = get_samples_buffer(&music);

            let h = d.get_screen_height() as f32;
            let w = d.get_screen_width() as f32;
            let cell_w: f32 = w / my_buffer.len() as f32;

            for (i, v) in my_buffer.iter().enumerate() {
                let sample: f32 = *v as f32;
                if sample > 0.0 {
                    // Normalize values to fit screen
                    let t = sample / i32::MAX as f32;
                    let v_pos = Vector2 {
                        x: cell_w * i as f32,
                        y: h / 2.0 - h / 2.0 * t,
                    };
                    let v_size = Vector2 {
                        x: cell_w,
                        y: h / 2.0 * t,
                    };
                    d.draw_rectangle_v(v_pos, v_size, Color::new(245, 85, 73, 255));
                } else {
                    let t: f32 = sample / i32::MIN as f32;
                    let v_pos = Vector2 {
                        x: cell_w * i as f32,
                        y: h / 2.0,
                    };
                    let v_size = Vector2 {
                        x: cell_w,
                        y: h / 2.0 * t,
                    };
                    d.draw_rectangle_v(v_pos, v_size, Color::new(245, 85, 73, 255));
                }
            }
        }

        // event loop
        {
            if rl.is_key_pressed(KEY_SPACE) {
                if ra.is_music_stream_playing(&music) {
                    ra.pause_music_stream(&mut music);
                } else {
                    ra.resume_music_stream(&mut music);
                }
            }
        }

        // music stream
        {
            ra.update_music_stream(&mut music);
        }
    }
}
