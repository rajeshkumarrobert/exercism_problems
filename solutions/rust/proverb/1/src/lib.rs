pub fn build_proverb(list: &[&str]) -> String {
    let list_value = list.iter().cloned().collect::<Vec<_>>();
    let mut res = String::new();
    if list.len() == 0{
        return res;
    }
    if list_value.len() > 1{
        for x in list_value.windows(2){
            let val= format!("For want of a {} the {} was lost.",x[0].to_string(),x[1].to_string());
            res.push_str(&val);
            res.push_str("\n");
        }
        res.push_str(&format!("And all for the want of a {}.",list_value[0]));
    }else {
        res.push_str(&format!("And all for the want of a {}.",list_value[0]));
    }
    res
}
