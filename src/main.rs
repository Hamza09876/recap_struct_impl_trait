//calling a module present in project
mod random_info;
use random_info::*;

#[allow(dead_code)]
//defined a struct
struct DougsData{
    some_int: i32,
    some_float: f64,
    some_bool: bool,
    random: RandomInfo,
}
//here is the proof and reason why impl are not part of struct
//you can extend struct anywhere at any point
impl RandomInfo {   
    pub fn is_greater(&mut self, compare_to:i64)-> bool{
        self.some_int > compare_to
    }
}
#[allow(unused_variables)]
fn main() {
    let mut random_info_var = RandomInfo{
        call_count:0,
        some_bool: true,
        some_int: 22,
    };

    //here we use dot operator because self defined is with lowercase s in impl function
    let is_this_smaller = random_info_var.is_smaller(9);
    let is_this_greater = random_info_var.is_smaller(10);
    //made it mutable so could be changes
    let mut dougs_var = DougsData{
        some_int:80,
        some_float:8.9,
        some_bool:true,
    //here we used double colon to access function because Self is defined on type with capital S
        random: RandomInfo::new(true),
    };

    //trying to mutate 
    dougs_var.some_int = 100;

    //instantiated using existing values
    let dougs_var_2 = DougsData{
        some_int: 7,
        ..dougs_var
    };
}