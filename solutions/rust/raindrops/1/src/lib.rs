pub fn raindrops(n: u32) -> String {
     let res = match (n.is_multiple_of(3),n.is_multiple_of(5),n.is_multiple_of(7)) {
      (true,true,true)  => String::from("PlingPlangPlong"),
      (false,true,true) => String::from("PlangPlong"),
      (true,false,true)  => String::from("PlingPlong"),
      (true,true,false)  => String::from("PlingPlang"),
      (false,false,true)  => String::from("Plong"),
      (true,false,false)  => String::from("Pling"),
      (false,true,false)  => String::from("Plang"),
      (false,false,false)  => String::from(n.to_string()),
    };
    res   
}