extern crate rodio;
//https://www.rustwiki.org.cn/zh-CN/cargo/getting-started/index.html
//https://sdk.nnsdao.com/docs/rust-guide/rust-unit-test
//https://practice-zh.course.rs/flow-control.html
use std::fs::File;
use std::io::BufReader;
// use rodio::Source;
use glob::glob;
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::io::{self}; // 导入Rng trait，它提供了生成随机数的方法

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
    const ABS_PATH: &str = "/Users/liyan/Music/";
    // 定义文件路径模式
    let pattern = String::from(ABS_PATH) + "*.mp3";
    let result = get_files_from_pattern(&pattern);
    if let Some(file_names) = result.ok() {
        for name in file_names {
            let abs_name = ABS_PATH.to_string() + &name;
            print!("add file : {} \n", abs_name);
            sink.append(Decoder::new(BufReader::new(File::open(abs_name).unwrap())).unwrap());
        }
    }
}

fn play_random(sink: &Sink) {
    let total = sink.len();
    // 生成一个随机数
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=total);
    println!("Random number: {}", random_number);
    for _i in 0..random_number {
        sink.skip_one();
    }
    check_empty(sink);
}

fn check_empty(sink: &Sink) {
    if sink.empty() {
        add_music_list(&sink);
        sink.play();
    }
}

fn play_sink(sink: &Sink) {
    add_music_list(&sink);
    sink.play();
}

fn main() {
    println!("Enjoy it!");
    main_loop();
    println!("Playing audio...end");
}

fn main_loop() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    loop {
        println!("->: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "exit" => {
                    println!("exit");
                    sink.stop();
                    break;
                }
                "random" => {
                    play_random(&sink);
                }
                "play" => {
                    play_sink(&sink);
                }
                "pause" => {
                    sink.pause();
                }
                "next" => {
                    sink.skip_one();
                }
                "len" => {
                    print!("sink list len is : {} \n", sink.len());
                }
                "resume" => {
                    sink.play();
                }
                _ => {
                    println!("你输入的是：{}", input);
                    check_empty(&sink);
                }
            },
            Err(error) => {
                println!("无法读取输入：{}", error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use rodio::source::Source;
    use rodio::{Decoder, OutputStream};
    use std::fs::File;
    use std::io::BufReader;
    use std::io::{self, Write}; // 导入Rng trait，它提供了生成随机数的方法

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

    #[test]
    fn test_count_random() {
        0.89.abs();
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();
        // 生成1到100之间的随机整数
        let random_number = rng.gen_range(1..=10);
        print!("random number is : {} \n", random_number);
        for i in 1..=random_number {
            print!("i number is : {} \n", i)
        }
    }
}
