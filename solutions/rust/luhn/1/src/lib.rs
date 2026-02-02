/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut numeric_char = code.chars().collect::<Vec<_>>();
    numeric_char.retain(|c| !c.is_whitespace());
    if numeric_char.len() <= 1 || numeric_char.iter().any(|x| !x.is_numeric()){
        return false;
    }
    let numeric_number = numeric_char.iter().map(|x| x.to_digit(10)).collect::<Vec<_>>();
    let mut numeric_number= numeric_number.iter().filter_map(|x| *x).collect::<Vec<_>>();
    numeric_number.reverse();
    let mut credit_card = vec![];
    for (index, value) in numeric_number.iter().enumerate(){
        if index % 2 != 0{
            let mut temp_value = *value as i32  * 2;
            if temp_value > 9{
                temp_value-=9;
            }
            credit_card.push(temp_value);
        }else {
            credit_card.push(*value as i32);
        }
    }
    let res = credit_card.iter().sum::<i32>();
    res%10 == 0
}
