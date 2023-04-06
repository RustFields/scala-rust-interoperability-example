
use std::panic;

// This is the interface to the JVM that we'll call the majority of our
// methods on.
use jni::JNIEnv;
// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::JObject;
use jni::sys::jint;

// This keeps Rust from "mangling" the name and making it unique for this
// crate.
#[no_mangle]
pub extern "system" fn Java_io_github_filippovissani_example_Divider_divideBy(
    env: JNIEnv, // a wrapper from the Rust jni library
    object: JObject, // the object whose field we want to access
    denominator: jint,
) -> jint {
    let result = panic::catch_unwind(|| {
        println!("Hello from Rust!");
        // "numerator" is the name of the field and "I" is the type of the field ("I" stands for primitive int in Java bytecode)
        // not statically verified by the compiler and so can fail in runtime – thus the unwraps
        let numerator = env.get_field(object, "numerator", "I").unwrap().i().unwrap();
        println!("Printing from rust library. numerator: {}", numerator);
        println!("Printing from rust library. denominator: {}", denominator);
        numerator / denominator
    });
    /*
    In Rust, panics across FFI boundaries are an undefined behavior.
    To avoid doing that, we should catch them using catch_unwind.
    https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-panics
    */
    result.unwrap_or_else(|e| {
        let description = e
            .downcast_ref::<String>()
            .map(|e| &**e)
            .or_else(|| e.downcast_ref::<&'static str>().copied())
            .unwrap_or("Unknown error in native code.");
        // don't `unwrap` `throw_new`, another JVM exception might have already been thrown, in which case the `Result` is `Err`
        let _ = env.throw_new("java/lang/RuntimeException", description);
        -1 // some value to satisfy type checker, it won't be used
    })
}