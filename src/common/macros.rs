#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        std::collections::HashMap::from([$(($k, $v),)*])
    }};
}
