use crate::loadinput::loadinput;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OperatorType {
    Bitcount(usize),
    Pakcount(usize),
    Immediate,
}

#[derive(Clone, Debug)]
struct Packet {
    packet_id : usize,
    type_id : usize,
    sub_packet_count : OperatorType,
    sub_packets : Vec<Packet>,
    immediate : usize,
}

fn len_id(bin_string : &String, index : usize) -> (OperatorType, usize) {
    match bin_string.chars().nth(index).unwrap() {
        '0' => {
            let bitlen = usize::from_str_radix(&bin_string[index+1..index+16], 2).unwrap();
            (OperatorType::Bitcount(bitlen), index+16)
        },
        '1' => {
            let paklen = usize::from_str_radix(&bin_string[index+1..index+12], 2).unwrap();
            (OperatorType::Pakcount(paklen), index+12)
        },
        _ => {
            eprintln!("3: Uh oh");
            (OperatorType::Immediate, 0)
        },
    }
}

fn parse_immediate(bin_string : &String, mut index : usize) -> (usize, usize) {
    let mut done = false;
    let mut nibbles : Vec<usize> = Vec::new();
    while !done {
        let nibble = &bin_string[index..index+5];
        let lead = nibble.chars().nth(0).unwrap();
        index += 5;
        if lead == '0' {
            done = true;
            nibbles.push(usize::from_str_radix(&nibble[1..], 2).unwrap());
        }else if lead == '1' {
            nibbles.push(usize::from_str_radix(&nibble[1..], 2).unwrap());
        }
    }
    let mut immediate = 0;
    for val in nibbles {
        immediate <<= 4;
        immediate += val;
    }
    (immediate, index)
}

fn parse_bin_string(bin_string : &String, packets : &mut Vec<Packet>,
                    mut packet_index : usize, mut packets_left : OperatorType) -> usize {
    let mut done = false;
    while !done {
        let initial_index = packet_index;
        let packet_id = usize::from_str_radix(&bin_string[packet_index..packet_index+3], 2).unwrap();
        //println!("Packet ID {}", packet_id);
        packet_index += 3;
        let type_id = &bin_string[packet_index..packet_index+3];
        match type_id {
            "000" |
            "001" |
            "010" |
            "011" |
            "101" |
            "110" |
            "111" => {
                let type_id = usize::from_str_radix(type_id, 2).unwrap();
                //println!("Type ID {}", type_id);
                let (sub_packet_count, next_index) = len_id(&bin_string, packet_index+3);
                packet_index = next_index;
                let mut packet = Packet {
                    packet_id,
                    type_id,
                    sub_packet_count,
                    sub_packets : Vec::new(),
                    immediate : 0,
                };
                //println!("Type ID {} Sub packet count: {:#?}, Packets Left: {:#?}", type_id, sub_packet_count, packets_left);
                packet_index = parse_bin_string(bin_string, &mut packet.sub_packets, packet_index, sub_packet_count);
                packets.push(packet);
                match packets_left {
                    OperatorType::Bitcount(mut thingey) => {
                        thingey -= packet_index - initial_index;
                        packets_left = OperatorType::Bitcount(thingey);
                        if thingey == 0 { done = true; }
                    },
                    OperatorType::Pakcount(mut thingey) => {
                        thingey -= 1;
                        packets_left = OperatorType::Pakcount(thingey);
                        if thingey == 0 { done = true; }
                    }
                    OperatorType::Immediate => eprintln!("4: Uh oh"),
                }
            },
            "100" => {
                let type_id = usize::from_str_radix(type_id, 2).unwrap();
                let (immediate, next_index) = parse_immediate(bin_string, packet_index+3);
                packet_index = next_index;
                let packet = Packet {
                    packet_id,
                    type_id,
                    sub_packet_count : OperatorType::Immediate,
                    sub_packets : Vec::new(),
                    immediate,
                };
                //println!("Type ID {} Sub packet count: Immediate, Packets Left: {:#?}", type_id, packets_left);
                packets.push(packet);
                match packets_left {
                    OperatorType::Bitcount(mut thingey) => {
                        thingey -= packet_index - initial_index;
                        packets_left = OperatorType::Bitcount(thingey);
                        if thingey == 0 { done = true; }
                    },
                    OperatorType::Pakcount(mut thingey) => {
                        thingey -= 1;
                        packets_left = OperatorType::Pakcount(thingey);
                        if thingey == 0 { done = true; }
                    }
                    OperatorType::Immediate => eprintln!("4: Uh oh"),
                }
            },
            _ => eprintln!("2: Uh oh"),
        }
        if (bin_string.len() - packet_index) < 11 {
            done = true;
        }
    }
    packet_index
}

fn sum_pids(packets : &Vec<Packet>) -> usize {
    let mut accumulator = 0;
    for packet in packets {
        accumulator += packet.packet_id;
        accumulator += sum_pids(&packet.sub_packets);
    }
    accumulator
}

fn create_bin_string(hexadecimal : &String) -> String {
    let mut bin_string : String = String::new();
    for chr in hexadecimal.chars() {
        match chr {
            '0' => { bin_string.push('0'); bin_string.push('0'); bin_string.push('0'); bin_string.push('0'); },
            '1' => { bin_string.push('0'); bin_string.push('0'); bin_string.push('0'); bin_string.push('1'); },
            '2' => { bin_string.push('0'); bin_string.push('0'); bin_string.push('1'); bin_string.push('0'); },
            '3' => { bin_string.push('0'); bin_string.push('0'); bin_string.push('1'); bin_string.push('1'); },
            '4' => { bin_string.push('0'); bin_string.push('1'); bin_string.push('0'); bin_string.push('0'); },
            '5' => { bin_string.push('0'); bin_string.push('1'); bin_string.push('0'); bin_string.push('1'); },
            '6' => { bin_string.push('0'); bin_string.push('1'); bin_string.push('1'); bin_string.push('0'); },
            '7' => { bin_string.push('0'); bin_string.push('1'); bin_string.push('1'); bin_string.push('1'); },
            '8' => { bin_string.push('1'); bin_string.push('0'); bin_string.push('0'); bin_string.push('0'); },
            '9' => { bin_string.push('1'); bin_string.push('0'); bin_string.push('0'); bin_string.push('1'); },
            'A' => { bin_string.push('1'); bin_string.push('0'); bin_string.push('1'); bin_string.push('0'); },
            'B' => { bin_string.push('1'); bin_string.push('0'); bin_string.push('1'); bin_string.push('1'); },
            'C' => { bin_string.push('1'); bin_string.push('1'); bin_string.push('0'); bin_string.push('0'); },
            'D' => { bin_string.push('1'); bin_string.push('1'); bin_string.push('0'); bin_string.push('1'); },
            'E' => { bin_string.push('1'); bin_string.push('1'); bin_string.push('1'); bin_string.push('0'); },
            'F' => { bin_string.push('1'); bin_string.push('1'); bin_string.push('1'); bin_string.push('1'); },
            _ => eprintln!("1: Uh oh"),
        }
    }
    bin_string
}

pub fn day16p1() {
    let input = loadinput("./input/day16.txt");
    let bin_string = create_bin_string(&input[0]);
    //println!("{}", bin_string);

    let mut packets : Vec<Packet> = Vec::new();
    parse_bin_string(&bin_string, &mut packets, 0, OperatorType::Bitcount(bin_string.len()));
    let sum = sum_pids(&packets);
    println!("Version sum: {}", sum);
}

fn calc_packets(packets : &Vec<Packet>, operation : usize) -> usize {
    let mut accumulator = 0;
    match operation {
        0 => {  //Sum
            for packet in packets {
                if packet.sub_packet_count == OperatorType::Immediate {
                    accumulator += packet.immediate;
                }else{
                    accumulator += calc_packets(&packet.sub_packets, packet.type_id);
                }
            }
        },
        1 => {  //Product
            let mut multiplier = 1;
            for packet in packets {
                if packet.sub_packet_count == OperatorType::Immediate {
                    multiplier *= packet.immediate;
                }else{
                    multiplier *= calc_packets(&packet.sub_packets, packet.type_id);
                }
            }
            accumulator += multiplier;
        },
        2 => {  //Minimum
            let mut vals : Vec<usize> = Vec::new();
            for packet in packets {
                if packet.sub_packet_count == OperatorType::Immediate {
                    vals.push(packet.immediate);
                }else{
                    vals.push(calc_packets(&packet.sub_packets, packet.type_id));
                }
                vals.sort();
                accumulator = vals[0];
            }
        },
        3 => {  //Maximum
            let mut vals : Vec<usize> = Vec::new();
            for packet in packets {
                if packet.sub_packet_count == OperatorType::Immediate {
                    vals.push(packet.immediate);
                }else{
                    vals.push(calc_packets(&packet.sub_packets, packet.type_id));
                }
                vals.sort();
                accumulator = *vals.last().unwrap();
            }
        },
        5 => {  //Greater than
            let mut vals : Vec<usize> = Vec::new();
            for packet in packets {
                if packet.sub_packet_count == OperatorType::Immediate {
                    vals.push(packet.immediate);
                }else{
                    vals.push(calc_packets(&packet.sub_packets, packet.type_id));
                }
            }
            if vals[0] > vals[1] {
                accumulator = 1;
            }
        },
        6 => {  //Less than
            let mut vals : Vec<usize> = Vec::new();
            for packet in packets {
                if packet.sub_packet_count == OperatorType::Immediate {
                    vals.push(packet.immediate);
                }else{
                    vals.push(calc_packets(&packet.sub_packets, packet.type_id));
                }
            }
            if vals[0] < vals[1] {
                accumulator = 1;
            }
        },
        7 => {  //Equal to
            let mut vals : Vec<usize> = Vec::new();
            for packet in packets {
                if packet.sub_packet_count == OperatorType::Immediate {
                    vals.push(packet.immediate);
                }else{
                    vals.push(calc_packets(&packet.sub_packets, packet.type_id));
                }
            }
            if vals[0] == vals[1] {
                accumulator = 1;
            }
        },
        _ => eprintln!("Invalid operation!"),
    }
    accumulator
}

pub fn day16p2() {
    let input = loadinput("./input/day16.txt");
    let bin_string = create_bin_string(&input[0]);
    //println!("{}", bin_string);

    let mut packets : Vec<Packet> = Vec::new();
    parse_bin_string(&bin_string, &mut packets, 0, OperatorType::Bitcount(bin_string.len()));
//    for packet in &packets {
//        println!("{:#?}", packet);
//    }
    let val = calc_packets(&packets, 0);
    println!("Value: {}", val);
}
