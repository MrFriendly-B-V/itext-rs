use jni::JNIEnv;
use jni::objects::JByteArray;
use jni::sys::{jbyte, jsize};
use jni::errors::Result;
use crate::java_object;
use crate::javax::{ImageInputStream, ImageIO};

java_object!(ByteArrayOutputStream);
java_object!(ByteArrayInputStream);
java_object!(BufferedImage);

impl<'a>ByteArrayOutputStream<'a> {
    pub fn new(env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object("java/io/ByteArrayOutputStream", "()V", &[])?;
        Ok(Self(obj))
    }

    pub fn to_byte_array(&self, env: &mut JNIEnv<'a>) -> Result<Vec<u8>> {
        let obj = env.call_method(self, "toByteArray", "()[B", &[])?.l()?;
        let obj_arr: JByteArray = obj.into();
        let size = env.get_array_length(&obj_arr)?;

        let mut buf = vec![0; size as usize];
        env.get_byte_array_region(&obj_arr, 0, &mut buf)?;

        let buf = buf.into_iter()
            .map(|x| x as u8)
            .collect::<Vec<_>>();
        Ok(buf)
    }
}

impl<'a> ByteArrayInputStream<'a> {
    pub fn new(bytes: &[u8], env: &mut JNIEnv<'a>) -> Result<Self> {
        let byte_arr = env.new_byte_array(bytes.len() as jsize)?;
        let jbytes = bytes.into_iter().map(|x| *x as jbyte).collect::<Vec<_>>();
        env.set_byte_array_region(&byte_arr, 0, &jbytes)?;

        let obj = env.new_object("java/io/ByteArrayInputStream", "([B)V", &[(&byte_arr).into()])?;
        Ok(Self(obj))
    }
}

impl<'a> BufferedImage<'a> {
    pub fn new_from_image_input_stream(image_input_stream: ImageInputStream<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        ImageIO::read_image_input_stream(image_input_stream, env)
    }

    pub fn get_width(&self, env: &mut JNIEnv<'a>) -> Result<i32> {
        Ok(env.call_method(self, "getWidth", "()I", &[])?.i()?)
    }

    pub fn get_height(&self, env: &mut JNIEnv<'a>) -> Result<i32> {
        Ok(env.call_method(self, "getHeight", "()I", &[])?.i()?)
    }
}