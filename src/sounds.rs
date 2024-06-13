use macroquad::audio::{play_sound, play_sound_once, PlaySoundParams, Sound};

pub async fn play_music(sound: Sound) {
    // Play the background music
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );
}

pub async fn play_collision_sound(sound: Sound) {
    // Play the collision sound
    play_sound_once(sound);
}
