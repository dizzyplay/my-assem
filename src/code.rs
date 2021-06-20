fn dest(symbol: String) -> String {
    let mut code = 0;
    if symbol.contains("M") {
        code += 1;
    }
    if symbol.contains("D") {
        code += 2;
    }
    if symbol.contains("A") {
        code += 4;
    }
    format!("{:0>3b}",code)
    //    0 fill > align 3 width
    // https://stackoverflow.com/questions/50458144/what-is-the-easiest-way-to-pad-a-string-with-0-to-the-left
}

#[test]
fn dest_test(){
    assert_eq!(dest(format!("")), "000");
    assert_eq!(dest(format!("M")), "001");
    assert_eq!(dest(format!("D")), "010");
    assert_eq!(dest(format!("MD")), "011");
    assert_eq!(dest(format!("A")), "100");
    assert_eq!(dest(format!("AM")), "101");
    assert_eq!(dest(format!("AD")), "110");
    assert_eq!(dest(format!("AMD")), "111");
}