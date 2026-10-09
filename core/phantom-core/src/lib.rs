use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jstring;

#[no_mangle]
pub extern "system" fn Java_com_phantom_aos_core_bridge_PtyBridge_nativePing(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = env.new_string("PhantomAOS Rust core is alive").unwrap();
    output.into_raw()
}
