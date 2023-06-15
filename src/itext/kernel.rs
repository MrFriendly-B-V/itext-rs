use crate::java::ByteArrayOutputStream;
use crate::java_object;
use jni::errors::Result;
use jni::objects::JObject;
use jni::JNIEnv;

java_object!(PdfDocument);
java_object!(PdfWriter);
java_object!(SolidLine);
java_object!(PageSize);

pub enum ColorConstant {
    Black,
}

impl<'a> PdfDocument<'a> {
    pub fn new(writer: &PdfWriter<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/kernel/pdf/PdfDocument",
            "(Lcom/itextpdf/kernel/pdf/PdfWriter;)V",
            &[(&writer).into()],
        )?;
        Ok(Self(obj))
    }

    pub fn get_default_page_size(&self, env: &mut JNIEnv<'a>) -> Result<PageSize<'a>> {
        let obj = env
            .call_method(
                self,
                "getDefaultPageSize",
                "()Lcom/itextpdf/kernel/geom/PageSize;",
                &[],
            )?
            .l()?;
        Ok(PageSize(obj))
    }
}

impl<'a> PdfWriter<'a> {
    pub fn new(byte_stream: &ByteArrayOutputStream<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/kernel/pdf/PdfWriter",
            "(Ljava/io/OutputStream;)V",
            &[(&byte_stream).into()],
        )?;
        Ok(Self(obj))
    }
}

impl<'a> SolidLine<'a> {
    pub fn new(line_width: f32, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/kernel/pdf/canvas/draw/SolidLine",
            "(F)V",
            &[line_width.into()],
        )?;
        Ok(Self(obj))
    }

    pub fn set_color(&self, color: ColorConstant, env: &mut JNIEnv<'a>) -> Result<()> {
        let color_j = color.get_java_value(env)?;
        env.call_method(
            self,
            "setColor",
            "(Lcom/itextpdf/kernel/colors/Color;)V",
            &[(&color_j).into()],
        )?;
        Ok(())
    }
}

impl ColorConstant {
    pub(crate) fn get_java_value<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        let field_name = match self {
            Self::Black => "BLACK",
        };

        let obj = env
            .get_static_field(
                "com/itextpdf/kernel/colors/ColorConstants",
                field_name,
                "Lcom/itextpdf/kernel/colors/Color;",
            )?
            .l()?;
        Ok(obj)
    }
}

impl<'a> PageSize<'a> {
    pub fn get_width(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        Ok(env.call_method(self, "getWidth", "()F", &[])?.f()?)
    }
}
