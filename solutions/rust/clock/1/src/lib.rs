use std::fmt;
#[derive(PartialEq,PartialOrd,Debug)]
pub struct Clock{
    hours:i32,
    minutes:i32
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) ->Self{

        let res :(i32,i32)=reduce_numbers_to_hours(hours,minutes);
        println!("The result Value{:0>2}, {:0>2}",res.0,res.1);
        Clock{hours:res.0, minutes:res.1}
    }

    pub fn add_minutes(&self, mut minutes: i32) -> Self {
        let hours = self.hours;
        minutes=self.minutes+minutes;
       let res:(i32,i32) = reduce_to_small_mins(hours, minutes);
       Clock{hours:res.0, minutes:res.1}
    }
}
pub fn reduce_to_small_mins(hours:i32 ,minutes: i32)->(i32,i32){
        let mut mins:i32 =minutes;
        let mut hrs:i32=hours;
        if minutes>=60 || minutes<= -60{
            hrs += minutes/60;
            mins = minutes%60; 
            if mins.is_negative(){
                hrs = hrs-1;
                mins = 60+mins;
            }
            return reduce_numbers_to_hours(hrs,mins);
        }else {
            if mins.is_negative(){
                if hrs==0{
                    hrs=24-1;
                    mins=60+mins;
                }else{
                    hrs=hrs-1;
                    mins=60+mins;
                }
                return (hrs,mins);
            }
            (hrs,mins) 
        }    
}
pub fn reduce_numbers_to_hours(hours: i32,minutes: i32)->(i32,i32){
    let (mut hrs,mins)=reduce_to_small_mins(hours, minutes);

    if hrs>=24 || hrs<= -24{
        let mut act_hrs = hrs%24;
        if act_hrs.is_negative(){
           act_hrs = 24+act_hrs;
        }
        return (act_hrs,mins)
    }else {
        if hrs.is_negative(){
            hrs = 24+hrs;
            return (hrs,mins)
        }
        (hrs,mins)
    }
}