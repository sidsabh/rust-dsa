use std::{cmp::max, collections::HashMap};

use dsa_lib::io::Scanner;

fn solve(a_reader: &mut Scanner<impl std::io::BufRead>, a_writer: &mut impl std::io::Write) {

    let num_testcases = a_reader.next();

    for _ in 0..num_testcases {

        let num_locations : usize = a_reader.next();

        let mut path_lengths: HashMap<usize, Vec<u32>> = HashMap::new();

        let all_locations = (1..=num_locations).map(|n| n.to_string()).collect::<Vec<String>>().join(" ");
        let mut global_max_length = usize::MIN;
        for location in 1..=num_locations {
            writeln!(a_writer, "? {location} {num_locations} {all_locations}").unwrap();
            a_writer.flush().unwrap();
            let max_length= a_reader.next::<usize>();
            global_max_length = max(global_max_length, max_length);
            if let Some(locations) = path_lengths.get_mut(&max_length) {
                locations.push(location as u32);
            } else {
                path_lengths.insert(max_length, vec![location as u32]);
            }
        }
        let mut last_location= path_lengths[&global_max_length][0];
        let mut path = last_location.to_string();
        
        let mut current_length = global_max_length - 1;
        while current_length != 0 {
            for location in &path_lengths[&current_length]  {
                writeln!(a_writer, "? {last_location} {} {}", 2, last_location.to_string()+" "+&location.to_string()).unwrap();
                a_writer.flush().unwrap();
                let path_length = a_reader.next::<usize>();
                if path_length == 2 {
                    path.push_str(&(String::from(" ")+&location.to_string()));
                    last_location = *location;
                    break;
                }
            }
            current_length-=1;
        }

        writeln!(a_writer, "! {global_max_length} {path}").unwrap();

    }
}

fn main() {
    let mut my_reader = Scanner::new(std::io::stdin().lock());
    solve(&mut my_reader, &mut std::io::stdout().lock());
}