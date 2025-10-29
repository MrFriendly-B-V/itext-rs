# itext-rs
Rust bindings to the Java library iText-PDF version 9.2.

## Purpose
Generating PDFs in Rust is at the early stages. Java's PDF world is pretty far
along. This library allows a Rust program to create PDFs using iText PDF.

## Compiling
If the `bundled` feature is enabled:  
- The JVM should be installed.
- The environmental variable `JAVA_HOME` should be set.

## Usage
If the `bundled` feature is enabled:  
When using JNI's invocation API, the JAR file embedded in this crate
should be added to the classpath:
1. Save the jarfile (const `bundle::DEPENDENCIES`) to disk
2. Add the option `-Djava.class.path=<PATH TO JARFILE>` to the JVM's start parameters.

If the `bundled` feature is not enabled, you are responsible for providing the itext-pdf libraries to the JVM.

## License

MIT or Apache-2.0, at your option.