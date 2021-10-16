#!/bin/bash

for i in {1..25}
do
echo "use crate::{Runnable, Day};

type Data = Vec<i32>;
impl Runnable<Data> for Day<$i> {
    fn init() -> (Self, Data) {
        let v = vec![0];
        (Self {}, v)
    }
    fn one(&self, _data: &mut Data) {
        
    }
    fn two(&self, _data: &mut Data) {
        
    }
}
" >> "src/d$i.rs"
done

