use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn mod_days(_input: TokenStream) -> TokenStream {
    let mut res = String::new();
    for i in 1..26 {
        res += &format!("mod d{};", i);
    }
    res.parse().unwrap()
}

#[proc_macro]
pub fn generate_day_match(_input: TokenStream) -> TokenStream {
    let r = 1_u8..26; // == [1,25]
    let res = quote! {
        match day {
            #(#r => {
                let input = &std::fs::read_to_string(format!("./inputs/input{}.txt", #r)).unwrap();
                let start = Instant::now();
                let (d, mut data) = Day::<#r>::init(input);
                let delta = start.elapsed();
                println!("Data parsing: {}us", delta.as_micros());
                d.run(&mut data, run);
            })*
            _ => panic!("Days out of Bounds! No presents for you!"),
        }
    };
    res.into()
}

