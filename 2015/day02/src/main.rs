use std::{collections::{BinaryHeap, BTreeMap}};


/// BoxDim (length l, width w, height h)
#[derive(Debug)]
struct BoxDim(u32, u32, u32);

fn wrapping_paper_in_sq_feet (present: &BoxDim) -> u32 {
    let (l, w, h) = (present.0, present.1, present.2);
    
    let d1 = l * w;
    let d2 = w * h;
    let d3 = h * l;
    
    let heap = BinaryHeap::from([d1, d2, d3]);

    let slack_paper = heap.into_sorted_vec()[0]; 

    2 * (d1 + d2 + d3) + slack_paper
}

fn read_box_dims (presents: &Vec<BoxDim>) -> u32 {
    
    let mut total_order = 0;

    for present in presents {
        total_order = total_order + wrapping_paper_in_sq_feet(present);
    }

    total_order
}

fn needed_ribbon_per_present (present: &BoxDim) -> u32 {
    let (l, w, h) = (present.0, present.1, present.2);
    let (d1, d2) = shortest_persent_perimeter(&present);
    
    let required_ribbon = 2 * d1 + 2 * d2; 
    let required_bow = l * w * h;
    
    required_ribbon + required_bow
}

fn shortest_persent_perimeter (present: &BoxDim) -> (u32, u32) {
    let (l, w, h) = (present.0, present.1, present.2);
    let mut perimeters: BTreeMap<u32, (u32, u32)> = BTreeMap::new();

    perimeters.insert(l * w, (l,w)); 
    perimeters.insert(l * h, (l,h)); 
    perimeters.insert(w * h, (w,h)); 
    
    *perimeters.values().next().unwrap()
}

fn calculate_needed_ribbon (presents: &Vec<BoxDim>) -> u32 {
    let mut total_ribbon = 0;

    for p in presents {
        total_ribbon = total_ribbon + needed_ribbon_per_present(&p);
    }

    total_ribbon
}

fn map_input_to_box_dims (input: &str) -> Vec<BoxDim> {

    let dims = input.split('\n').collect::<Vec<&str>>();
    let mut presents: Vec<BoxDim>  = Vec::new();

    for d in dims {

        let ns: Vec<&str>  = d.split('x').collect();

        if ns.len() == 3 {
            let n: Vec<u32> = ns.into_iter().map(|n| n.parse::<u32>().unwrap()).collect();
            presents.push(BoxDim(n[0], n[1],n [2]));
        }

    }

    presents
}

fn main() {
    let puzzle_input = std::fs::read_to_string("assets/puzzle-input.txt").expect("Should contain the puzzle input.");
    let presents: Vec<BoxDim> = map_input_to_box_dims(&puzzle_input);

    println!("Total presents: {}", presents.len());

    println!("Total wrapping paper: {}", read_box_dims(&presents));    
    println!("Total ribbon + bow: {}", calculate_needed_ribbon(&presents));
}


#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn should_return_58 () {
        let box_dim = BoxDim(2,3,4);

        assert_eq!(58, wrapping_paper_in_sq_feet(&box_dim));
    }

    #[test]
    fn should_return_43 () {
        let box_dim = BoxDim(1,1,10);

        assert_eq!(43, wrapping_paper_in_sq_feet(&box_dim));
    }
   
    #[test]
    fn should_return_34_for_ribbon_plus_bow () {
        let box_dim = BoxDim(2,3,4);

        assert_eq!(34, needed_ribbon_per_present(&box_dim));
    }

    #[test]
    fn should_return_14_for_ribbon_plus_bow () {
        let box_dim = BoxDim(1,1,10);

        assert_eq!(14, needed_ribbon_per_present(&box_dim));
    }
}
