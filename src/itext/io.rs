use crate::java_object;
use convert_case::{Case, Casing};
use jni::errors::Result;
use jni::objects::JObject;
use jni::sys::{jbyte, jsize};
use jni::JNIEnv;
use strum_macros::Display;

java_object!(ImageData);
java_object!(FontProgram);
java_object!(FontProgramFactory);

impl<'a> ImageData<'a> {
    pub fn new(env: &mut JNIEnv<'a>, bytes: &[u8]) -> Result<Self> {
        let jbytes = bytes.iter().map(|x| *x as jbyte).collect::<Vec<_>>();

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

    pub fn get_width(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        let obj = env.call_method(self, "getWidth", "()F", &[])?.f()?;
        Ok(obj)
    }

    pub fn get_height(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        let obj = env.call_method(self, "getHeight", "()F", &[])?.f()?;
        Ok(obj)
    }
}

#[derive(Debug, Clone, Display)]
pub enum StandardFont {
    Courier,
    CourierBold,
    CourierBoldoblique,
    CourierOblique,
    Helvetica,
    HelveticaBold,
    HelveticaBoldoublique,
    HelveticaOublique,
    Symbol,
    TimesBold,
    TimesBolditalic,
    TimesItalic,
    TimesRoman,
    Zapfdingbats,
}

impl StandardFont {
    pub(crate) fn get_java_value<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        let field_name = self.to_string().to_case(Case::ScreamingSnake);
        env.get_static_field(
            "com/itextpdf/io/font/constants/StandardFonts",
            field_name,
            "Ljava/lang/String;",
        )?
        .l()
    }
}

impl<'a> FontProgramFactory<'a> {
    pub fn new_from_ttf(bytes: &[u8], env: &mut JNIEnv<'a>) -> Result<FontProgram<'a>> {
        let byte_array = env.new_byte_array(bytes.len() as jsize)?;
        env.set_byte_array_region(
            &byte_array,
            0,
            &bytes.iter().map(|byte| *byte as jbyte).collect::<Vec<_>>(),
        )?;

        let object = env
            .call_static_method(
                "com/itextpdf/io/font/FontProgramFactory",
                "createFont",
                "([B)Lcom/itextpdf/io/font/FontProgram;",
                &[(&byte_array).into()],
            )?
            .l()?;
        Ok(FontProgram(object))
    }
}

#[derive(Debug, Clone, Display)]
pub enum PdfEncodings {
    Cp1250,
    Cp1252,
    Cp1253,
    Cp1257,
    IdentityH,
    IdentityV,
    Macroman,
    PdfDocEncoding,
    Symbol,
    UnicodeBig,
    UnicodeBigUnmarked,
    Utf8,
    Winansi,
    Zapfdingbats,
}

impl PdfEncodings {
    pub(crate) fn get_java_value<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        let field_name = self.to_string().to_case(Case::ScreamingSnake);
        env.get_static_field(
            "com/itextpdf/io/font/PdfEncodings",
            field_name,
            "Ljava/lang/String;",
        )?
        .l()
    }
}
