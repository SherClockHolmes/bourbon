pub mod prelude;

pub trait Renderable {
    type Values;

    fn new(values: Self::Values) -> Self;
    fn render(&self) -> Result<String, String>;
}

mod string_values {
    use super::Renderable;

    pub trait ToStringValue {
        fn to_string_value(&self) -> String;
    }

    impl ToStringValue for String {
        fn to_string_value(&self) -> String {
            self.clone()
        }
    }

    macro_rules! impl_to_string_value {
        ($($type:ty),* $(,)? ) => {
            $(
                impl ToStringValue for $type {
                    fn to_string_value(&self) -> String {
                        self.to_string()
                    }
                }
            )*
        };
    }

    impl_to_string_value! {
        i32,
        u32,
        i64,
        u64,
    }

    impl<R: Renderable> ToStringValue for R {
        fn to_string_value(&self) -> String {
            self.render().unwrap_or_else(|_| "Render failed".to_string())
        }
    }
}

