use crate::itext::io::{FontProgram, ImageData, PdfEncodings, StandardFont};
use crate::java::ByteArrayOutputStream;
use crate::java_object;
use convert_case::{Case, Casing};
use jni::errors::Result;
use jni::objects::{JObject, JValueGen};
use jni::sys::jboolean;
use jni::JNIEnv;
use strum_macros::Display;

java_object!(PdfDocument);
java_object!(PdfWriter);
java_object!(SolidLine);
java_object!(PageSize);
java_object!(Color);
java_object!(PdfFont);
java_object!(PdfFontFactory);
java_object!(PdfCanvas);
java_object!(PdfPage);
java_object!(PdfExtGState);
java_object!(PdfFormXObject);
java_object!(Rectangle);

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

    pub fn get_page(&self, page_num: i32, env: &mut JNIEnv<'a>) -> Result<PdfPage<'a>> {
        let obj = env
            .call_method(
                self,
                "getPage",
                "(I)Lcom/itextpdf/kernel/pdf/PdfPage;",
                &[page_num.into()],
            )?
            .l()?;
        Ok(PdfPage(obj))
    }

    pub fn get_number_of_pages(&self, env: &mut JNIEnv<'a>) -> Result<i32> {
        let obj = env.call_method(self, "getNumberOfPages", "()I", &[])?.i()?;
        Ok(obj)
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
            &[r.into(), g.into(), b.into()],
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
        Ok(Self(env.new_object(
            "Lcom/itextpdf/kernel/geom/PageSize;",
            "(FF)V",
            &[width.into(), height.into()],
        )?))
    }

    pub fn get_width(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        env.call_method(self, "getWidth", "()F", &[])?.f()
    }

    pub fn get_height(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        env.call_method(self, "getHeight", "()F", &[])?.f()
    }
}

impl<'a> PdfFontFactory<'a> {
    pub fn create_from_standard_font(
        standard_font: StandardFont,
        env: &mut JNIEnv<'a>,
    ) -> Result<PdfFont<'a>> {
        let font = standard_font.get_java_value(env)?;
        let object = env
            .call_static_method(
                "com/itextpdf/kernel/font/PdfFontFactory",
                "createFont",
                "(Ljava/lang/String;)Lcom/itextpdf/kernel/font/PdfFont;",
                &[(&font).into()],
            )?
            .l()?;
        Ok(PdfFont(object))
    }

    pub fn create_from_program(program: FontProgram, env: &mut JNIEnv<'a>) -> Result<PdfFont<'a>> {
        let object = env
            .call_static_method(
                "com/itextpdf/kernel/font/PdfFontFactory",
                "createFont",
                "(Lcom/itextpdf/io/font/FontProgram;)Lcom/itextpdf/kernel/font/PdfFont;",
                &[program.as_ref().into()],
            )?
            .l()?;
        Ok(PdfFont(object))
    }

    pub fn create_from_program_with_encoding(
        program: FontProgram,
        encoding: PdfEncodings,
        env: &mut JNIEnv<'a>,
    ) -> Result<PdfFont<'a>> {
        let encoding = encoding.get_java_value(env)?;
        let object = env.call_static_method(
            "com/itextpdf/kernel/font/PdfFontFactory",
            "createFont",
            "(Lcom/itextpdf/io/font/FontProgram;Ljava/lang/String;)Lcom/itextpdf/kernel/font/PdfFont;",
            &[program.as_ref().into(), (&encoding).into()]
        )?.l()?;
        Ok(PdfFont(object))
    }

    pub fn create_from_program_with_encoding_embedded(
        program: FontProgram,
        encoding: PdfEncodings,
        env: &mut JNIEnv<'a>,
    ) -> Result<PdfFont<'a>> {
        let encoding = encoding.get_java_value(env)?;
        let object = env.call_static_method(
            "com/itextpdf/kernel/font/PdfFontFactory",
            "createFont",
            "(Lcom/itextpdf/io/font/FontProgram;Ljava/lang/String;Z)Lcom/itextpdf/kernel/font/PdfFont;",
            &[program.as_ref().into(), (&encoding).into(), JValueGen::Bool(true as jboolean)]
        )?.l()?;
        Ok(PdfFont(object))
    }
}

impl<'a> PdfCanvas<'a> {
    pub fn new_with_page(page: &PdfPage<'a>, env: &mut JNIEnv<'a>) -> Result<PdfCanvas<'a>> {
        let obj = env.new_object(
            "com/itextpdf/kernel/pdf/canvas/PdfCanvas",
            "(Lcom/itextpdf/kernel/pdf/PdfPage;)V",
            &[(&page).into()],
        )?;
        Ok(Self(obj))
    }

    pub fn save_state(&self, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "saveState",
            "()Lcom/itextpdf/kernel/pdf/canvas/PdfCanvas;",
            &[],
        )?;
        Ok(self)
    }

    pub fn restore_state(&self, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "restoreState",
            "()Lcom/itextpdf/kernel/pdf/canvas/PdfCanvas;",
            &[],
        )?;
        Ok(self)
    }

    pub fn set_ext_g_state(
        &self,
        ext_g_state: &PdfExtGState<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        env.call_method(
            self,
            "setExtGState",
            "(Lcom/itextpdf/kernel/pdf/extgstate/PdfExtGState;)Lcom/itextpdf/kernel/pdf/canvas/PdfCanvas;",
            &[
                (&ext_g_state).into()
            ],
        )?;
        Ok(self)
    }

    pub fn add_image_with_transformation_matrix(
        &self,
        image_data: &ImageData<'a>,
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
        inline: bool,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        env.call_method(
            self,
            "addImageWithTransformationMatrix",
            "(Lcom/itextpdf/io/image/ImageData;FFFFFFZ)Lcom/itextpdf/kernel/pdf/xobject/PdfXObject;",
            &[
                image_data.into(),
                a.into(),
                b.into(),
                c.into(),
                d.into(),
                e.into(),
                f.into(),
                inline.into(),
            ]
        )?;
        Ok(self)
    }

    pub fn add_x_object_at_form(
        &self,
        x_object: &PdfFormXObject,
        x: f32,
        y: f32,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        env.call_method(
            self,
            "addXObjectAt",
            "(Lcom/itextpdf/kernel/pdf/xobject/PdfXObject;FF)Lcom/itextpdf/kernel/pdf/canvas/PdfCanvas;",
            &[
                (&x_object).into(),
                x.into(),
                y.into()
            ]
        )?;

        Ok(self)
    }
}

impl<'a> PdfPage<'a> {
    pub fn set_ignore_page_rotation_for_content(
        &self,
        ignore_page_rotation_for_content: bool,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        env.call_method(
            self,
            "setIgnorePageRotationForContent",
            "(Z)Lcom/itextpdf/kernel/pdf/PdfPage;",
            &[ignore_page_rotation_for_content.into()],
        )?;
        Ok(self)
    }

    pub fn get_page_size(&self, env: &mut JNIEnv<'a>) -> Result<Rectangle<'a>> {
        let obj = env
            .call_method(
                self,
                "getPageSize",
                "()Lcom/itextpdf/kernel/geom/Rectangle;",
                &[],
            )?
            .l()?;
        Ok(Rectangle(obj))
    }

    pub fn get_page_size_with_rotation(&self, env: &mut JNIEnv<'a>) -> Result<Rectangle<'a>> {
        let obj = env
            .call_method(
                self,
                "getPageSizeWithRotation",
                "()Lcom/itextpdf/kernel/geom/Rectangle;",
                &[],
            )?
            .l()?;
        Ok(Rectangle(obj))
    }
}

impl<'a> Rectangle<'a> {
    pub fn new_w_h(width: f32, height: f32, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/kernel/geom/Rectangle",
            "(FF)V",
            &[width.into(), height.into()],
        )?;
        Ok(Self(obj))
    }

    pub fn new_x_y_w_h(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        env: &mut JNIEnv<'a>,
    ) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/kernel/geom/Rectangle",
            "(FFFF)V",
            &[x.into(), y.into(), width.into(), height.into()],
        )?;
        Ok(Self(obj))
    }

    pub fn get_top(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        let obj = env.call_method(self, "getTop", "()F", &[])?.f()?;
        Ok(obj)
    }

    pub fn get_right(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        let obj = env.call_method(self, "getRight", "()F", &[])?.f()?;
        Ok(obj)
    }

    pub fn get_bottom(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        let obj = env.call_method(self, "getBottom", "()F", &[])?.f()?;
        Ok(obj)
    }

    pub fn get_left(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        let obj = env.call_method(self, "getLeft", "()F", &[])?.f()?;
        Ok(obj)
    }
}

impl<'a> PdfExtGState<'a> {
    pub fn new(env: &mut JNIEnv<'a>) -> Result<PdfExtGState<'a>> {
        let obj = env.new_object("com/itextpdf/kernel/pdf/extgstate/PdfExtGState", "()V", &[])?;
        Ok(PdfExtGState(obj))
    }

    pub fn set_fill_opacity(
        &self,
        env: &mut JNIEnv<'a>,
        filling_alpha_constant: f32,
    ) -> Result<&Self> {
        env.call_method(
            self,
            "setFillOpacity",
            "(F)Lcom/itextpdf/kernel/pdf/extgstate/PdfExtGState;",
            &[filling_alpha_constant.into()],
        )?;
        Ok(self)
    }
}

impl<'a> PdfFormXObject<'a> {}
