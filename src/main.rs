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
        println!("{}", i.err().unwrap().get_error_message());
        return;
    }
    let mut ini = i.ok().unwrap();
    let s = ini.get_section("section").unwrap();
    
    println!("Name = {} with {} keys",s.get_name(),s.get_keys_count());
    println!("Section: {}", ini.get_sections_count(true));
    println!("Has default section: {}",ini.has_default_section());
    for sect in &ini {
        println!("Section: {}",sect.get_name());
        for value in sect {
            println!("  {} = {:?}",value.get_name(),value.get_value());
        }
        println!("");
    }
    println!("text = {}",ini["values"].get_name());
    ini["abc"].set("value", "123");
    ini["abc"].set("value2", true);
    println!("ini[abc][value] = {:?}",ini.get_section("abc").unwrap().get("value"));
    println!("ini[abc][value2] = {:?}",ini.get_value("abc", "value2"));
    for sect in &ini {
        println!("Section: {}",sect.get_name());
    } 
    
}
