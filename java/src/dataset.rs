use std::sync::Arc;
use tokio::runtime::Runtime;
use std::io;

use arrow_schema::Schema as ArrowSchema;
use lance::{Error, Result};
use lance::dataset::ReadParams;

const DEFAULT_INDEX_CACHE_SIZE: usize = 256;


use lance::dataset::{
    scanner::Scanner as LanceScanner, Dataset as LanceDataset, Version, WriteMode, WriteParams,
};


// 定义一个 Rust 结构体
pub struct Dataset {
    uri: String,
    ds: Arc<LanceDataset>,
    rt: Arc<Runtime>,
}

impl Dataset {
    pub fn new(
        uri: String,
        version: Option<u64>,
        block_size: Option<usize>,
        index_cache_size: Option<usize>,
    ) -> Result<Self> {
        println!("uri: {:?}", uri);
        println!("version: {:?}", version);
        println!("block_size: {:?}", block_size);
        println!("index_cache_size: {:?}", index_cache_size);
        let rt = Runtime::new()?;
        let params = ReadParams {
            block_size,
            index_cache_size: index_cache_size.unwrap_or(DEFAULT_INDEX_CACHE_SIZE),
            session: None,
            store_options: None,
        };
        keyboard_confirm();
        let dataset = rt.block_on(async {
            if let Some(ver) = version {
                println!("checkout_with_params: uri={}, ver={}", uri, ver);
                LanceDataset::checkout_with_params(uri.as_str(), ver, &params).await
            } else {
                println!("open_with_params: uri={}, params:{:?}", uri, params);
                LanceDataset::open_with_params(uri.as_str(), &params).await
            }
        });
        println!("after dataset: uri={}", uri);
        match dataset {
            Ok(ds) => Ok(Self {
                uri,
                ds: Arc::new(ds),
                rt: Arc::new(rt),
            }),
            Err(err) => Err(Error::IO { message: err.to_string() }),
        }
        
    }

    pub fn schema(self) -> ArrowSchema {
        ArrowSchema::from(self.ds.schema())
    }

    pub fn count_rows(&self) -> Result<usize> {
        self.rt.block_on(async {
            self.ds
                .count_rows()
                .await
                .map_err(|err| Error::IO{message: err.to_string()})
        })
    }
}


pub fn keyboard_confirm() {
    let mut input = String::new();
    println!("Press enter to continue...");
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    println!("You entered: {}", input);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset() {
        let uri: String = "/home/weijie-li/data/9940C".to_string();
        let dataset = Dataset::new(uri.clone(), None, None, None).unwrap();
        assert_eq!(dataset.uri, uri);
        assert_eq!(dataset.count_rows().unwrap(), 329);
    }
}

// use jni::{
//     objects::{AutoLocal, JClass, JObject, JValue},
//     sys::jobject,
//     JNIEnv,
// };

// // 实现 JavaClass 和 IntoJavaObject trait
// impl<'a> jni::JavaClass for Dataset {
//     const SIGNATURE: &'static str = "com/lance/Dataset"; // Java 类的签名
// }

// impl<'a> jni::IntoJavaObject<'a> for Dataset {
//     fn into_java_object(self, env: &'a JNIEnv) -> jni::objects::AutoLocal<'a, jobject> {
//         // 创建一个 Java 对象
//         let java_obj = env.new_object(
//             "com/lance/Dataset",
//             "()V",
//             &[],
//         ).expect("failed to create Java object");

//         // 将 Rust 结构体的字段转换为 Java 对象
//         env.set_field(
//             java_obj.into_inner(),
//             "uri",
//             "Ljava/lang/String;",
//             JValue::Object(JObject::from(env.new_string(self.uri).expect("failed to create Java string"))),
//         ).expect("failed to set uri");

//         // 将 Arc<LanceDataset> 转换为 Java 对象
//         let ds_obj = env.new_object(
//             "com/lance/LanceDataset",
//             "()V",
//             &[],
//         ).expect("failed to create Java object");

//         let ds_ptr: *mut LanceDataset = Arc::into_raw(self.ds);
//         env.set_long_field(
//             ds_obj.into_inner(),
//             "ptr",
//             ds_ptr as jlong,
//         ).expect("failed to set ptr");

//         env.set_field(
//             java_obj.into_inner(),
//             "ds",
//             "Lcom/lance/LanceDataset;",
//             JValue::Object(ds_obj),
//         ).expect("failed to set ds");

//         // 将 Arc<Runtime> 转换为 Java 对象
//         let rt_obj = env.new_object(
//             "com/lance/Runtime",
//             "()V",
//             &[],
//         ).expect("failed to create Java object");

//         let rt_ptr: *mut Runtime = Arc::into_raw(self.rt);
//         env.set_long_field(
//             rt_obj.into_inner(),
//             "ptr",
//             rt_ptr as jlong,
//         ).expect("failed to set ptr");

//         env.set_field(
//             java_obj.into_inner(),
//             "rt",
//             "Lcom/lance/Runtime;",
//             JValue::Object(rt_obj),
//         ).expect("failed to set rt");

//         // 返回一个 AutoLocal 实例
//         AutoLocal::new(env, java_obj.into_inner()).expect("failed to create AutoLocal")
//     }
// }