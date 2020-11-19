use std::fs::read_to_string;
use std;

pub fn load_html() -> String {
    let content = read_to_string("./theme/index.html");

    let html = match content {
        Ok(v) => v,
        Err(_e) => "".to_string(),
    };

    let image_rel_path = std::path::Path::new("theme/images/ear.png");
    let pwd = std::env::current_dir().unwrap();
    let file_abs_path = pwd.join(image_rel_path);
    let file_abs_path = file_abs_path.into_os_string().into_string().unwrap();
    let file_abs_path = format!("\"{}\"", file_abs_path);

    let html_str: &str = &*html;

    let html = html_str.replace("{{ image_ear }}", &*file_abs_path);
    html
}