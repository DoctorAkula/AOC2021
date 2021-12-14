use crate::loadinput::loadinput;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct PairRules {
    pattern : String,
    element : String,
}

#[derive(Clone, Debug)]
struct ElementInsert {
    index : usize,
    element : String,
}

pub fn day14p1() {
    let input = loadinput("./input/day14.txt");
    let mut input_iter = input.iter();
    let mut polymer : String = input_iter.next().unwrap().to_string();
    input_iter.next();
    let pair_rules : Vec<PairRules> = input_iter.map(|val| {
        let mut delim = val.split(" -> ");
        let pattern = delim.next().unwrap().to_string();
        let element = delim.next().unwrap().to_string();
        PairRules{pattern, element}
    }).collect();
//    for pair in &pair_rules {
//        println!("Pattern: {} -> Element: {}", pair.pattern, pair.element);
//    }
//    println!("Seed: {}", polymer);

    for _i in 0..10 {
        let mut element_inserts : Vec<ElementInsert> = Vec::new();
        for index in 0..polymer.len()-1 {
            let poly_pair = &polymer[index..index+2];
            //println!("Polymer pair: {}", poly_pair);
            for pair in &pair_rules {
                if pair.pattern == poly_pair {
                    let element = pair.element.clone();
                    let index = index + 1;
                    element_inserts.push(ElementInsert{index, element});
                }
            }
        }
        element_inserts.sort_by(|a, b| a.index.cmp(&b.index));
        element_inserts.reverse();
        for element_insert in element_inserts {
            polymer.insert_str(element_insert.index, &element_insert.element);
        }
        //println!("Polymer: {}", polymer);
    }
//    println!("Polymer length: {}", polymer.len());
    let mut char_counts : HashMap<char, usize> = HashMap::new();
    for monomer in polymer.chars() {
        if let Some(count) = char_counts.get_mut(&monomer) {
            *count += 1;
        }else{
            char_counts.insert(monomer, 1);
        }
    }
    let mut char_counts : Vec<usize> = char_counts.iter().map(|(_key, val)| *val).collect();
    char_counts.sort();
    println!("Element differential: {}", char_counts.last().unwrap() - char_counts[0]);
}

pub fn day14p2() {
    let input = loadinput("./input/day14.txt");
    let mut input_iter = input.iter();
    let polymer : String = input_iter.next().unwrap().to_string();
    input_iter.next();
    let pair_rules : HashMap<String, String> = input_iter.map(|val| {
        let mut delim = val.split(" -> ");
        let pattern = delim.next().unwrap().to_string();
        let element = delim.next().unwrap().to_string();
        (pattern, element)
    }).collect();

    let mut pair_occurrences : HashMap<String, usize> = HashMap::new();

    for index in 0..polymer.len()-1 {
        let poly_pair = &polymer[index..index+2];
        if let Some(occurrences) = pair_occurrences.get_mut(poly_pair){
            *occurrences += 1;
        }else{
            pair_occurrences.insert(poly_pair.to_string(), 1);
        }
    }

    for _i in 0..40 {
        let mut temp_occ : HashMap<String, usize> = HashMap::new();
        for (pair, occurrences) in pair_occurrences.iter() {
            let insert = pair_rules.get(pair).unwrap();

            let left_pair = &mut pair[0..1].to_string();
            left_pair.push_str(&insert.clone());
            let mut right_pair = insert.to_string();
            right_pair.push_str(&pair[1..2].to_string());
            //println!("Rule: {} -> {}, Left: {}, Right: {}", pair, insert, left_pair, right_pair);

            if let Some(occy) = temp_occ.get_mut(left_pair){
                *occy += occurrences;
            }else{
                temp_occ.insert(left_pair.to_string(), *occurrences);
            }
            if let Some(occy) = temp_occ.get_mut(&right_pair){
                *occy += occurrences;
            }else{
                temp_occ.insert(right_pair.to_string(), *occurrences);
            }
        }

        pair_occurrences = temp_occ;
    }

    //println!("");
    let mut char_counts : HashMap<char, usize> = HashMap::new();
    for (pair, occurrences) in pair_occurrences.iter() {
        //println!("Pair: {}, Occurrences: {}", pair, occurrences);
        let letter = pair.chars().nth(0).unwrap();
        if let Some(count) = char_counts.get_mut(&letter) {
            *count += *occurrences;
        }else{
            char_counts.insert(letter.clone(), *occurrences);
        }
    }
    *char_counts.get_mut(&polymer.chars().last().unwrap()).unwrap() += 1;   //Magic
//    for (letter, count) in char_counts.iter() {
//        println!("Letter: {}, Count: {}", letter, count);
//    }
    let mut char_counts : Vec<usize> = char_counts.iter().map(|(_key, val)| *val).collect();
    char_counts.sort();
    println!("Element differential: {}", char_counts.last().unwrap() - char_counts[0]);
}
