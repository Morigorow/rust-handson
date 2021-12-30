use std::fs::read_to_string;

struct GrepArgs {
    pattern: String,
    path: String,
}

impl GrepArgs{
    //型に紐づく静的メソッド
    //※第一引数にselfをつけるとインスタンスメソッドになる
    fn new(path: String, pattern: String) -> GrepArgs {
        GrepArgs { path, pattern }
    }
}

fn run(state: GrepArgs) {
    match read_to_string(state.path) {
        Ok(content) => grep(content, state.pattern),
        Err(reason) => println!("{}", reason),
    }
}

fn grep(content: String, pattern: String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn main() {
    let pattern = std::env::args().nth(1);
    let path = std::env::args().nth(2);

    match (pattern, path) {
        //スタティックメソッドは 構造体名::メソッド名 で呼び出し
        (Some(pattern), Some(path)) => run(GrepArgs::new(path, pattern)),
        _ => println!("pattern or path is not specified!"),
    }
}
