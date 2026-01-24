#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let res = match (first_list.len(),second_list.len()){
        (0,0) => Comparison::Equal,
        (0,_) => Comparison::Sublist,
        (_,0) => Comparison::Superlist,
        _ => check_comparison(first_list, second_list)
    };
    res
}

pub fn check_comparison(first_list: &[i32], second_list: &[i32]) -> Comparison{
    if first_list == second_list{
            return Comparison::Equal;
        }else if first_list.windows(second_list.len()).any(|arr| arr==second_list) {
            return Comparison::Superlist;
        }else if second_list.windows(first_list.len()).any(|arr| arr == first_list) {
            return Comparison::Sublist;
        }else {
            return Comparison::Unequal;
        }
}