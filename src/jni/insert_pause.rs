use jni::{JNIEnv};
use jni::sys::{jshortArray, jint, jsize};
use log4j::{JavaLogger, LogLevel};
use jni::objects::JClass;

#[no_mangle]
pub extern "system" fn Java_net_betterplayer_betterplayer_libbetterplayer_LibBetterPlayer_insertPause(env: JNIEnv<'_>, _: JClass<'_>, original: jshortArray, pause_len: jint) -> jshortArray {
    let logger = JavaLogger::new(&env, "net.betterplayer.betterplayer.LibBetterPlayer").expect("Failed to create logger");
    let original_len = match env.get_array_length(original) {
        Ok(l) => l,
        Err(e) => {
            logger.log(LogLevel::Error, format!("Failed to get length of array 'original': {:?}", e)).expect("Failed to log");
            return std::ptr::null_mut();
        }
    };

    let mut buf: Vec<i16> = Vec::with_capacity(original_len as usize);
    match env.get_short_array_region(original, 0, &mut buf) {
        Ok(_) => {},
        Err(e) => {
            logger.log(LogLevel::Error, format!("Failed to get region of array 'original': {:?}", e)).expect("Failed to log");
            return std::ptr::null_mut();
        }
    }

    let pause = crate::audio::generate_pause(pause_len);
    buf.extend(pause);

    let new_arr = match env.new_short_array(buf.len() as jsize) {
        Ok(a) => a,
        Err(e) => {
            logger.log(LogLevel::Error, format!("Failed to create array 'new_arr': {:?}", e)).expect("Failed to log");
            return std::ptr::null_mut();
        }
    };

    match env.set_short_array_region(new_arr, 0, &buf) {
        Ok(_) => {},
        Err(e) => {
            logger.log(LogLevel::Error, format!("Failed to set region of array 'new_arr': {:?}", e)).expect("Failed to log");
            return std::ptr::null_mut();
        }
    }

    new_arr
}