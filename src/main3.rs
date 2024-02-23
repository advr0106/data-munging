use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn diff(a: f64, b: f64) -> f64 {
    (a - b).abs()
}
fn proccess_file(path: &str,skip: usize, n1: usize, n2: usize, n3: usize) -> Option<String>{
    let process = BufReader::new(File::open(path).unwrap())
    .lines()
    .skip(skip)
    .filter_map(|line| {
        let line = line.unwrap();
        let mut parts = line.split_whitespace().filter(|s| !s.is_empty() && !s.contains('-'));
        let data1 = parts.nth(n1)?.to_string();
        let data2 = parts.nth(n2)?.parse::<f64>().ok()?;
        let data3 = parts.nth(n3)?.parse::<f64>().ok()?;
        Some((data1, diff(data2, data3)))
    })
    .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    .map(|(data1, _)| data1);
    return process;
}

fn main() {

    match proccess_file("C:\\Users\\alexv\\OneDrive - INTEC\\Asignaturas\\Trimestre 8\\Tendencia\\tareas\\data-munging\\src\\weather.dat", 2, 0, 0, 0) {
        Some(day) => println!("Día con la menor diferencia de temperatura: {}", day),
        None => println!("No se encontraron datos válidos."),
    }

    match proccess_file("C:\\Users\\alexv\\OneDrive - INTEC\\Asignaturas\\Trimestre 8\\Tendencia\\tareas\\data-munging\\src\\football.dat", 1, 1, 4, 0) {
        Some(team) => println!("Equipo con la menor diferencia de goles: {}", team),
        None => println!("No valid data found."),
    }
}
