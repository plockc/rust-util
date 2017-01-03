extern crate yaml_rust;

use std::collections::btree_map::BTreeMap;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use yaml_rust::{Yaml, YamlEmitter};

fn insert_yaml(map: &mut BTreeMap<Yaml, Yaml>, k:&str, v:Yaml) {
    map.insert(Yaml::String(k.to_string()), v); 
}

// TODO: most occurrence, least
// freq table with value: {% of count, and min/max % positionally}
fn main() {
    let reader = BufReader::new(io::stdin());
    let mut freqs = BTreeMap::new();
    let mut val_total:i64 = 0;
    let mut total_cnt:i32 = 0;
    for line in reader.lines() {
        let val = line.unwrap().parse::<i64>().unwrap();
        // or_insert returns a mutable reference, so deref to increment
        *freqs.entry(val).or_insert(0) += 1;
        val_total += val;
        total_cnt += 1;
    }

    let freq_len:i32 = freqs.len() as i32;
    let last_freq = freqs.len() - 1;

    // load statistics into yaml structure
    let mut stats = BTreeMap::new();

    let percentiles = [1, 2, 3, 4, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50,
                       55, 60, 65, 70, 75, 80, 85, 90, 95, 96, 97, 98, 99];
    let percentile_indexes = percentiles.iter().map(|&x| -> i32 {
        let pos = x as f32*total_cnt as f32/100.0;
        if pos == (x*total_cnt/100) as f32 {
            return pos as i32 - 1;
        } else {
            return pos as i32;
        }}).collect::<Vec<i32>>();
    println!("indexes: {:?}", percentile_indexes);

    let mut percentile_values = BTreeMap::new();
    let mut next_percentile_index = 0;
    let mut cur_cnt = 0;
    // for loop over referenced collection (borrowing the collection)
    for (i, (&val, cnt)) in freqs.iter().enumerate() {
        // TODO: this if/else should be done outside the loop
        if i == 0 {
            insert_yaml(&mut stats, "min", Yaml::Integer(val));
        } else if i == last_freq {
            insert_yaml(&mut stats, "max", Yaml::Integer(val));
        }
        cur_cnt = cur_cnt + cnt;
        while next_percentile_index < percentile_indexes.len() && 
                cur_cnt >= percentile_indexes[next_percentile_index] {
            percentile_values.insert(
                Yaml::Integer(percentiles[next_percentile_index] as i64),
                Yaml::Integer(val)); 
            next_percentile_index += 1;
        }
    }
    println!("Freqs: {:?}", freqs);

    insert_yaml(&mut stats, "percentiles", Yaml::Hash(percentile_values));
    insert_yaml(&mut stats, "cardinality",
                Yaml::Integer(freq_len as i64));
    insert_yaml(&mut stats, "count", Yaml::Integer(total_cnt as i64));
    //insert_yaml(&mut stats, "counts", Yaml::Hash(freqs));
    // add statistics
    if freq_len > 0 {
        insert_yaml(&mut stats, "average", Yaml::Real((val_total as f64/total_cnt as f64).to_string()));
        insert_yaml(&mut stats, "sum", Yaml::Integer(val_total));
    }

    // write out YAML
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&Yaml::Hash(stats)).unwrap();
    }
    println!("{}", out_str);
}
