use crate::itext::kernel::{Color, PdfDocument, PdfFormXObject};
use crate::java_object;
use jni::JNIEnv;

java_object!(BarcodeEAN);

pub enum BarcodeType {
    Ean13,
    Ean8,
    Supp2,
    Supp5,
    Upca,
    Upce,
}

impl<'a> BarcodeEAN<'a> {
    fn barcode_type_value(
        barcode_type: &BarcodeType,
        env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<i32> {
        let fname = match barcode_type {
            BarcodeType::Ean13 => "EAN13",
            BarcodeType::Ean8 => "EAN8",
            BarcodeType::Supp2 => "SUPP2",
            BarcodeType::Supp5 => "SUPP5",
            BarcodeType::Upca => "UPCA",
            BarcodeType::Upce => "UPCE",
        };

        let field = env
            .get_static_field("com/itextpdf/barcodes/BarcodeEAN", fname, "I")?
            .i()?;

        Ok(field)
    }

    pub fn new(pdf_document: &PdfDocument<'a>, env: &mut JNIEnv<'a>) -> jni::errors::Result<Self> {
        let obj = env.new_object(
            "com/itextpdf/barcodes/BarcodeEAN",
            "(Lcom/itextpdf/kernel/pdf/PdfDocument;)V",
            &[(&pdf_document).into()],
        )?;

        Ok(Self(obj))
    }

    pub fn set_code_type(
        &self,
        barcode_type: &BarcodeType,
        env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<&Self> {
        let ty = Self::barcode_type_value(barcode_type, env)?.into();
        env.call_method(self, "setCodeType", "(I)V", &[ty])?;
        Ok(self)
    }

    pub fn set_code(&self, code: &str, env: &mut JNIEnv<'a>) -> jni::errors::Result<&Self> {
        env.call_method(
            self,
            "setCode",
            "(Ljava/lang/String;)V",
            &[(&env.new_string(code)?).into()],
        )?;

        Ok(self)
    }

    pub fn create_form_x_object(
        &self,
        color_bar: &Color<'a>,
        color_text: &Color<'a>,
        pdf_document: &PdfDocument<'a>,
        env: &mut JNIEnv<'a>,
    ) -> jni::errors::Result<PdfFormXObject<'a>> {
        let obj  = env.call_method(
            self,
            "createFormXObject",
            "(Lcom/itextpdf/kernel/colors/Color;Lcom/itextpdf/kernel/colors/Color;Lcom/itextpdf/kernel/pdf/PdfDocument;)Lcom/itextpdf/kernel/pdf/xobject/PdfFormXObject;",
            &[
                (&color_bar).into(),
                (&color_text).into(),
                (&pdf_document).into(),
            ]
        )?.l()?;

        Ok(PdfFormXObject(obj))
    }
}
