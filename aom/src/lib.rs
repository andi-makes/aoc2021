use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_day_match(_input: TokenStream) -> TokenStream {
    let r = 1_u8..26; // == [1,25]
    let res = quote! {
        match day {
            #(#r => {
                let (d, mut data) = Day::<#r>::init();
                d.run(&mut data, run);
            })*
            _ => panic!("Days out of Bounds! No presents for you!"),
        }
    };
    res.into()
}

