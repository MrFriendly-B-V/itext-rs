#[macro_export]
macro_rules! java_object {
    ($name:ident) => {
        pub struct $name<'a>(pub(crate) ::jni::objects::JObject<'a>);

        impl<'a> AsRef<::jni::objects::JObject<'a>> for $name<'a> {
            fn as_ref(&self) -> &::jni::objects::JObject<'a> {
                &self.0
            }
        }
    }
}