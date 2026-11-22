pub mod engine;

#[cfg(target_os = "android")]
mod android_jni {
    use crate::engine::ENGINE;
    use std::ffi::{c_char, c_void, CString};

    #[repr(C)]
    pub struct JNINativeInterface {
        reserved: [*const c_void; 167],
        pub new_string_utf: unsafe extern "C" fn(
            env: *mut *const JNINativeInterface,
            bytes: *const c_char,
        ) -> *mut c_void,
    }

    pub type JNIEnv = *mut *const JNINativeInterface;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_cdh_rustandroid_MainActivity_stepEngine(
        env: JNIEnv,
        _class: *mut c_void,
        delta: i64,
    ) -> *mut c_void {
        let output = match ENGINE.lock() {
            Ok(mut eng) => eng.process_input(delta),
            Err(_) => String::from("Engine Mutex Poisoned"),
        };

        let c_str = CString::new(output).unwrap_or_else(|_| CString::new("UTF8 Error").unwrap());
        ((*(*env)).new_string_utf)(env, c_str.as_ptr())
    }
}
