use ::code::CodeTemplate;
use askama::Template;

#[derive(Template)]
#[template(path = "csharp_code.cs", escape = "none")]
pub struct CSharpCodeTemplate {
    comment_files: Vec<(String, Vec<String>)>,
    executable: String,
    assets: Vec<(String, String)>,
}

impl CodeTemplate for CSharpCodeTemplate {
    fn render(executable_b64: String, payload_b64: Vec<(String, String)>, comment_files: Vec<(String, Vec<String>)>) -> String {
        let template = CSharpCodeTemplate {executable: executable_b64,  assets: payload_b64, comment_files};
        template.render().unwrap()
    }
}
