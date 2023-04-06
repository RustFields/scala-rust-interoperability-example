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
pub extern "system" fn Java_io_github_filippovissani_example_Divider_divideBy<'local>(
    mut env: JNIEnv<'local>, // a wrapper from the Rust jni library
    object: JObject<'local>, // the object whose field we want to access
    denominator: jint,
) -> jint {
    println!("Hello from Rust!");
    // "numerator" is the name of the field and "I" is the type of the field ("I" stands for primitive int in Java bytecode)
    // not statically verified by the compiler and so can fail in runtime – thus the unwraps
    let numerator = env.get_field(object, "numerator", "I").unwrap().i().unwrap();
    println!("Printing from rust library. numerator: {}", numerator);
    println!("Printing from rust library. denominator: {}", denominator);
    let result = numerator / denominator;
    result
}