extern crate ocl;
extern crate rand;

use crate::preprocessor::preprocess;

#[cfg(test)]
mod tests;
mod cl_part;
mod preprocessor;


/** It is the main */
fn main() {
    let src = vec![';' as char; 1024];
    let mut res = vec![';' as char; src.len()];
    preprocess(&src, &mut res).expect("Unable to parse");
    println!("Nothing to do :)");
    println!("run tests instead");
}


