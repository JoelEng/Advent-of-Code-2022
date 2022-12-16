use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut example = false;
    let day;
    let input_path = match &parse_macro_input!(args as AttributeArgs)[..] {
        [NestedMeta::Lit(Lit::Int(lit_day))] => {
            day = lit_day.token().to_string();
            format!("../../inputs/{}.in", day)
        }
        [NestedMeta::Lit(Lit::Int(lit_day)), NestedMeta::Lit(Lit::Int(_))] => {
            example = true;
            day = lit_day.token().to_string();
            format!("../../input_examples/{}.in", day)
        }
        _ => panic!("Expected one integer argument"),
    };

    let mut aoc_solution = parse_macro_input!(input as ItemFn);
    aoc_solution.sig.ident = Ident::new("aoc_solution", aoc_solution.sig.ident.span());

    let tokens = quote! {
      /// Returns `actual` when using real problem input, and `example` when using example input
      fn aox<T>(actual: T, example: T) -> T {
        if #example {
          example
        } else {
          actual
        }
      }

      const INPUT: &str = include_str!(#input_path);
      #aoc_solution
      fn main() {
        println!("\x1b[4;1mDay {}:\x1b[0m", #day);
        let now = ::std::time::Instant::now();
        let (p1, p2) = aoc_solution(INPUT.trim_end());
        let time = now.elapsed();

        let file = std::fs::read_to_string(format!("answers/{}.sol", #day)).unwrap();
        let ans1 = onig::Regex::new(r"part one: ([^\n]*)").unwrap().captures_iter(&file).next().unwrap().at(1).unwrap();
        let ans2 = onig::Regex::new(r"part two: ([^\n]*)").unwrap().captures_iter(&file).next().unwrap().at(1).unwrap();


        print!("Part one: ");
        if ans1 != "" && !#example {
          if ans1 == p1.to_string() {
            print!("\x1b[32m");
          } else {
            print!("\x1b[31m");
          }
        }
        println!("{}\x1b[0m", p1);

        print!("Part two: ");
        if ans2 != "" && !#example {
          if ans2 == p2.to_string() {
            print!("\x1b[32m");
          } else {
            print!("\x1b[31m");
          }
        }
        println!("{}\x1b[0m", p2);
        if #example {
          println!("\x1b[101mUSING EXAMPLE INPUT\x1b[0m");
        } else {
          if time.as_millis() <= 10 {
            print!("\x1b[102m"); // green
          } else if time.as_millis() <= 1000 {
            print!("\x1b[103m"); // yellow
          } else {
            print!("\x1b[101m"); // red
          }
          print!("\x1b[30m");
        }
        if time.as_millis() > 0 {
          print!("Time: {}ms", time.as_millis());
        } else {
          print!("Time: {}μs", time.as_micros());
        }
        println!("\x1b[0m");
      }
    };
    TokenStream::from(tokens)
}
