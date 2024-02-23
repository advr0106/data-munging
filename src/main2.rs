use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    // Análisis del archivo "football.dat"
    let team_with_smallest_goal_diff = BufReader::new(File::open("C:\\Users\\alexv\\OneDrive - INTEC\\Asignaturas\\Trimestre 8\\Tendencia\\tareas\\data-munging\\src\\football.dat").unwrap())
        .lines()
        .skip(1) // Skip the header
        .filter_map(|line| {
            let line = line.unwrap();
            let mut parts = line.split_whitespace().filter(|s| !s.is_empty() && !s.contains('-'));
            let team = parts.nth(1)?.trim_end_matches('.').to_string();
            let goals_for = parts.nth(4)?.parse::<f64>().ok()?;
            let goals_against = parts.next()?.parse::<f64>().ok()?;
            Some((team, (goals_for - goals_against).abs()))
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(team, _)| team);

        match team_with_smallest_goal_diff {
            Some(team) => println!("Equipo con la menor diferencia de goles: {}", team),
            None => println!("No valid data found."),
        }
}