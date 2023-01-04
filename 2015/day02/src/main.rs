use std::{collections::BinaryHeap};


/// BoxDim (length l, width w, height h)
#[derive(Debug)]
struct BoxDim(u32, u32, u32);



fn wrapping_paper_in_sq_feet (bd: BoxDim) -> u32 {
    let (l, w, h) = (bd.0, bd.1, bd.2);
    
    let d1 = l * w;
    let d2 = w * h;
    let d3 = h * l;
    
    let heap = BinaryHeap::from([d1, d2, d3]);

    let slack_paper = heap.into_sorted_vec()[0]; 

    2 * (d1 + d2 + d3) + slack_paper
}

fn read_box_dims (input: &str) -> u32 {
    let dims = input.split('\n').collect::<Vec<&str>>();
    let mut dbs: Vec<BoxDim>  = Vec::new();

    for (_i, d) in dims.iter().enumerate() {

        let ns: Vec<&str>  = d.split('x').collect();

        if ns.len() == 3 {
            let n: Vec<u32> = ns.into_iter().map(|n| n.parse::<u32>().unwrap()).collect();
            dbs.push(BoxDim(n[0], n[1],n [2]));
        }
    }

    let mut total_order = 0;

    for b in dbs {
        total_order = total_order + wrapping_paper_in_sq_feet(b);
    }

    total_order
}

fn main() {
    let puzzle_input = std::fs::read_to_string("assets/puzzle-input.txt").expect("Should contain the puzzle input.");
    println!("Total order: {}", read_box_dims(&puzzle_input));    
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn should_return_58 () {
        let box_dim = BoxDim(2,3,4);

        assert_eq!(58, wrapping_paper_in_sq_feet(box_dim));
    }

    #[test]
    fn should_return_43 () {
        let box_dim = BoxDim(1,1,10);

        assert_eq!(43, wrapping_paper_in_sq_feet(box_dim));
    }
    
}
