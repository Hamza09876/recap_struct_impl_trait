pub struct RandomInfo{
    pub some_bool: bool,
    pub some_int: i64,
}
impl RandomInfo {
    pub fn new(param_a: bool) -> Self{
        Self { 
            some_bool: !param_a, 
            some_int: 8 
        }
    }

    pub fn is_smaller(&self, compare_to:i64)-> bool{
        self.some_int < compare_to
    }
}
