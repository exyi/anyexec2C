use ::code::CodeTemplate;
use askama::Template;

#[derive(Template)]
#[template(path = "python.py", escape = "none")]
pub struct PythonCodeTemplate {
    comment_files: Vec<(String, Vec<String>)>,
    executable: String,
    assets: Vec<(String, String)>,
}

impl CodeTemplate for PythonCodeTemplate {
    fn render(executable_b64: String, payload_b64: Vec<(String, String)>, comment_files: Vec<(String, Vec<String>)>) -> String {
        let template = PythonCodeTemplate {executable: executable_b64,  assets: payload_b64, comment_files};
        template.render().unwrap()
    }
}
