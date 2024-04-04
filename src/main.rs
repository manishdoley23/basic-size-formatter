use std::env;

#[derive(Debug)]
struct Size {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}
impl Size {
    fn get_the_val(size_type: &str) -> usize {
        match size_type {
            "b" => 1,
            "kb" => 1_000,
            "mb" => 1_000_000,
            _ => 1_000_000_000,
        }
    }

    fn print(size: usize, size_type: &str) -> Self {
        Size {
            bytes: (size as f64 * (Self::get_the_val(size_type) as f64)).to_string(),
            kilobytes: (size as f64 * (Self::get_the_val(size_type) as f64 / 1_000.0)).to_string(),
            megabytes: (size as f64 * (Self::get_the_val(size_type) as f64 / 1_000_000.0))
                .to_string(),
            gigabytes: (size as f64 * (Self::get_the_val(size_type) as f64 / 1_000_000_000.0))
                .to_string(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: Vec<&str> = args[1].split(" ").collect();
    let size: usize = input[0].parse().unwrap();
    let size_type = input[1];

    let result = Size::print(size, size_type);
    println!("{:?}", result);
}
