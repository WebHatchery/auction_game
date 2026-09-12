use macroquad_toolkit::audio::SoundManager;
use macroquad_toolkit::synth::{render_wav, SynthConfig, Voice, Wave};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SoundEffect {
    Button,
    Bid,
    Hammer,
    Week,
}

pub async fn load_audio() -> Result<SoundManager<SoundEffect>, String> {
    let mut audio = SoundManager::new();
    let config = SynthConfig {
        sample_rate: 22_050,
        master_gain: 0.24,
    };
    let effects = [
        (
            SoundEffect::Button,
            vec![Voice::tone(0.0, 0.055, 720.0, 0.42).wave(Wave::Triangle)],
        ),
        (
            SoundEffect::Bid,
            vec![
                Voice::tone(0.0, 0.09, 330.0, 0.46)
                    .glide(520.0)
                    .wave(Wave::Triangle),
                Voice::tone(0.06, 0.08, 660.0, 0.22),
            ],
        ),
        (
            SoundEffect::Hammer,
            vec![
                Voice::tone(0.0, 0.11, 392.0, 0.50).wave(Wave::Triangle),
                Voice::tone(0.09, 0.16, 196.0, 0.55).wave(Wave::Square),
            ],
        ),
        (
            SoundEffect::Week,
            vec![
                Voice::tone(0.0, 0.10, 260.0, 0.40),
                Voice::tone(0.10, 0.15, 390.0, 0.40),
            ],
        ),
    ];

    for (id, voices) in effects {
        let bytes = render_wav(&voices, &config, sound_seed(id));
        audio.load_sound_bytes(id, &bytes).await?;
    }
    Ok(audio)
}

fn sound_seed(effect: SoundEffect) -> u64 {
    match effect {
        SoundEffect::Button => 0xA11C_E001,
        SoundEffect::Bid => 0xA11C_E002,
        SoundEffect::Hammer => 0xA11C_E003,
        SoundEffect::Week => 0xA11C_E004,
    }
}
