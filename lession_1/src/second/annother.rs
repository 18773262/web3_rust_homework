pub fn loop_ua_z() {
    println!("loop A to z");
    for ch in 'A'..='z' {
        println!("{ch}");
    }
}
pub fn print_ascii_start_with_a() {
    let start: u8 = b'A';
    let total_rows = 15;

    for row in 0..total_rows {
        for col in 0..4 {
            let index = (row + col * total_rows) as u8;
            let ascii_value = start + index;
            print!("{:<4}{:<5}｜", ascii_value, ascii_value as char);
        }
        println!();
    }
}

pub fn from_str_to_array() {
    let s = "你好世界 Hello, world!";
    let chars: Vec<char> = s.chars().collect(); println!("vec: {chars:?}");

    let length =  s.chars().count();
    println!("length: {length}");

    let mut v = Vec::with_capacity(length);

    for (idx, item) in s.chars().enumerate() { v[idx] = item;
    }
    println!("array: {v:?}");

    let mut iter = v.iter().enumerate();
    assert_eq!(Some((0, &'你')), iter.next());
}

