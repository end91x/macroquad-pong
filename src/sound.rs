use macroquad::audio::{play_sound, play_sound_once, PlaySoundParams, Sound};

pub async fn play_music(sound: Sound) {
    // Play the background music
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume: 0.1,
        },
    );
}

pub async fn play_collision_sound(sound: Sound) {
    play_sound_once(sound);
}
