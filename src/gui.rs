#[macro_use]
mod join_text;

const INDEX: &str = include_str!("../theme/index.html");

const JS_INI: &str = include_str!("../theme/js/initial_settings.js");
const JS_KNOBS: &str = include_str!("../theme/js/input_knobs.js");
const JS_ROTATE: &str = include_str!("../theme/js/rotate_knob.js");
const JS_BIG: &str = include_str!("../theme/js/big-ear.js");

const CSS_MAIN: &str = include_str!("../theme/css/main.css");

const IMAGE_EAR: &str = include_str!("../theme/images/ear");
const IMAGE_BACKGROUND: &str = include_str!("../theme/images/background");
const IMAGE_KNOB_BACKGROUND: &str = include_str!("../theme/images/knob_background");

pub fn generate_html() -> String {
    let js_tags: String = join_text!(JS_KNOBS, JS_ROTATE, JS_BIG, JS_INI);
    let css_tags: String = join_text!(CSS_MAIN);

    let before: Vec<&str> = vec!["{{ js }}", "{{ css }}", "{{ image_ear }}", "{{ image_background }}", "{{ image_knob_background }}"];
    let after:Vec<&str> = vec![&*js_tags, &*css_tags, IMAGE_EAR, IMAGE_BACKGROUND, IMAGE_KNOB_BACKGROUND];

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