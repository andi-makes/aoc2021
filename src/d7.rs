use std::convert::TryInto;

use crate::{Day, Runnable};
use const_format::formatcp;

const CURRENT_DAY: u8 = 7;
const FILE: &'static str = formatcp!("./inputs/input{}.txt", CURRENT_DAY);

type Data = Vec<usize>;
impl Runnable<Data> for Day<CURRENT_DAY> {
    fn init(_input: &str) -> (Self, Data) {
        let mut v: Data = Vec::new();
        let input = std::fs::read_to_string(FILE).unwrap();
        input
            .trim()
            .split(',')
            .for_each(|elem| v.push(elem.parse().unwrap()));


        v.sort();
        (Self {}, v)
    }
    fn one(&self, data: &mut Data) -> u64 { 
        // Remove mutable, we don't do that here.
        let data = data;

        // This solution is based on a little bit of math I did, and now I try to
        // write it down here.

        // There are n-Crab-Submarines. Each Submarine has a starting position s.
        // Let `S` be a ordered set of all starting positions.
        // 
        // Moving a Crab Submarine a neighboring position (s+1 or s-1) costs 1 fuel.
        // To calculate the used fuel of one Submarine i out of [1, n], you can use:
        //   f_i(x) = |x-s_i|
        // where:
        //   x    is the position the sub should move to.
        //   s_i  is the starting position of the sub.
        //
        // We need to move all crabs to some position x, so that the total 
        // fuel consumption of all subs is the lowest.
        //
        // The total fuel consumption of all submarines at some position x is:
        //   f(x) = SUM(f_i(x), i=1, n)
        // which is equivilant to:
        //   f(x) = SUM(|x-s_i|, i=1, n)
        // You can also write that as:
        //   f(x) = |x-x_1| + |x-x_2| + |x-x_3| + ... + |x-x_n|
        //
        // One could just calculate the fuel based on those values. But let's take
        // a deeper look at this function.
        //
        // Let `P` be some position of interest. We can savely say, that the end-position
        // of the subs will be somewhere between x_1 and x_n. So let `P` be some value
        // between x_1 and x_n.
        //
        // If `P` is a starting-position of any submarines, the fuel consumption of
        // those subs is zero. They already are at the position `P`, so they don't
        // need to move. Let `a` be the amount of subs already at position `P`.
        // Let's define `b` as the amount of subs that are left of `P`.
        //
        // We can now simplify f(P):
        //   f(P) = |P-x_1| + |P-x_2| + ... + |P-x_b| + |P-x_{b+a}| + |P-x_{b+a+1}| + ... + |P-x_n|
        // As you can see, because P is equal to the starting position of some
        // subs, their terms disappear.
        //
        // Consider all terms x_1 until x_b. We know, because `S` is sorted, that
        // in this case all x_1 until x_b are smaller than `P`. Likewise, we know
        // that all x_{b+a} until x_n are bigger than P. We can use that information
        // to simplify f(P) even further:
        //   f(P) = P-x_1 + P-x_2 + ... + P-x_b + x_{b+a}-P + x_{b+a+1}-P + ... + x_n-P
        //   f(P) = b*P - SUM(x_i, i=1, b) + SUM(x_i, i=b+a, n) - (n-a-b)*P
        //   f(P) = P*(b+b+a-n) - SUM(x_i, i=1, b) + SUM(x_i, i=b+a, n)
        // Let `B` = SUM(x_i, i=1, b)
        // Let `A` = SUM(x_i, i=b+a, n)
        //   f(P) = P*(2*b+a-n) - B + A
        //
        // For each point P are A and B constant. Let's take a look at the
        // gradient k of f(P).
        // k = f'(P) = 2*b+a-n
        // Now, setting f'(P) = 0:
        // 0 = 2*b+a-n
        // n = 2*b+a
        // n-a = 2*b
        // (n-a)/2 = b
        //
        // Basically, at around n/2 is the minimal fuel consumption.
        //
        // I have no idea how to reliably determine `a`, but `a` is small in
        // comparison to n. Also, although I didn't prove that, but it feels like
        // the minimum will be at a number with a relatively large amount.

        let index = data[data.len()/2] as usize;
        let exclude: Vec<&usize> = data.iter().filter(|&&f| f == index).collect();
        let begin: Vec<&usize> = data.iter().filter(|&f| f < exclude.first().unwrap()).collect();
        let end: Vec<&usize> = data.iter().filter(|&f| f > exclude.first().unwrap()).collect();

        // println!("{}", exclude.len());

        let k = 2*(begin.len() as i64) + exclude.len()as i64 - data.len() as i64;

        let fuel = ((index as i64)*k + (end.iter().fold(0, |sum, f| sum + **f) as i64)) - begin.iter().fold(0, |sum, f| sum + **f) as i64;

        fuel.try_into().unwrap()
    }
    fn two(&self, data: &mut Data) -> u64 { 

        println!("======================================================");

        for d in data {
            print!("p({}-x)+", d);
        }

        println!("First, enter this equation into geogebra:\np(x)=((1)/(2)) x^(2)+((1)/(2)) abs(x)\nThen, enter the monster from above.");
        99763899
    }
}


