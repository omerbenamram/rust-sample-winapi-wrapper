pub fn nul_terminated_i8_arr_to_string(arr: &[i8]) -> String {
    let mut result: String = String::new();
    for integer in arr.iter() {
        if *integer == 0 {
            break;
        }
        result.push(*integer as u8 as char)
    }
    result
}