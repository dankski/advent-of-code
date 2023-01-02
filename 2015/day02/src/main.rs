use std::{collections::BinaryHeap, cmp::Reverse};


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

fn main() {
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
