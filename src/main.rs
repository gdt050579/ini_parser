mod ini;

fn main() {
    let i = ini::Ini::from(
        "[section] ; my section
            compute = 'some value'
            value = 100

            ; a comment
    ",
    );
    if i.is_err() {
        println!("{}",i.err().unwrap());
        return;
    }
    let ini = i.ok().unwrap();
    let s = ini.get_section("section").unwrap();
}
