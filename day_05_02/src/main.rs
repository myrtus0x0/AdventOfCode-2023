use std::fs;
use itertools::Itertools;

#[derive(Copy, Debug, Clone)]
struct Map {
    dest_range_start: u64,
    src_range_start: u64,
    range_length: u64,
}

struct GardenMap {
    translation_maps: Vec<Map>
}

fn parse_map(map_data:&str) -> GardenMap {
    let mut gard_map = GardenMap {
        translation_maps: Vec::new()
    };

    for (i, info) in map_data.split("\n").enumerate() {
        if i == 0 {
            continue;
        }

        let my_map = Map{
            dest_range_start: info.split(" ").nth(0).unwrap().parse::<u64>().unwrap(),
            src_range_start: info.split(" ").nth(1).unwrap().parse::<u64>().unwrap(),
            range_length: info.split(" ").nth(2).unwrap().parse::<u64>().unwrap(),
        };

        gard_map.translation_maps.push(my_map);
    }

    return gard_map
}

fn parse_seed_map(seed_map:String) -> u64 {
    let mut maps: Vec<GardenMap> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();
    for (i, chunk) in seed_map.split("\n\n").enumerate() {
        // parse out the seeds
        if i == 0 {
            let seed_info = chunk.split(": ").nth(1).unwrap().split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            // dbg!(seed_info.clone());
            for mut temp_chunk in &seed_info.iter().chunks(2) {
                let start = temp_chunk.next().unwrap();
                let range = temp_chunk.next().unwrap();
                println!("pushing seeds: {} - {}", start, start+range);
                for j in 0..*range {
                    seeds.push((start+j) as u64);
                }
            }
        } else {
            maps.push(parse_map(chunk));
        }
    }

    let mut final_destinations: Vec<u64> = Vec::new();
    for seed in seeds {
        let mut traversal: u64 = seed;
        // let mut travel_str: String = String::new();
        // travel_str.push_str(&traversal.to_string());
        
        for map in &maps {
            let old_trav = traversal;
            for layout_range in &map.translation_maps {
                if traversal >= layout_range.src_range_start && traversal <= layout_range.src_range_start + layout_range.range_length {
                    let mut diff: u64 = 0;
                    if layout_range.dest_range_start > layout_range.src_range_start {
                        diff = layout_range.dest_range_start - layout_range.src_range_start;
                        traversal = traversal + diff;
                    } else {
                        diff = layout_range.src_range_start - layout_range.dest_range_start;
                        traversal = traversal - diff;
                    }
                    
                    // travel_str.push_str("->");
                    // travel_str.push_str(&traversal.to_string());
                    break
                }
            }
            if traversal == old_trav {
                // travel_str.push_str("->");
                // travel_str.push_str(&traversal.to_string());
            }
        }

        final_destinations.push(traversal);
        // println!("seed: {} = {}", seed, travel_str);
    }

    return *final_destinations.iter().min().unwrap();
}

fn main() {
    let file_path = "./puzzle";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let lowest_dest = parse_seed_map(contents);
    dbg!(lowest_dest);
    

}