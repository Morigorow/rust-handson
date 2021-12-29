use std::fs::read_to_string;

fn grep(content: String, pattern: String) {
    //.lines()関数で1行ずつイテレータとして処理
    for line in content.lines() {
        //containsは引数に&strを求めるため、as_str関数で&str型に変換
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

fn run(path: String, pattern: String) {
    match read_to_string(path) {
        Ok(content) => grep(content, pattern),
        Err(reason) => println!("{}", reason),
    }
}


fn main() {
    //第一引数：検索したい文字列
    let pattern = std::env::args().nth(1);
    let path = std::env::args().nth(2);
    
    match (pattern, path) {
        //Some、NoneはOption型のバリアント(列挙子)。値がない可能性を考慮するときに使用。
        (Some(pattern), Some(path)) => run(path, pattern),
        _ => println!("Pattern or path is not specified!"),
    }
}