use crate::java_object;
use jni::errors::Result;
use jni::sys::{jbyte, jsize};
use jni::JNIEnv;

java_object!(ImageData);

impl<'a> ImageData<'a> {
    pub fn new(env: &mut JNIEnv<'a>, bytes: &[u8]) -> Result<Self> {
        let jbytes = bytes.into_iter().map(|x| *x as jbyte).collect::<Vec<_>>();

        let byte_arr = env.new_byte_array(jbytes.len() as jsize)?;
        env.set_byte_array_region(&byte_arr, 0, &jbytes)?;

        let obj = env
            .call_static_method(
                "com/itextpdf/io/image/ImageDataFactory",
                "create",
                "([B)Lcom/itextpdf/io/image/ImageData;",
                &[(&byte_arr).into()],
            )?
            .l()?;
        Ok(Self(obj))
    }
}
