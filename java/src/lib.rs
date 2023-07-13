pub(crate) mod dataset;

pub use dataset::Dataset;


// This is the interface to the JVM that we'll
// call the majority of our methods on.
use jni::JNIEnv;

// These objects are what you should use as arguments to your native function.
// They carry extra lifetime information to prevent them escaping from the
// current local frame (which is the scope within which local (temporary)
// references to Java objects remain valid)
use jni::objects::{JClass, JString, JObject};
use jni::sys::jlong;


// This `#[no_mangle]` keeps rust from "mangling" the name and making it unique
// for this crate. The name follow a strict naming convention so that the
// JNI implementation will be able to automatically find the implementation
// of a native method based on its name.
//
// The `'local` lifetime here represents the local frame within which any local
// (temporary) references to Java objects will remain valid.
//
// It's usually not necessary to explicitly name the `'local` input lifetimes but
// in this case we want to return a reference and show the compiler what
// local frame lifetime it is associated with.
//
// Alternatively we could instead return the `jni::sys::jstring` type instead
// which would represent the same thing as a raw pointer, without any lifetime,
// and at the end use `.into_raw()` to convert a local reference with a lifetime
// into a raw pointer.
#[no_mangle]
pub extern "system" fn Java_lance_Lance_hello<'local>(
    // Notice that this `env` argument is mutable. Any `JNIEnv` API that may
    // allocate new object references will take a mutable reference to the
    // environment.
    mut env: JNIEnv<'local>,
    // this is the class that owns our static method. Not going to be used, but
    // still needs to have an argument slot
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    // First, we have to get the string out of java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    // Then we have to create a new java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");
    output
}

#[repr(C)]
pub enum OptionJLong {
    None,
    Some(jlong),
}

use std::fmt;

impl fmt::Debug for OptionJLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptionJLong::None => write!(f, "None"),
            OptionJLong::Some(value) => write!(f, "Some({})", value),
        }
    }
}


#[no_mangle]
pub extern "system" fn Java_lance_Lance_newDataset<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    uri: JString<'local>,
) -> jlong {
    // println!("version: {:?}", version);
    // println!("block_size: {:?}", block_size);
    // println!("index_cache_size: {:?}", index_cache_size);

    // let version_option = if version.is_null() {
    //     None
    // } else {
    //     // 获取 Integer 类型的值
    //     let version_int = env.call_method(
    //         version,
    //         "get",
    //         "()I",
    //         &[],
    //     ).unwrap().i().map(|value| value as u64).unwrap();

    //     // 将 jint 类型的值转换为 Option<Integer> 类型的值
    //     Option::from(version_int)
    // };

    // let block_size_option = if block_size.is_null() {
    //     None
    // } else {
    //     // 获取 Integer 类型的值
    //     let block_size_int = env.call_method(
    //         block_size,
    //         "get",
    //         "()I",
    //         &[],
    //     ).unwrap().i().map(|value| value as usize).unwrap();

    //     // 将 jint 类型的值转换为 Option<Integer> 类型的值
    //     Option::from(block_size_int)
    // };

    // let index_cache_size_option = if index_cache_size.is_null() {
    //     None
    // } else {
    //     // 获取 Integer 类型的值
    //     let index_cache_size_int = env.call_method(
    //         index_cache_size,
    //         "get",
    //         "()I",
    //         &[],
    //     ).unwrap().i().map(|value| value as usize).unwrap();

    //     // 将 jint 类型的值转换为 Option<Integer> 类型的值
    //     Option::from(index_cache_size_int)
    // };
    
    let _uri = jstring_to_string(env, &uri);
    // let version_u64 = match version {
    //     OptionJLong::Some(v) => {
    //         Some(v.try_into().unwrap())
    //     }
    //     OptionJLong::None => None,
    // };
    // let block_usize = match block_size {
    //     OptionJLong::Some(v) => {
    //         Some(v.try_into().unwrap())
    //     }
    //     OptionJLong::None => None,
    // };
    // let index_cache_size_usize: Option<usize> = match index_cache_size {
    //     OptionJLong::Some(v) => {
    //         Some(v.try_into().unwrap())
    //     }
    //     OptionJLong::None => None,
    // };
    let dataset = Dataset::new(_uri, None, None, None);
    Box::into_raw(Box::new(dataset)) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_lance_Lance_countRows(
    env: JNIEnv,
    _class: JClass,
    dataset_ptr: jlong,
) -> jlong {
    println!("dataset_ptr: {:?}", dataset_ptr);
    let ds = &mut *(dataset_ptr as *mut Dataset);
    let count = ds.count_rows().unwrap();
    println!("count: {:?}", count);

    count as jlong
    
}



fn jstring_to_string(mut env: JNIEnv, jstr: &JString) -> String {
    // Convert jstring to CString
    let c_string = env.get_string(jstr).expect("Failed to convert jstring to CString");

    // Convert CString to Rust String
    let rust_string = c_string.to_string_lossy().into_owned();

    rust_string
}
