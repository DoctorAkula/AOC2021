use crate::loadinput::loadinput;

pub fn day08p1() {
    let input = loadinput("./input/day8.txt");
    let digit_quads : Vec<String> = input.iter().map(|val|
        (*val.split('|').last().unwrap()).to_string()).collect();

    let mut individual_digits : Vec<String> = Vec::new();
    for digit_quad in digit_quads {
        let mut digits : Vec<String> = digit_quad.split(' ').map(|val| (*val).to_string()).collect();
        individual_digits.append(&mut digits);
    }

    let unique_digits = individual_digits.iter().fold(0, |acc,val| {
        match val.len() {
            7 => acc + 1,
            4 => acc + 1,
            3 => acc + 1,
            2 => acc + 1,
            _ => acc
        }
    });
    println!("Unique digits: {}", unique_digits);
}

pub fn day08p2() {
    let input = loadinput("./input/day8.txt");
    let (digit_samples_list, digit_quads) : (Vec<String>, Vec<String>) = input.iter().map(|val| {
        let mut iter = val.split('|');
        ((*iter.next().unwrap()).to_string(),
        (*iter.next().unwrap()).to_string())
    }).unzip();

    let mut accumulator = 0;
    for (digit_samples, digit_quad) in digit_samples_list.iter().zip(&digit_quads) {
        let mut known : [&str; 10] = [""; 10];
        let digit_samples : Vec<&str> = digit_samples.split(' ').collect();
        let digit_quad : Vec<&str> = digit_quad.split(' ').collect();

        for digit_sample in &digit_samples {  /*Gets 1,4,7, and 8*/
            match digit_sample.len() {
                7 => known[8] = digit_sample,
                4 => known[4] = digit_sample,
                3 => known[7] = digit_sample,
                2 => known[1] = digit_sample,
                _ => ()
            }
        }

        for digit_sample in &digit_samples {  /*Finds 3*/
            if (digit_sample.len() == 5) && 
                digit_sample.contains(&known[1][0..1]) && 
                digit_sample.contains(&known[1][1..2]) {
                    known[3] = digit_sample;
            }
        }

//        let top_segment : &str = {     /*Finds top segment*/
//            let mut seven = known[7].to_string();
//            seven.remove(seven.find(&known[1][0..1]).unwrap());
//            seven.remove(seven.find(&known[1][1..2]).unwrap());
//            &seven.clone()
//        };

        let middle_segment : &str = {   /*Finds middle segment*/
            let mut four = known[4].to_string();
            four.remove(four.find(&known[1][0..1]).unwrap());
            four.remove(four.find(&known[1][1..2]).unwrap());
            let mut mseg : char = ' ';
            for seg in four.chars() {
                if known[3].contains(seg) {
                    mseg = seg;
                    break;
                }
            }
            &mseg.to_string()
        };

        let topleft_segment : &str = {  /*Finds top left segment*/
            let mut four = known[4].to_string();
            four.remove(four.find(&known[1][0..1]).unwrap());
            four.remove(four.find(&known[1][1..2]).unwrap());
            four.remove(four.find(middle_segment).unwrap());
            &four.clone()
        };

        for digit_sample in &digit_samples {  /*Finds 0*/
            if (digit_sample.len() == 6) && 
                !digit_sample.contains(middle_segment){
                    known[0] = digit_sample;
            }
        }

        for digit_sample in &digit_samples {  /*Finds 2*/
            if (digit_sample.len() == 5) && 
                !digit_sample.contains(topleft_segment) &&
                !(&known[3] == digit_sample) {
                    known[2] = digit_sample;
            }
        }

        let topright_segment : &str = {  /*Finds top right segment*/
            let two = known[2].to_string();
            let mut ret = "";
            if two.contains(&known[1][0..1]) {
                ret =  &known[1][0..1];
            }else if two.contains(&known[1][1..2]) {
                ret = &known[1][1..2];
            }
            ret
        };

        for digit_sample in &digit_samples {  /*Finds 5*/
            if (digit_sample.len() == 5) && 
                !digit_sample.contains(topright_segment){
                    known[5] = digit_sample;
            }
        }

        for digit_sample in &digit_samples {  /*Finds 6*/
            if (digit_sample.len() == 6) && 
                !digit_sample.contains(topright_segment){
                    known[6] = digit_sample;
            }
        }

        for digit_sample in &digit_samples {  /*Finds 9 (Through process of elimination)*/
            if digit_sample.len() == 6 {
                let mut matched = false;
                for val in known {
                    if &val == digit_sample {
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    known[9] = digit_sample;
                    break;
                }
            }
        }

 //       for digit_sample in &digit_samples {
 //           print!("{}, ", digit_sample);
 //       }
 //       println!("");

 //       /*Visibile digit check*/
 //       for (num, val) in known.iter().enumerate() {
 //           println!("Number {} = {}", num, val);
 //       }
 //       println!("");
 //       println!("Top segment: {}", top_segment);
 //       println!("Middle segment: {}", middle_segment);
 //       println!("Top left segment: {}", topleft_segment);
 //       println!("Top right segment: {}", topright_segment);

        /*Convert digits to binary number*/
        let mut value = 0;
        for (pos, digit) in digit_quad.iter().enumerate() {
            if digit == &"" { continue; }
            let mut binary = 10;

            for (num, val) in known.iter().enumerate() {    /*Find the correct digit*/
                if val.len() != digit.len() {
                    continue;
                }
                binary = num;
                for seg in val.chars() {
                    if !digit.contains(seg) {
                        binary = 10;
                        break;
                    }
                }
                if binary != 10 {
                    break;
                }
            }

//            print!("{}, ", digit);
            if binary == 10 {
//                eprint!("Uh oh ");
            }else{
                value += 10_usize.pow((digit_quad.len() - pos - 1) as u32) * binary;
            }
        }
//        println!("");
//        println!("Value: {}", value);
//        println!("---END SEARCH---\n");
        accumulator += value;
    }
    println!("Final sum: {}", accumulator);
}
