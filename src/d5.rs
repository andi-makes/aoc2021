use crate::{Runnable, Day};

type Data = Vec<i32>;
impl Runnable<Data> for Day<5> {
    fn init() -> (Self, Data) {
        let v = vec![0];
        (Self {}, v)
    }
    fn one(&self, _data: &mut Data) {
        
    }
    fn two(&self, _data: &mut Data) {
        
    }
}

