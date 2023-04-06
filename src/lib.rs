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