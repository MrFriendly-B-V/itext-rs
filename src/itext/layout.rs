use jni::errors::Result;
use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::jsize;
use crate::itext::io::ImageData;
use crate::itext::kernel::{ColorConstant, PdfDocument, SolidLine};
use crate::java_object;

java_object!(Document);
java_object!(Table);
java_object!(Cell);
java_object!(Paragraph);
java_object!(LineSeparator);
java_object!(Image);

pub trait ElementPropertyContainer<'a> where Self: AsRef<JObject<'a>> {
    fn set_border(&self, border: Border, env: &mut JNIEnv<'a>, ) -> Result<()> {
        let border_j = border.get_java_constant(env)?;
        env.call_method(self.as_ref(), "setBorder", "(Lcom/itextpdf/layout/borders/Border;)Lcom/itextpdf/layout/IPropertyContainer;", &[(&border_j).into()])?;

        Ok(())
    }

    fn set_horizontal_alignment(&self, alignment: HorizontalAlignment, env: &mut JNIEnv<'a>) -> Result<()> {
        let halign_j = alignment.get_java_value(env)?;
        env.call_method(self.as_ref(), "setHorizontalAlignment", "(Lcom/itextpdf/layout/property/HorizontalAlignment;)Lcom/itextpdf/layout/IPropertyContainer;", &[(&halign_j).into()])?;
        Ok(())
    }

    fn set_text_alignment(&self, alignment: TextAlignment, env: &mut JNIEnv<'a>) -> Result<()> {
        let talign_j = alignment.get_java_value(env)?;
        env.call_method(self.as_ref(), "setTextAlignment", "(Lcom/itextpdf/layout/property/TextAlignment;)Lcom/itextpdf/layout/IPropertyContainer;", &[(&talign_j).into()])?;
        Ok(())
    }

    fn set_bold(&self, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self.as_ref(), "setBold", "()Lcom/itextpdf/layout/IPropertyContainer;", &[])?;
        Ok(())
    }
}

pub trait BlockElement<'a> where Self: AsRef<JObject<'a>> {
    fn set_width(&self, width: f32, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self.as_ref(), "setWidth", "(F)Lcom/itextpdf/layout/element/IElement;", &[width.into()])?;
        Ok(())
    }

    fn set_height(&self, height: f32, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self.as_ref(), "setHeight", "(F)Lcom/itextpdf/layout/element/IElement;", &[height.into()])?;
        Ok(())
    }

    fn set_margin_bottom(&self, margin: f32, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self.as_ref(), "setMarginBottom", "(F)Lcom/itextpdf/layout/element/IElement;", &[margin.into()])?;
        Ok(())
    }

    fn set_margin_top(&self, margin: f32, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self.as_ref(), "setMarginTop", "(F)Lcom/itextpdf/layout/element/IElement;", &[margin.into()])?;
        Ok(())
    }
}

pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

pub enum Border {
    Solid {
        width: f32,
        color: ColorConstant,
    },
    NoBorder,
}

pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justified,
    JustifiedAll,
}

impl<'a> Document<'a> {
    pub fn new(pdf_document: &PdfDocument<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object("com/itextpdf/layout/Document", "(Lcom/itextpdf/kernel/pdf/PdfDocument;)V", &[(&pdf_document).into()])?;
        Ok(Self(obj))
    }

    pub fn set_margins(&self, top: f32, right: f32, bottom: f32, left: f32, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "setMargins", "(FFFF)V", &[top.into(), right.into(), bottom.into(), left.into()])?;
        Ok(())
    }

    pub fn add_table(&self, table: Table<'a>, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "add", "(Lcom/itextpdf/layout/element/IBlockElement;)Lcom/itextpdf/layout/Document;", &[(&table).into()])?;
        Ok(())
    }

    pub fn add_line_seperator(&self, line_seperator: LineSeparator<'a>, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "add", "(Lcom/itextpdf/layout/element/IBlockElement;)Lcom/itextpdf/layout/Document;", &[(&line_seperator).into()])?;
        Ok(())
    }

    pub fn close(self, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "close", "()V", &[])?;
        Ok(())
    }

    pub fn get_left_margin(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        Ok(env.call_method(self, "getLeftMargin", "()F", &[])?.f()?)
    }

    pub fn get_right_margin(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        Ok(env.call_method(self, "getRightMargin", "()F", &[])?.f()?)
    }
}

impl<'a> ElementPropertyContainer<'a> for Table<'a> {}

impl<'a> Table<'a> {
    pub fn new(point_column_widths: &[f32], env: &mut JNIEnv<'a>) -> Result<Self> {
        let array = env.new_float_array(point_column_widths.len() as jsize)?;
        env.set_float_array_region(&array, 0, &point_column_widths)?;

        let obj = env.new_object("com/itextpdf/layout/element/Table", "([F)V", &[(&array).into()])?;
        Ok(Self(obj))
    }

    pub fn start_new_row(&self, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "startNewRow", "()Lcom/itextpdf/layout/element/Table;", &[])?;
        Ok(())
    }

    pub fn add_cell(&self, cell: Cell<'a>, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "addCell", "(Lcom/itextpdf/layout/element/Cell;)Lcom/itextpdf/layout/element/Table;", &[(&cell).into()])?;
        Ok(())
    }

    pub fn use_all_available_width(&self, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "useAllAvailableWidth", "()Lcom/itextpdf/layout/element/Table;", &[])?;
        Ok(())
    }
}

impl HorizontalAlignment {
    fn get_java_value<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        let field_name = match self {
            Self::Left => "LEFT",
            Self::Center => "CENTER",
            Self::Right => "RIGHT",
        };

        let obj = env.get_static_field("com/itextpdf/layout/property/HorizontalAlignment", field_name, "Lcom/itextpdf/layout/property/HorizontalAlignment;")?.l()?;
        Ok(obj)
    }
}

impl Border {
    fn get_java_constant<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        Ok(match self {
            Self::Solid { width, color} => {
                let color_j = color.get_java_value(env)?;
                env.new_object("com/itextpdf/layout/borders/SolidBorder", "(Lcom/itextpdf/kernel/colors/Color;F)V", &[(&color_j).into(), (*width).into()])?.into()
            },
            Self::NoBorder => JObject::null().into(),
        })
    }
}

impl TextAlignment {
    fn get_java_value<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        let field_name = match self {
            Self::Left => "LEFT",
            Self::Center => "CENTER",
            Self::Right => "RIGHT",
            Self::Justified => "JUSTIFIED",
            Self::JustifiedAll => "JUSTIFIED_ALL",
        };

        let obj = env.get_static_field("com/itextpdf/layout/property/TextAlignment", field_name, "Lcom/itextpdf/layout/property/TextAlignment;")?.l()?;
        Ok(obj)
    }
}

impl<'a> ElementPropertyContainer<'a> for Cell<'a> {}
impl<'a> BlockElement<'a> for Cell<'a> {}

impl<'a> Cell<'a> {
    pub fn new(env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object("com/itextpdf/layout/element/Cell", "()V", &[])?;
        Ok(Self(obj))
    }

    pub fn add_paragraph(&self, paragraph: Paragraph<'a>, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "add", "(Lcom/itextpdf/layout/element/IBlockElement;)Lcom/itextpdf/layout/element/Cell;", &[(&paragraph).into()])?;
        Ok(())
    }

    pub fn add_image(&self, image: Image<'a>, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "add", "(Lcom/itextpdf/layout/element/IBlockElement;)Lcom/itextpdf/layout/element/Cell;", &[(&image).into()])?;
        Ok(())
    }
}

impl<'a> BlockElement<'a> for Paragraph<'a> {}
impl<'a> ElementPropertyContainer<'a> for Paragraph<'a> {}

impl<'a> Paragraph<'a> {
    pub fn new(env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object("com/itextpdf/layout/element/Paragraph", "()V", &[])?;
        Ok(Self(obj))
    }

    pub fn new_with_text(text: &str, env: &mut JNIEnv<'a>) -> Result<Self> {
        let string = env.new_string(text)?;
        let obj = env.new_object("com/itextpdf/layout/element/Paragraph", "(Ljava/lang/String;)V", &[(&string).into()])?;

        Ok(Self(obj))
    }
}

impl<'a> BlockElement<'a> for LineSeparator<'a> {}
impl<'a> ElementPropertyContainer<'a> for LineSeparator<'a> {}

impl<'a> LineSeparator<'a> {
    pub fn new_solid(line: SolidLine<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object("com/itextpdf/layout/element/LineSeparator", "(Lcom/itextpdf/kernel/pdf/canvas/draw/ILineDrawer;)V", &[(&line).into()])?;
        Ok(Self(obj))
    }
}

impl<'a> ElementPropertyContainer<'a> for Image<'a> {}

impl<'a> Image<'a> {
    pub fn new(image_data: ImageData<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object("com/itextpdf/layout/element/Image", "(Lcom/itextpdf/io/image/ImageData;)V", &[(&image_data).into()])?;
        Ok(Self(obj))
    }

    pub fn set_width(&self, width: f32, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self.as_ref(), "setWidth", "(F)Lcom/itextpdf/layout/element/Image;", &[width.into()])?;
        Ok(())
    }

    pub fn set_height(&self, height: f32, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self.as_ref(), "setHeight", "(F)Lcom/itextpdf/layout/element/Image;", &[height.into()])?;
        Ok(())
    }
}