use convert_case::{Case, Casing};
use crate::java::ByteArrayOutputStream;
use crate::java_object;
use jni::errors::Result;
use jni::objects::JObject;
use jni::JNIEnv;
use strum_macros::Display;

java_object!(PdfDocument);
java_object!(PdfWriter);
java_object!(SolidLine);
java_object!(PageSize);
java_object!(Color);

#[derive(Clone, Display)]
pub enum ColorConstant {
    Black,
    Blue,
    Cyan,
    DarkGray,
    Gray,
    Green,
    LightGray,
    Magenta,
    Orange,
    Pink,
    Red,
    White,
    Yellow,
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

impl<'a> Color<'a> {
    pub fn from_rgb(r: f32, g: f32, b: f32, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/kernel/colors/DeviceRgb",
            "(FFF)V",
            &[r.into(), g.into(), b.into()]
        )?;
        Ok(Self(obj))
    }

    pub fn from_constant(constant: ColorConstant, env: &mut JNIEnv<'a>) -> Result<Self> {
        Ok(Self(constant.get_java_value(env)?))
    }
}

impl ColorConstant {
    pub(crate) fn get_java_value<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        let field_name = self.to_string().to_case(Case::ScreamingSnake);

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

macro_rules! page_size_constant {
    ($(($fun_name:ident, $field_name:expr)),*) => {
        impl<'a> PageSize<'a> {
            $(
                pub fn $fun_name(env: &mut JNIEnv<'a>) -> Result<Self> {
                    let obj = env
                        .get_static_field(
                            "com/itextpdf/kernel/geom/PageSize",
                            $field_name,
                            "Lcom/itextpdf/kernel/geom/PageSize;",
                        )?
                        .l()?;
                    Ok(Self(obj))
                }
            )*
        }
    }
}

page_size_constant!(
    (new_a0, "A0"),
    (new_a1, "A1"),
    (new_a2, "A2"),
    (new_a3, "A3"),
    (new_a4, "A4"),
    (new_a5, "A5"),
    (new_a6, "A6"),
    (new_a7, "A7"),
    (new_a8, "A8"),
    (new_a9, "A9"),
    (new_a10, "A10"),
    (new_b0, "B0"),
    (new_b1, "B1"),
    (new_b2, "B2"),
    (new_b3, "B3"),
    (new_b4, "B4"),
    (new_b5, "B5"),
    (new_b6, "B6"),
    (new_b7, "B7"),
    (new_b8, "B8"),
    (new_b9, "B9"),
    (new_b10, "B10"),
    (new_executive, "EXECUTIVE"),
    (new_ledger, "LEDGER"),
    (new_legal, "LEGAL"),
    (new_letter, "LETTER"),
    (new_tabloid, "TABLOID")
);

impl<'a> PageSize<'a> {
    pub fn new(width: f32, height: f32, env: &mut JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.new_object("Lcom/itextpdf/kernel/geom/PageSize;", "(FF)V", &[width.into(), height.into()])?))
    }

    pub fn get_width(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        Ok(env.call_method(self, "getWidth", "()F", &[])?.f()?)
    }

    pub fn get_height(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        Ok(env.call_method(self, "getHeight", "()F", &[])?.f()?)
    }
}
