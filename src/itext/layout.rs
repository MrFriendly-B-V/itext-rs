use crate::itext::io::ImageData;
use crate::itext::kernel::{
    Color, ColorConstant, PageSize, PdfCanvas, PdfDocument, PdfFont, PdfPage, Rectangle, SolidLine,
};
use crate::java_object;
use convert_case::{Case, Casing};
use jni::errors::Result;
use jni::objects::{JObject, JValueGen};
use jni::sys::jsize;
use jni::JNIEnv;
use strum_macros::Display;

java_object!(Document);
java_object!(Table);
java_object!(Cell);
java_object!(Paragraph);
java_object!(LineSeparator);
java_object!(Image);
java_object!(Canvas);

pub trait RootElement<'a>
where
    Self: AsRef<JObject<'a>> + ElementPropertyContainer<'a>,
{
    fn add<F: BlockElement<'a>>(&self, element: F, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "add",
            "(Lcom/itextpdf/layout/element/IBlockElement;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[element.as_ref().into()],
        )?;
        Ok(self)
    }

    fn add_image(&self, element: &Image<'a>, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "add",
            "(Lcom/itextpdf/layout/element/Image;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[element.as_ref().into()],
        )?;
        Ok(self)
    }
}

impl<'a, T: RootElement<'a>> RootElement<'a> for &T {}

pub trait ElementPropertyContainer<'a>
where
    Self: AsRef<JObject<'a>>,
{
    fn set_fixed_position(
        &self,
        left: f32,
        bottom: f32,
        width: f32,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setFixedPosition",
            "(FFF)Lcom/itextpdf/layout/IPropertyContainer;",
            &[left.into(), bottom.into(), width.into()],
        )?;
        Ok(self)
    }

    fn set_border(&self, border: Border, env: &mut JNIEnv<'a>) -> Result<&Self> {
        let border_j = border.get_java_constant(env)?;
        env.call_method(
            self.as_ref(),
            "setBorder",
            "(Lcom/itextpdf/layout/borders/Border;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[(&border_j).into()],
        )?;
        Ok(self)
    }

    fn set_border_bottom(&self, border: Border, env: &mut JNIEnv<'a>) -> Result<&Self> {
        let border_j = border.get_java_constant(env)?;
        env.call_method(
            self.as_ref(),
            "setBorderBottom",
            "(Lcom/itextpdf/layout/borders/Border;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[(&border_j).into()],
        )?;
        Ok(self)
    }

    fn set_border_top(&self, border: Border, env: &mut JNIEnv<'a>) -> Result<&Self> {
        let border_j = border.get_java_constant(env)?;
        env.call_method(
            self.as_ref(),
            "setBorderTop",
            "(Lcom/itextpdf/layout/borders/Border;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[(&border_j).into()],
        )?;
        Ok(self)
    }

    fn set_border_left(&self, border: Border, env: &mut JNIEnv<'a>) -> Result<&Self> {
        let border_j = border.get_java_constant(env)?;
        env.call_method(
            self.as_ref(),
            "setBorderLeft",
            "(Lcom/itextpdf/layout/borders/Border;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[(&border_j).into()],
        )?;
        Ok(self)
    }

    fn set_border_right(&self, border: Border, env: &mut JNIEnv<'a>) -> Result<&Self> {
        let border_j = border.get_java_constant(env)?;
        env.call_method(
            self.as_ref(),
            "setBorderRight",
            "(Lcom/itextpdf/layout/borders/Border;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[(&border_j).into()],
        )?;
        Ok(self)
    }

    fn set_horizontal_alignment(
        &self,
        alignment: HorizontalAlignment,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        let halign_j = alignment.get_java_value(env)?;
        env.call_method(self.as_ref(), "setHorizontalAlignment", "(Lcom/itextpdf/layout/property/HorizontalAlignment;)Lcom/itextpdf/layout/IPropertyContainer;", &[(&halign_j).into()])?;
        Ok(self)
    }

    fn set_text_alignment(&self, alignment: TextAlignment, env: &mut JNIEnv<'a>) -> Result<&Self> {
        let talign_j = alignment.get_java_value(env)?;
        env.call_method(self.as_ref(), "setTextAlignment", "(Lcom/itextpdf/layout/property/TextAlignment;)Lcom/itextpdf/layout/IPropertyContainer;", &[(&talign_j).into()])?;
        Ok(self)
    }

    fn set_bold(&self, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setBold",
            "()Lcom/itextpdf/layout/IPropertyContainer;",
            &[],
        )?;
        Ok(self)
    }

    fn set_italic(&self, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setItalic",
            "()Lcom/itextpdf/layout/IPropertyContainer;",
            &[],
        )?;
        Ok(self)
    }

    fn set_font_size(&self, font_size: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setFontSize",
            "(F)Lcom/itextpdf/layout/IPropertyContainer;",
            &[font_size.into()],
        )?;
        Ok(self)
    }

    fn set_font_color(&self, color: &Color<'a>, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setFontColor",
            "(Lcom/itextpdf/kernel/colors/Color;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[(&color).into()],
        )?;
        Ok(self)
    }

    fn set_font_color_with_opacity(
        &self,
        color: &Color<'a>,
        opacity: f32,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setFontColor",
            "(Lcom/itextpdf/kernel/colors/Color;F)Lcom/itextpdf/layout/IPropertyContainer;",
            &[(&color).into(), JValueGen::Float(opacity)],
        )?;
        Ok(self)
    }

    fn set_font(&self, font: &PdfFont<'a>, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setFont",
            "(Lcom/itextpdf/kernel/font/PdfFont;)Lcom/itextpdf/layout/IPropertyContainer;",
            &[(&font).into()],
        )?;

        Ok(self)
    }
}

impl<'a, T: ElementPropertyContainer<'a>> ElementPropertyContainer<'a> for &T {}

pub trait BlockElement<'a>
where
    Self: AsRef<JObject<'a>>,
{
    fn set_width(&self, width: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setWidth",
            "(F)Lcom/itextpdf/layout/element/IElement;",
            &[width.into()],
        )?;
        Ok(self)
    }

    fn set_height(&self, height: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setHeight",
            "(F)Lcom/itextpdf/layout/element/IElement;",
            &[height.into()],
        )?;
        Ok(self)
    }

    fn set_margin_bottom(&self, margin: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setMarginBottom",
            "(F)Lcom/itextpdf/layout/element/IElement;",
            &[margin.into()],
        )?;
        Ok(self)
    }

    fn set_margin_top(&self, margin: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setMarginTop",
            "(F)Lcom/itextpdf/layout/element/IElement;",
            &[margin.into()],
        )?;
        Ok(self)
    }

    fn set_margin_left(&self, margin: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setMarginLeft",
            "(F)Lcom/itextpdf/layout/element/IElement;",
            &[margin.into()],
        )?;
        Ok(self)
    }

    fn set_margin_right(&self, margin: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setMarginRight",
            "(F)Lcom/itextpdf/layout/element/IElement;",
            &[margin.into()],
        )?;
        Ok(self)
    }

    fn set_vertical_alignment(
        &self,
        alignment: VerticalAlignment,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        let valign_j = alignment.get_java_value(env)?;
        env.call_method(self.as_ref(), "setVerticalAlignment", "(Lcom/itextpdf/layout/property/VerticalAlignment;)Lcom/itextpdf/layout/element/IElement;", &[(&valign_j).into()])?;
        Ok(self)
    }
}

impl<'a, T: BlockElement<'a>> BlockElement<'a> for &T {}

pub trait Element<'a>
where
    Self: AsRef<JObject<'a>>,
{
}

impl<'a, T: Element<'a>> Element<'a> for &T {}

#[derive(Clone, Display)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Clone, Display)]
pub enum VerticalAlignment {
    Bottom,
    Middle,
    Top,
}

#[derive(Clone)]
pub enum Border {
    Solid { width: f32, color: ColorConstant },
    NoBorder,
}

#[derive(Clone)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justified,
    JustifiedAll,
}

impl<'a> ElementPropertyContainer<'a> for Document<'a> {}
impl<'a> RootElement<'a> for Document<'a> {}

impl<'a> Document<'a> {
    pub fn new(pdf_document: &PdfDocument<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/layout/Document",
            "(Lcom/itextpdf/kernel/pdf/PdfDocument;)V",
            &[(&pdf_document).into()],
        )?;
        Ok(Self(obj))
    }

    pub fn new_with_flush(
        pdf_document: &PdfDocument<'a>,
        page_size: &PageSize,
        immediate_flush: bool,
        env: &mut JNIEnv<'a>,
    ) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/layout/Document",
            "(Lcom/itextpdf/kernel/pdf/PdfDocument;Lcom/itextpdf/kernel/geom/PageSize;Z)V",
            &[
                pdf_document.into(),
                page_size.into(),
                immediate_flush.into(),
            ],
        )?;
        Ok(Self(obj))
    }

    pub fn set_margins(
        &self,
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        env.call_method(
            self,
            "setMargins",
            "(FFFF)V",
            &[top.into(), right.into(), bottom.into(), left.into()],
        )?;
        Ok(self)
    }

    pub fn close(self, env: &mut JNIEnv<'a>) -> Result<()> {
        env.call_method(self, "close", "()V", &[])?;
        Ok(())
    }

    pub fn get_left_margin(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        env.call_method(self, "getLeftMargin", "()F", &[])?.f()
    }

    pub fn get_right_margin(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        env.call_method(self, "getRightMargin", "()F", &[])?.f()
    }

    pub fn get_bottom_margin(&self, env: &mut JNIEnv<'a>) -> Result<f32> {
        env.call_method(self, "getBottomMargin", "()F", &[])?.f()
    }

    pub fn get_pdf_document(&self, env: &mut JNIEnv<'a>) -> Result<PdfDocument<'a>> {
        let object = env
            .call_method(
                self,
                "getPdfDocument",
                "()Lcom/itextpdf/kernel/pdf/PdfDocument;",
                &[],
            )?
            .l()?;
        Ok(PdfDocument(object))
    }
}

impl<'a> ElementPropertyContainer<'a> for Table<'a> {}
impl<'a> Element<'a> for Table<'a> {}
impl<'a> BlockElement<'a> for Table<'a> {}

impl<'a> Table<'a> {
    pub fn new(point_column_widths: &[f32], env: &mut JNIEnv<'a>) -> Result<Self> {
        let array = env.new_float_array(point_column_widths.len() as jsize)?;
        env.set_float_array_region(&array, 0, point_column_widths)?;

        let obj = env.new_object(
            "com/itextpdf/layout/element/Table",
            "([F)V",
            &[(&array).into()],
        )?;
        Ok(Self(obj))
    }

    pub fn start_new_row(&self, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "startNewRow",
            "()Lcom/itextpdf/layout/element/Table;",
            &[],
        )?;
        Ok(self)
    }

    pub fn add_cell(&self, cell: &Cell<'a>, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "addCell",
            "(Lcom/itextpdf/layout/element/Cell;)Lcom/itextpdf/layout/element/Table;",
            &[(&cell).into()],
        )?;
        Ok(self)
    }

    pub fn use_all_available_width(&self, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "useAllAvailableWidth",
            "()Lcom/itextpdf/layout/element/Table;",
            &[],
        )?;
        Ok(self)
    }

    pub fn set_fixed_layout(&self, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "setFixedLayout",
            "()Lcom/itextpdf/layout/element/Table;",
            &[],
        )?;

        Ok(self)
    }
}

impl HorizontalAlignment {
    fn get_java_value<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        let field_name = self.to_string().to_case(Case::ScreamingSnake);

        let obj = env
            .get_static_field(
                "com/itextpdf/layout/property/HorizontalAlignment",
                field_name,
                "Lcom/itextpdf/layout/property/HorizontalAlignment;",
            )?
            .l()?;
        Ok(obj)
    }
}

impl VerticalAlignment {
    fn get_java_value<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        let field_name = self.to_string().to_case(Case::ScreamingSnake);

        let obj = env
            .get_static_field(
                "com/itextpdf/layout/property/VerticalAlignment",
                field_name,
                "Lcom/itextpdf/layout/property/VerticalAlignment;",
            )?
            .l()?;
        Ok(obj)
    }
}

impl Border {
    fn get_java_constant<'a>(&self, env: &mut JNIEnv<'a>) -> Result<JObject<'a>> {
        Ok(match self {
            Self::Solid { width, color } => {
                let color_j = color.get_java_value(env)?;
                env.new_object(
                    "com/itextpdf/layout/borders/SolidBorder",
                    "(Lcom/itextpdf/kernel/colors/Color;F)V",
                    &[(&color_j).into(), (*width).into()],
                )?
            }
            Self::NoBorder => JObject::null(),
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

        let obj = env
            .get_static_field(
                "com/itextpdf/layout/property/TextAlignment",
                field_name,
                "Lcom/itextpdf/layout/property/TextAlignment;",
            )?
            .l()?;
        Ok(obj)
    }
}

impl<'a> ElementPropertyContainer<'a> for Cell<'a> {}
impl<'a> BlockElement<'a> for Cell<'a> {}
impl<'a> Element<'a> for Cell<'a> {}

impl<'a> Cell<'a> {
    pub fn new(env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object("com/itextpdf/layout/element/Cell", "()V", &[])?;
        Ok(Self(obj))
    }

    pub fn new_with_span(rowspan: i32, colspan: i32, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/layout/element/Cell",
            "(II)V",
            &[rowspan.into(), colspan.into()],
        )?;

        Ok(Self(obj))
    }

    pub fn add_image(&self, image: &Image<'a>, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self,
            "add",
            "(Lcom/itextpdf/layout/element/IBlockElement;)Lcom/itextpdf/layout/element/Cell;",
            &[(&image).into()],
        )?;
        Ok(self)
    }

    pub fn add<F: BlockElement<'a>>(
        &self,
        block_element: &F,
        env: &mut JNIEnv<'a>,
    ) -> Result<&Self> {
        env.call_method(
            self,
            "add",
            "(Lcom/itextpdf/layout/element/IBlockElement;)Lcom/itextpdf/layout/element/Cell;",
            &[block_element.as_ref().into()],
        )?;
        Ok(self)
    }
}

impl<'a> BlockElement<'a> for Paragraph<'a> {}
impl<'a> ElementPropertyContainer<'a> for Paragraph<'a> {}
impl<'a> Element<'a> for Paragraph<'a> {}

impl<'a> Paragraph<'a> {
    pub fn new(env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object("com/itextpdf/layout/element/Paragraph", "()V", &[])?;
        Ok(Self(obj))
    }

    pub fn new_with_text(text: &str, env: &mut JNIEnv<'a>) -> Result<Self> {
        let string = env.new_string(text)?;
        let obj = env.new_object(
            "com/itextpdf/layout/element/Paragraph",
            "(Ljava/lang/String;)V",
            &[(&string).into()],
        )?;

        Ok(Self(obj))
    }
}

impl<'a> BlockElement<'a> for LineSeparator<'a> {}
impl<'a> ElementPropertyContainer<'a> for LineSeparator<'a> {}
impl<'a> Element<'a> for LineSeparator<'a> {}

impl<'a> LineSeparator<'a> {
    pub fn new_solid(line: SolidLine<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/layout/element/LineSeparator",
            "(Lcom/itextpdf/kernel/pdf/canvas/draw/ILineDrawer;)V",
            &[(&line).into()],
        )?;
        Ok(Self(obj))
    }
}

impl<'a> ElementPropertyContainer<'a> for Image<'a> {}
impl<'a> Element<'a> for Image<'a> {}

impl<'a> Image<'a> {
    pub fn new(image_data: ImageData<'a>, env: &mut JNIEnv<'a>) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/layout/element/Image",
            "(Lcom/itextpdf/io/image/ImageData;)V",
            &[(&image_data).into()],
        )?;
        Ok(Self(obj))
    }

    pub fn set_width(&self, width: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setWidth",
            "(F)Lcom/itextpdf/layout/element/Image;",
            &[width.into()],
        )?;
        Ok(self)
    }

    pub fn set_height(&self, height: f32, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(
            self.as_ref(),
            "setHeight",
            "(F)Lcom/itextpdf/layout/element/Image;",
            &[height.into()],
        )?;
        Ok(self)
    }
}

impl<'a> ElementPropertyContainer<'a> for Canvas<'a> {}
impl<'a> RootElement<'a> for Canvas<'a> {}

impl<'a> Canvas<'a> {
    pub fn new_from_canvas(
        pdf_canvas: &PdfCanvas<'a>,
        root_area: &Rectangle<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/layout/Canvas",
            "(Lcom/itextpdf/kernel/canvas/PdfCanvas;Lcom/itextpdf/kernel/geom/Rectangle;)V",
            &[pdf_canvas.into(), root_area.into()],
        )?;
        Ok(Self(obj))
    }

    pub fn new_from_canvas_flush(
        pdf_canvas: &PdfCanvas<'a>,
        root_area: &Rectangle<'a>,
        immediate_flush: bool,
        env: &mut JNIEnv<'a>,
    ) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/layout/Canvas",
            "(Lcom/itextpdf/kernel/canvas/PdfCanvas;Lcom/itextpdf/kernel/geom/Rectangle;Z)V",
            &[pdf_canvas.into(), root_area.into(), immediate_flush.into()],
        )?;
        Ok(Self(obj))
    }

    pub fn new_from_page(
        pdf_page: &PdfPage<'a>,
        root_area: &Rectangle<'a>,
        env: &mut JNIEnv<'a>,
    ) -> Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/layout/Canvas",
            "(Lcom/itextpdf/kernel/pdf/PdfPage;Lcom/itextpdf/kernel/geom/Rectangle;)V",
            &[pdf_page.into(), root_area.into()],
        )?;
        Ok(Self(obj))
    }

    ///  Performs an entire recalculation of the element flow on the canvas, taking into account all its current child elements.
    pub fn relayout(&self, env: &mut JNIEnv<'a>) -> Result<&Self> {
        env.call_method(self, "relayout", "()V", &[])?;
        Ok(self)
    }
}
