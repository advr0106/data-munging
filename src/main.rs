use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let smallest_spread_day = BufReader::new(File::open("C:\\Users\\alexv\\OneDrive - INTEC\\Asignaturas\\Trimestre 8\\Tendencia\\tareas\\data-munging\\src\\weather.dat").unwrap())
        .lines()
        .skip(2)
        .filter_map(|line| {
            let line = line.unwrap();
            let mut parts = line.split_whitespace().filter(|&s| !s.is_empty());
            let day = parts.next()?.parse::<usize>().ok()?;
            let max_temp = parts.nth(0)?.parse::<f64>().ok()?;
            let min_temp = parts.nth(0)?.parse::<f64>().ok()?;
            Some((day, max_temp - min_temp))
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(day, _)| day);

    match smallest_spread_day {
        Some(day) => println!("Day with smallest temperature spread: {}", day),
        None => println!("No valid data found."),
    }
}