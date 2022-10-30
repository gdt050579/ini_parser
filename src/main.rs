mod ini;

fn main() {
    let i = ini::Ini::from(
        "
    default_value = 100

    [section] ; my section
    compute = 'some value'
    value = 100

    ; a comment
    [values]
    int = 10
    hex = 0xFFFF
    negative = -123
    float = 123.5
    bool = true
    string = 'a string value'
    ",
    );
    if i.is_err() {
        println!("{}", i.err().unwrap());
        return;
    }
    let ini = i.ok().unwrap();
    let s = ini.get_section("section").unwrap();
    println!("Name = {} with {} keys",s.get_name(),s.get_key_count());
    println!("Section: {}", ini.get_sections_count(true));
    println!("Has default section: {}",ini.has_default_section());
    for sect in &ini {
        println!("Section: {}",sect.get_name());
    }
    println!("text = {}",ini["values"].get_name());
    
}
