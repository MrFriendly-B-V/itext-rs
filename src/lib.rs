//! # iText-rs
//! Rust bindings to the iText 7.1.18 library.
//!
//! ## Bindings
//!
//! This library provides (partial) bindings for the following iText classes:
//! - com.itextpdf.io
//!     - ImageData
//! - com.itextpdf.kernel
//!     - PdfDocument
//!     - PdfWriter
//!     - SolidLine
//!     - PageSize
//!     - ColorConstant
//! - com.itextpdf.layout
//!     - Document
//!     - Table
//!     - Cell
//!     - Paragraph
//!     - LineSeparator
//!     - Image
//!
//! To support the iText-PDF types, there are also some other bindings included:
//! - java.io.ByteArrayOutputStream
//! - java.io.ByteArrayInputStream
//! - java.awt.image.BufferedImage
//! - javax.imageio.ImageIO
//! - javax.imageio.stream.ImageInputStream
//!
//! PR's to add missing bindings or to add to incomplete bindings are more than welcome.
//!
//! ## The library
//! To function, this crate needs a JVM, obviously. Furthermore it needs the iText kernel, layout and io libraries.
//! These are included in this crate via the `bundled` feature. A JAR file containing all dependencies is then included
//! at `itext::bundle::DEPENDENCIES`;
//!
//! If you're using the invocation API of the JVM to start it from code, add the option `-Djava.class.path=<PATH TO JARFILE>`
//! to the JVM's start options. Of course, the dependencies jarfile must be saved to disk for this to work.
//!
//! ## License
//! This library is provided under the MIT or Apache 2.0 license, at your option.

pub mod itext;
pub mod java;
pub mod javax;

mod java_object;

#[cfg(feature = "bundled")]
pub mod bundle {
    /// A JAR library containing iText-PDF and all of its dependencies.
    ///
    /// To use it, save the bytes to a file. Using JNI's invocation API you can then load it
    /// on the classpath using the option `-Djava.class.path=<path to saved jar>`.
    pub const DEPENDENCIES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dependencies.jar"));
}