use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ptr::null;

fn main() {
    // File hosts.txt must exist in the current path

    if let Ok(lines) = read_lines("input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut cases: i64 = 0;
        for line in lines.flatten() {
            let line: Vec<_> = line.split_whitespace().collect();
            let mut line_num: Vec<i64> = line.iter().map(|x| x.parse::<i64>().unwrap()).collect();
            let mut order = inc_or_dec(&mut line_num);
            let mut values = difference(&mut line_num, &mut order);
            let mut bad_levels = values.0;
            let mut difference = values.1;
            if order == "increasing" && difference == true { cases += 1; } else if order == "decreasing" && difference == true { cases += 1; } else if difference == false && bad_levels == 1 { if problem_dampener(&mut order, &mut line_num) == true {cases += 1}}
        } ;
        println!("{:?}", cases)
    }
}
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check(line: String) {
    let line: Vec<_> = line.split_whitespace().collect();
    let mut line_num: Vec<i64> = line.iter().map(|x| x.parse::<i64>().unwrap()).collect();
    line_num.sort();
    let mut iter = line_num.iter();
    let mut safe: bool = true;

    for n in line_num.iter() {
        if n < &iter.next().unwrap() {
        } else {
            safe = false;
            break;
        }
    }

    safe;

    println!("{:?}", line_num);
}

fn inc_or_dec(seq: &mut Vec<i64>) -> String {
    let mut safety: bool = true;
    let mut count = 0;
    let mut iterator = seq.iter();
    let mut order: String = String::new();

    if seq[0] <  seq[1] {
        order = "increasing".parse().unwrap();
    } else if seq[0] > seq[1] {
        order = "decreasing".parse().unwrap(); }
    else if seq[0] == seq[1] {
        if seq[1] < seq[2] { order = "increasing".parse().unwrap(); }
        else if seq[1] > seq[2] { order = "decreasing".parse().unwrap(); }
    }

order }


fn difference(line_num: &mut Vec<i64>, order: &mut String) -> (i64,bool) {
    let mut seq = line_num.clone();
    //seq.sort();
    let pos_range = 1..3;
    let neg_range = -3..-1;
    let mut safe: bool = true;


    //write match case to match between increasing and decreasing order
    let mut iter = seq.iter();
    let mut next_num = iter.next().unwrap();
    let mut bad_levels: i64 = 0;




if order == "increasing" {
    for (i, n) in seq.iter().enumerate().skip(1) {
        let mut diff = n - seq[i - 1] as i64;
        match diff {
            1 => continue,
            2 => continue,
            3 => continue,
            _ => {safe = false; bad_levels += 1; continue;},
        }
    }
}  else if order == "decreasing" {
    for (i, n) in seq.iter().enumerate().skip(1) {
        let mut diff = n - seq[i - 1];
        match diff {
            -1 => continue,
            -2 => continue,
            -3 => continue,
            _ => {safe = false; bad_levels += 1; continue;},
        }
    }

    }
    (bad_levels, safe)}

fn problem_dampener(order: &mut String, line_num: &mut Vec<i64>) -> bool {
//create 3 extra vecs to remove the value from and check
    let mut seq = line_num.clone();
    let mut test1 = line_num.clone();
    let mut test2 = line_num.clone();
    let mut test3 = line_num.clone();


    let mut safe = false;
    //let val = ();


        if order == "decreasing" {
            for (i, n) in line_num.iter().enumerate().zip(line_num.iter().skip(1)) {
            let mut diff = i.1 - n;
            match diff {
                1 => continue,
                2 => continue,
                3 => continue,
                _ => {

                    &test1.remove(i.0);
                    &test2.remove(i.0 + 1);
                    if i.0 == 0 {
                        //pass
                    }  else { &test3.remove(i.0 - 1); }

                    if difference(&mut test1, order).1 {
                        safe = true;
                    } else if difference(&mut test2, order).1 {
                        safe = true;
                    } else if difference(&mut test3, order).1 {
                        safe = true;
                    };
                },
            }
        }
    } else if order == "increasing" {
            for (i, n) in line_num.iter().enumerate().zip(line_num.iter().skip(1)) {
                let mut diff = i.1 - n;
                match diff {
                    -1 => continue,
                    -2 => continue,
                    -3 => continue,
                    _ => {

                        &test1.remove(i.0);
                        &test2.remove(i.0 + 1);
                        if i.0 == 0 {
                            //pass
                        }  else { &test3.remove(i.0 - 1); }

                        if difference(&mut test1, order).1 {
                            safe = true;
                        } else if difference(&mut test2, order).1 {
                            safe = true;
                        } else if difference(&mut test3, order).1 {
                            safe = true;
                        };
                    },
                }
            }
        }


safe}
