use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening th file: {:?}", error)
        }
    };
}
