use raylib::consts::KeyboardKey::KEY_SPACE;
use raylib::core::audio::{Music, RaylibAudio};
use raylib::core::color::Color;
use raylib::prelude::*;

// const MUSIC: &str = "songs/headie-one-back-to-basics.ogg";
const MUSIC: &str = "songs/crankdat-higher.ogg";
// const MUSIC: &str = "songs/boom-boom-japan.ogg";
const BUFFER_SIZE: usize = 4096;

fn get_samples_buffer(music: &Music) -> Vec<i16> {
    const HEADER_SIZE: usize = 128 + 64;
    let buffer_size = BUFFER_SIZE + HEADER_SIZE;
    let mut my_buffer: Vec<i16> = vec![0; buffer_size];
    if music.stream.buffer.is_null() {
        panic!("Buffer is null!");
    }
    unsafe {
        my_buffer.copy_from_slice(std::slice::from_raw_parts(
            music.stream.buffer as *const i16,
            buffer_size,
        ));
    }
    my_buffer.drain(0..HEADER_SIZE);
    my_buffer
}

fn main() {
    let (mut rl, thread) = raylib::init().size(860, 600).title("DJust").build();

    let mut ra: RaylibAudio = RaylibAudio::init_audio_device();
    let mut music = Music::load_music_stream(&thread, MUSIC).unwrap();
    ra.play_music_stream(&mut music);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
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
                    let t = sample / i16::MAX as f32;
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
                    let t: f32 = sample / i16::MIN as f32;
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
