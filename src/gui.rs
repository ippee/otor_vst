#[macro_use]
mod join_text;

const INDEX: &str = include_str!("../theme/index.html");

const JS_INVOKE: &str = include_str!("../theme/js/invoke_external_function.js");
const CSS_MAIN: &str = include_str!("../theme/css/main.css");

const IMAGE_EAR: &str = include_str!("../theme/images/ear");

pub fn generate_html() -> String {
    let js_tags: String = join_text!(JS_INVOKE);
    let css_tags: String = join_text!(CSS_MAIN);

    let before: Vec<&str> = vec!["{{ js }}", "{{ css }}", "{{ image_ear }}"];
    let after:Vec<&str> = vec![&*js_tags, &*css_tags, IMAGE_EAR];

    let html = replace_target(INDEX, before, after);

    html
}

fn replace_target(src: &str, before: Vec<&str>, after: Vec<&str>) -> String {
    let end = before.len();
    let mut result = "".to_string();
    let mut processing = src;

    for i in 0..end {
        result = processing.replace(before[i], after[i]);
        processing = &*result;
    }

    result
}