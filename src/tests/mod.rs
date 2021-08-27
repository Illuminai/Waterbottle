use super::*;
use preprocessor::preprocess;
use std::iter::FromIterator;

#[test]
fn asdfasdf() {
    let src_as_string =
r####"
pub fn test<'a, B>( &self, u: usize) {let k = 3;
println!("{}", k); k}"####;

    /*{
        let mut chars = src_as_string.chars();
        let mut i = 0;
        while let Some(c) = chars.next() {
            println!("{} {:?}", i, c);
            i += 1;
        }
    }*/

    let src_code = src_as_string.chars().collect::<Vec<char>>();

    let mut result_text = Vec::<char>::with_capacity(src_code.len());
    for _ in 0..src_code.len() {
        result_text.push(' ');
    }

    let comments = preprocess(&src_code,result_text.as_mut_slice()).unwrap();
    println!("{}", comments.len());

    for c in comments {
        println!("{:?}", c);
    }

    println!("{:?}", result_text);
    println!("{}", String::from_iter(&result_text));
}