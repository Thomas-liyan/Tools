extern crate rodio;
//https://www.rustwiki.org.cn/zh-CN/cargo/getting-started/index.html
//https://sdk.nnsdao.com/docs/rust-guide/rust-unit-test
use std::fs::File;
use std::io::BufReader;
// use rodio::Source;
use glob::glob;
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::io::{self};

fn get_music_source(f: String) -> Decoder<BufReader<File>> {
    print!("open File : {} \n", f);
    let file = BufReader::new(File::open(f).unwrap());
    // Decode that sound file into a source
    // return source
    return Decoder::new(file).unwrap();
}

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
}

fn add_music_list(sink: &Sink) {
    // 定义文件路径模式
    let pattern = "/Users/liyan/Music/*.mp3";
    let result = get_files_from_pattern(pattern);
    if let Some(file_names) = result.ok() {
        for name in file_names {
            let abs_name = "/Users/liyan/Music/".to_string() + &name;
            print!("add file : {} \n", abs_name);
            sink.append(get_music_source(abs_name));
        }
    }
}

fn play_sink(sink: &Sink) {
    add_music_list(&sink);
    sink.play();
}

fn main() {
    println!("Hello, world!");
    main_loop();
    println!("Playing audio..end");
}

fn main_loop() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    loop {
        println!("->: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if "exit" == input.trim() {
                    println!("exit");
                    sink.stop();
                    break;
                }
                if "play" == input.trim() {
                    play_sink(&sink);
                }
                if "pause" == input.trim() {
                    sink.pause();
                }
                if "next" == input.trim() {
                    sink.skip_one();
                }
                if "len" == input.trim() {
                    print!("sink list len is : {} \n", sink.len());
                }
                if "resume" == input.trim() {
                    sink.play();
                } else {
                    println!("你输入的是：{}", input);
                    if sink.empty() {
                        add_music_list(&sink);
                        sink.play();
                    }
                }
            }
            Err(error) => {
                println!("无法读取输入：{}", error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rodio::source::Source;
    use rodio::{Decoder, OutputStream};
    use std::fs::File;
    use std::io::BufReader;
    use std::io::{self, Write};

    #[test]
    fn internal() {
        assert_eq!(4, 2 + 2);
        print!("tests end------------");
    }

    #[test]
    fn show_files() {
        print!("show_files end------------");
    }

    #[test]
    fn test_main() {
        loop {
            println!("->: ");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if "exit" == input.trim() {
                        println!("exit");
                        break;
                    }
                }
                Err(error) => {
                    println!("无法读取输入：{}", error);
                }
            }
            println!("你输入的是：{}", input);
            io::stdout().flush().unwrap();
        }
    }

    #[test]
    fn test_play_sound() {
        // Get an output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open("/Users/liyan/Music/output.mp3").unwrap());
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples());
        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(std::time::Duration::from_secs(365));
    }
}
