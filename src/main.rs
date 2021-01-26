use std::fs;

fn main() {
    let file: Vec<u8> = fs::read("./input/CU_steg.bmp").unwrap();

    let q_one = parse(String::from("40"), 16, 16, 6, &file);
    println!("Question 1: {}", q_one);
    let q_two = parse(String::from("2710"), 100, 44, 6, &file);
    println!("Question 2: {}", q_two);
}

fn parse(hex_start_index: String, interval: u32, char_count: usize, binary_len: usize, parse_vec: &Vec<u8>) -> String {
    
    let compare = u32::from_str_radix(&hex_start_index, 16).unwrap();

    let mut final_str: String = String::from("");
    let mut to_finish = false;

    for (i, hex) in parse_vec.iter().enumerate() {
        // if it is an interval of binary_len and the character count is equal to the requested char count
        // end the loop and print.
        if final_str.len() % binary_len == 0 && final_str.len() / binary_len == char_count {
            break;
        }
        if compare == i as u32 {
            to_finish = true;
        }
        if !to_finish {
            continue;
        }

        if i as u32 % interval == 0 {
            let bin_rep: u8 = hex & 0x01;

            &final_str.push_str(&format!("{}", bin_rep));
        }
    }

    final_str
}
