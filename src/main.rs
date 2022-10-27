mod ini;

fn main() {
    let i = ini::Ini::from(
        "[section] ; my section
    value = 100
    compute = 'some value'
    ; a comment
    ",
    );
    if i.is_err() {
        println!("{:?}",i.err().unwrap());
    }
}
