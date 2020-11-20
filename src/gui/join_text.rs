#[macro_export]
macro_rules! join_text {
    ($($args:expr), *) => {{
        let res: String = "".to_string();

        $(
            let res = format!("{}{}", res, $args);
        )*
        
        res
    }};
}