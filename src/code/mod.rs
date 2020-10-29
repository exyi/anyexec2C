pub mod c_code;
pub mod csharp_code;

pub trait CodeTemplate {
    fn render(executable_b64: String, payload_b64: Vec<(String, String)>, comment_files: Vec<(String, Vec<String>)>) -> String;
}
