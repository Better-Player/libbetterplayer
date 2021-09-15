use jni::{JNIEnv};
use jni::objects::{JString, JClass};
use jni::sys::jshortArray;
use log4j::{JavaLogger, LogLevel};

#[no_mangle]
pub extern "system" fn Java_net_betterplayer_betterplayer_libbetterplayer_LibBetterPlayer_tts(env: JNIEnv<'_>, _: JClass<'_>, tts: JString<'_>) -> jshortArray {
    let logger = JavaLogger::new(&env, "net.betterplayer.betterplayer.LibBetterPlayer").expect("Failed to create logger");
    let tts_text: String = match env.get_string(tts) {
        Ok(t) => t.into(),
        Err(e) => {
            logger.log(LogLevel::Error, format!("Failed to convert `tts` to String: {:?}", e)).expect("Failed to log");
            return std::ptr::null_mut();
        }
    };

    let audio = crate::ffi::espeakng::speak(&tts_text);
    let converted = crate::audio::espeakng_to_jda(audio);

    let arr = match env.new_short_array(converted.len() as i32) {
        Ok(a) => a,
        Err(e) => {
            logger.log(LogLevel::Error, format!("Failed to create new short array: {:?}", e)).expect("Failed to log");
            return std::ptr::null_mut();
        }
    };

    match env.set_short_array_region(arr, 0, &converted) {
        Ok(_) => {},
        Err(e) => {
            logger.log(LogLevel::Error, format!("Failed to set short array region: {:?}", e)).expect("Failed to log");
            return std::ptr::null_mut();
        }
    }

    arr
}