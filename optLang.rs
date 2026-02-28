use std::fs;

fn main() -> () {
    let mut content : String = fs::read_to_string("source.opt")
        .expect("Couldn't read source file");

    content.pop();

    let mut anchor : usize = 0;

    for (index, item) in content.chars().enumerate() {
        match item {
            ' ' => {
                println!("{}", &content[anchor..index]);
                anchor = index + 1;
            },
            _ => {},
        }
    }

    println!("{}", &content[anchor..]);
}
