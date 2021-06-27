pub fn dest(symbol: String) -> String {
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

pub fn comp(symbol:String) -> String {
    match symbol.as_str() {
        "0" => format!("0101010"),
        "1" => format!("0111111"),
        "-1" => format!("0111010"),
        "D" => format!("0001100"),
        "A" => format!("0110000"),
        "!D" => format!("0001101"),
        "!A" => format!("0110001"),
        "-D" => format!("0001111"),
        "-A" => format!("0110011"),
        "D+1" => format!("0011111"),
        "A+1" => format!("0110111"),
        "D-1" => format!("0001110"),
        "A-1" => format!("0110010"),
        "D+A" => format!("0000010"),
        "D-A" => format!("0010011"),
        "A-D" => format!("0000111"),
        "D&A" => format!("0000000"),
        "D|A" => format!("0010101"),
        "M" => format!("1110000"),
        "!M" => format!("1110001"),
        "-M" => format!("1110011"),
        "M+1" => format!("1110111"),
        "M-1" => format!("1110010"),
        "D+M" => format!("1000010"),
        "D-M" => format!("1010011"),
        "M-D" => format!("1000111"),
        "D&M" => format!("1000000"),
        "D|M" => format!("1010101"),
        _ => panic!("wrong symbol")
    }
}

pub fn jump(symbol:String)->String{
    match symbol.as_str(){
        "" => format!("000"),
        "M" => format!("001"),
        "D" => format!("010"),
        "DM" => format!("011"),
        "A" => format!("100"),
        "AM" => format!("101"),
        "AD" => format!("110"),
        "ADM" => format!("111"),
        _ => panic!("wrong symbol")
    }
}

#[test]
fn jump_test(){
    assert_eq!(jump(format!("")), "000");
    assert_eq!(jump(format!("M")), "001");
    assert_eq!(jump(format!("D")), "010");
    assert_eq!(jump(format!("DM")), "011");
    assert_eq!(jump(format!("A")), "100");
    assert_eq!(jump(format!("AM")), "101");
    assert_eq!(jump(format!("AD")), "110");
    assert_eq!(jump(format!("ADM")), "111");
}

#[test]
fn comp_test(){
    assert_eq!(comp(format!("0")),"0101010");
    assert_eq!(comp(format!("1")),"0111111");
    assert_eq!(comp(format!("-1")),"0111010");
    assert_eq!(comp(format!("D")),"0001100");
    assert_eq!(comp(format!("A")),"0110000");
    assert_eq!(comp(format!("!D")),"0001101");
    assert_eq!(comp(format!("!A")),"0110001");
    assert_eq!(comp(format!("-D")),"0001111");
    assert_eq!(comp(format!("-A")),"0110011");
    assert_eq!(comp(format!("D+1")),"0011111");
    assert_eq!(comp(format!("A+1")),"0110111");
    assert_eq!(comp(format!("D-1")),"0001110");
    assert_eq!(comp(format!("A-1")),"0110010");
    assert_eq!(comp(format!("D+A")),"0000010");
    assert_eq!(comp(format!("A+1")),"0110111");
    assert_eq!(comp(format!("D-1")),"0001110");
    assert_eq!(comp(format!("A-1")),"0110010");
    assert_eq!(comp(format!("D+A")),"0000010");
    assert_eq!(comp(format!("D-A")),"0010011");
    assert_eq!(comp(format!("A-D")),"0000111");
    assert_eq!(comp(format!("D&A")),"0000000");
    assert_eq!(comp(format!("D|A")),"0010101");
    assert_eq!(comp(format!("M")),"1110000");
    assert_eq!(comp(format!("!M")),"1110001");
    assert_eq!(comp(format!("-M")),"1110011");
    assert_eq!(comp(format!("M+1")),"1110111");
    assert_eq!(comp(format!("M-1")),"1110010");
    assert_eq!(comp(format!("D+M")),"1000010");
    assert_eq!(comp(format!("D-M")),"1010011");
    assert_eq!(comp(format!("M-D")),"1000111");
    assert_eq!(comp(format!("D&M")),"1000000");
    assert_eq!(comp(format!("D|M")),"1010101");
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