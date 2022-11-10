use super::Ini;
use super::Value;

#[test]
fn check_simple_init() {
    let i = Ini::from("[Section]");
    assert!(i.is_ok(),"Fail to initialize a simple ini object");    
}


#[test]
fn check_bool_value() {
    let i = Ini::from("[Section]\ntrue_value = true\nfalse_value = false");
    assert!(i.is_ok(),"Fail to initialize a simple ini object");
    let ini = i.unwrap();
    let sect = ini.get_section("Section");
    assert!(sect.is_some(),"Unable to found section [Section]");
    let sect = sect.unwrap();    
    assert_eq!(sect.get("true_value"),Some(&Value::Bool(true)));
    assert_eq!(sect.get("false_value"),Some(&Value::Bool(false)));
    assert_eq!(sect.get("sone_value"),None);
}