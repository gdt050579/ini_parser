#[allow(unused_imports)]
use super::Ini;
#[allow(unused_imports)]
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

#[test]
fn check_int_value() {
    let i = Ini::from("[Section]\nv1 = 100\nv2 = -100\nv3 = 0\nv4 = 0xFF\nv5=12345678\nv6=0x1122334455667788");
    assert!(i.is_ok(),"Fail to initialize a simple ini object");
    let ini = i.unwrap();
    let sect = ini.get_section("Section");
    assert!(sect.is_some(),"Unable to found section [Section]");
    let sect = sect.unwrap();    
    assert_eq!(sect.get("v1"),Some(&Value::UInt64(100)));
    assert_eq!(sect.get("v2"),Some(&Value::Int64(-100)));
    assert_eq!(sect.get("v3"),Some(&Value::UInt64(0)));
    assert_eq!(sect.get("v4"),Some(&Value::UInt64(0xFF)));
    assert_eq!(sect.get("v5"),Some(&Value::UInt64(12345678)));
    assert_eq!(sect.get("v6"),Some(&Value::UInt64(0x1122334455667788)));
    assert_eq!(sect.get("sone_value"),None);
}

#[test]
fn check_incomplete_section_name() {
    let i = Ini::from("[incomplete_section\nv1 = 100");
    assert!(i.is_err(),"This code should have not been validated !");
    let err = i.err().unwrap();
    assert!(err.get_line_number()==1,"Error should have happen at line 1");
    assert!(err.get_error_message().starts_with("Expecting a ']' character to finish the section !\n[incomplete_section\n ^^^^^^^^^^^^^^^^^^"));
}