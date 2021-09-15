use jni::JNIEnv;
use jni::sys::{jbyteArray, jshortArray};
use jni::objects::{JClass, ReleaseMode};
use std::ffi::c_void;
use libc::size_t;

#[no_mangle]
pub extern "system" fn Java_net_betterplayer_betterplayer_libbetterplayer_LibBetterPlayer_toByteArray(env: JNIEnv<'_>, _: JClass<'_>, shorts: jshortArray) -> jbyteArray {
    let input_len = env.get_array_length(shorts).unwrap();
    let input_elems = env.get_short_array_elements(shorts, ReleaseMode::NoCopyBack).unwrap();

    let output = env.new_byte_array(input_len * 2).unwrap();
    let output_elems = env.get_byte_array_elements(output, ReleaseMode::NoCopyBack).unwrap();

    unsafe { libc::memcpy(output_elems as *mut c_void, input_elems as *mut c_void, (input_len * 2) as size_t) }

    output
}