use super::dj::draw_music;
use raylib::core::audio::{Music, RaylibAudio};
use raylib::core::color::Color;
use raylib::prelude::{measure_text, RaylibDraw, RaylibDrawHandle};

pub fn draw_text_in_center(d: &mut RaylibDrawHandle, text: &str, color: Color) {
    let screen_w = d.get_screen_width();
    let screen_h = d.get_screen_height();
    let txt_w = measure_text(text, 40);
    let txt_h: i32 = screen_h / 20;
    d.draw_text(
        text,
        screen_w / 2 - txt_w / 2,
        screen_h / 2 - txt_h / 2,
        40,
        color,
    );
}

pub fn draw_screen(d: &mut RaylibDrawHandle, ra: &mut RaylibAudio, music: &mut Option<Music>) {
    if let Some(music) = music.as_mut() {
        ra.update_music_stream(music);
        draw_music(d);
    } else {
        draw_text_in_center(d, "Drag&Drop music here", Color::WHITE);
    }
}
