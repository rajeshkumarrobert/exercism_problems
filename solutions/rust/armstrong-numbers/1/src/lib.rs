pub fn is_armstrong_number(num: u32) -> bool {
    let digit_num = num;
    let mut res = 0;
    let char_num = num.to_string().chars().collect::<Vec<_>>();
    let len = char_num.len() as u32;
    for x in char_num{
        let val = x.to_digit(10).unwrap();
        res += val.pow(len);
    }
    digit_num == res 
}
