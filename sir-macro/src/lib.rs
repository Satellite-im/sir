mod compile_style_item;

use crate::compile_style_item::{inner_compile_global_style, inner_compile_style_item};
use proc_macro::TokenStream;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::iter;

#[proc_macro]
pub fn compile_style_item(item: TokenStream) -> TokenStream {
    inner_compile_style_item(generate_class_name(&mut thread_rng()), item.into()).into()
}

#[proc_macro]
pub fn compile_global_style(item: TokenStream) -> TokenStream {
    inner_compile_global_style(item.into()).into()
}

const ID_LENGTH: usize = 15;
const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

fn generate_class_name(mut rng: &mut impl Rng) -> String {
    let chars: Vec<_> = ALPHABET.chars().collect();

    let class = iter::once(&'_')
        .chain((0..ID_LENGTH).map(|_| chars.choose(&mut rng).unwrap()))
        .collect();

    class
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_generate_class_name() {
        let mut rng = StdRng::seed_from_u64(42);
        // snapshot
        assert_eq!(generate_class_name(&mut rng), "_t5bwm0ra7vs5of3");
    }
}
