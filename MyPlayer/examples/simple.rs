use glob::glob;
use std::error::Error;

fn get_files_from_pattern(pattern: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let paths = glob(pattern)?;
    let mut file_names: Vec<String> = Vec::new();
    for path in paths {
        match path {
            Ok(path) => {
                file_names.push(path.file_name().unwrap().to_string_lossy().into_owned());
            }
            Err(e) => {}
        }
    }
    return Ok(file_names);
} //get_files_from_pattern

fn main() -> Result<(), Box<dyn Error>> {
    // 定义文件路径模式
    let pattern = "/Users/liyan/Music/*.mp3";

    let result = get_files_from_pattern(pattern);
   
    if let Some(file_names) = result.ok() {
        // 打印文件名
        for name in file_names {
            println!("/Users/liyan/Music/{}", name);
        }
    }

    Ok(())
}
