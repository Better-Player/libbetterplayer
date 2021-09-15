use crate::ffi::espeakng::Spoken;
use fon::Audio;
use fon::stereo::Stereo16;
use fon::mono::Mono16;

/// The samplerate expected by JDA
const JDA_SAMPLE_RATE: u32 = 48000;

/// Convert audio produced by espeakng to the spec required by JDA
pub fn espeakng_to_jda(spoken: Spoken) -> Vec<i16> {
    // espeak-ng produces 16 bit mono PCM audio
    // Source: https://github.com/espeak-ng/espeak-ng/issues/88#issuecomment-177192699
    let source = Audio::<Mono16>::with_i16_buffer(spoken.sample_rate, spoken.wav);

    // JDA wants 16 bit stero PCM audio
    // Source: https://ci.dv8tion.net/job/JDA/javadoc/net/dv8tion/jda/api/audio/AudioReceiveHandler.html#OUTPUT_FORMAT
    let mut target = Audio::<Stereo16>::with_stream(JDA_SAMPLE_RATE, &source);
    target.as_i16_slice().to_vec()
}

pub fn generate_pause(len: i32) -> Vec<i16> {
    let mut audio = Audio::<Mono16>::with_silence(JDA_SAMPLE_RATE, len as usize);
    audio.as_i16_slice().to_vec()
}