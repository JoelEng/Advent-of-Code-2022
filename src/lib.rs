use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut example = false;
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(day))] => format!("../../inputs/{}.in", day.token().to_string()),
        [NestedMeta::Lit(Lit::Int(day)), NestedMeta::Lit(Lit::Int(_))] => {
            example = true;
            format!("../../input_examples/{}.in", day.token().to_string())
        }
        _ => panic!("Expected one integer argument"),
    };

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
      const INPUT: &str = include_str!(#input_path);
      #aoc_solution
      fn main() {
        let now = ::std::time::Instant::now();
        let (p1, p2) = aoc_solution(INPUT.trim_end());
        let time = now.elapsed().as_millis();
        println!("Part one: {}", p1);
        println!("Part two: {}", p2);
        if #example {
          println!("\x1b[101mUSING EXAMPLE INPUT\x1b[0m");
        }
        if time <= 100 {
          print!("\x1b[102m"); // green
        } else if time <= 1000 {
          print!("\x1b[103m"); // yellow
        } else {
          print!("\x1b[101m"); // red
        }
        println!("\x1b[30mTime: {}ms\x1b[0m", time);
      }
    };
    TokenStream::from(tokens)
}
