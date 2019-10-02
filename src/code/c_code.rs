use ::code::CodeTemplate;
use askama::Template;

#[derive(Template)]
#[template(path = "c_code.c", escape = "none")]
pub struct CCodeTemplate {
    comment_files: Vec<(String, Vec<String>)>,
    executable: String,
    assets: Vec<(String, String)>,
}

impl CodeTemplate for CCodeTemplate {
    fn render(executable_b64: String, payload_b64: Vec<(String, String)>, comment_files: Vec<(String, Vec<String>)>) -> String {
        let template = CCodeTemplate {executable: executable_b64,  assets: payload_b64 , comment_files};
        template.render().unwrap()
    }
}



#[derive(Template)]
#[template(path = "c_code_with_checks.c", escape = "none")]
pub struct CCodeWithChecksTemplate {
    comment_files: Vec<(String, Vec<String>)>,
    executable: String,
    assets: Vec<(String, String)>,
}

impl CodeTemplate for CCodeWithChecksTemplate {
    fn render(executable_b64: String, payload_b64: Vec<(String, String)>, comment_files: Vec<(String, Vec<String>)>) -> String {
        let template = CCodeTemplate {executable: executable_b64,  assets: payload_b64 , comment_files};
        template.render().unwrap()
    }
}
