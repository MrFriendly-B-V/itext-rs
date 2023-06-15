use crate::java::{BufferedImage, ByteArrayInputStream};
use crate::java_object;
use jni::errors::Result;
use jni::JNIEnv;

java_object!(ImageIO);
java_object!(ImageInputStream);

impl<'a> ImageIO<'a> {
    pub fn create_image_input_stream(
        byte_stream: ByteArrayInputStream<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<ImageInputStream<'a>> {
        let obj = env
            .call_static_method(
                "javax/imageio/ImageIO",
                "createImageInputStream",
                "(Ljava/lang/Object;)Ljavax/imageio/stream/ImageInputStream;",
                &[(&byte_stream).into()],
            )?
            .l()?;
        Ok(ImageInputStream(obj))
    }

    pub fn read_image_input_stream(
        image_input_stream: ImageInputStream<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<BufferedImage<'a>> {
        let obj = env
            .call_static_method(
                "javax/imageio/ImageIO",
                "read",
                "(Ljavax/imageio/stream/ImageInputStream;)Ljava/awt/image/BufferedImage;",
                &[(&image_input_stream).into()],
            )?
            .l()?;
        Ok(BufferedImage(obj))
    }
}

impl<'a> ImageInputStream<'a> {
    pub fn new_from_byte_stream(
        byte_stream: ByteArrayInputStream<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<Self> {
        ImageIO::create_image_input_stream(byte_stream, env)
    }
}
