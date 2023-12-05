use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Number {
    row: usize,
    column_start: usize,
    column_end: usize,
}

#[derive(Debug)]
struct Gear {
    row: usize,
    column: usize,
}

fn schematic_row_find_numbers(row_i: usize, line: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut number_start: Option<usize> = None;
    for (j, c) in line.chars().enumerate() {
        let c_is_digit = c.is_digit(10);
        if c_is_digit {
            if number_start == None {
                number_start = Some(j);
            }
        }
        let c_is_last_char = j == line.len() - 1;
        if c_is_last_char || !c_is_digit {
            if let Some(ns) = number_start {
                let mut ne = j;
                if c_is_digit {
                    ne += 1;
                }
                numbers.push(Number {
                    row: row_i,
                    column_start: ns,
                    column_end: ne,
                });
            }
            number_start = None;
        }
    }
    numbers
}

fn schematic_row_find_gears(row_i: usize, line: &str) -> Vec<Gear> {
    let mut gears: Vec<Gear> = Vec::new();
    for (j, c) in line.chars().enumerate() {
        if c == '*' {
            gears.push(Gear {
                row: row_i,
                column: j,
            });
        }
    }
    gears
}

fn part1_schematic_number_is_part(lines: &Vec<String>, number: &Number) -> bool {
    let str_contains_symbol = |value: &str| -> bool {
        let symbol = value
            .chars()
            .filter(|v| !v.is_digit(10) && *v != '.')
            .take(1)
            .into_iter()
            .next();
        match symbol {
            Some(_) => true,
            None => false,
        }
    };
    let symbol_diagonal_exists = |i: usize, j: usize| -> bool {
        let mut lines_i: Vec<usize> = vec![i];
        if let Some(v) = i.checked_sub(1) {
            lines_i.push(v);
        }
        if i < lines.len() - 1 {
            if let Some(v) = i.checked_add(1) {
                lines_i.push(v);
            }
        }
        for ii in lines_i {
            let mut jl: usize = j;
            if let Some(v) = j.checked_sub(1) {
                jl = v;
            }
            let mut jr: usize = j;
            if let Some(v) = j.checked_add(2) {
                jr = v;
            }
            if let Some(v) = lines[ii as usize].get(jl..jr) {
                if str_contains_symbol(v) {
                    return true;
                }
            }
        }
        return false;
    };

    for column in number.column_start..number.column_end {
        if symbol_diagonal_exists(number.row, column) {
            return true;
        }
    }

    return false;
}

fn schematic_get_number(lines: &Vec<String>, number: &Number) -> u32 {
    if let Some(line) = lines.get(number.row) {
        match line.get(number.column_start..number.column_end) {
            Some(number_raw) => {
                let value: u32 = number_raw.parse().unwrap_or(0);
                return value;
            }
            None => return 0,
        }
    }
    return 0;
}

fn part2_schematic_gear_find_adjacent_numbers<'a>(
    numbers: &'a Vec<Number>,
    gear: &Gear,
) -> Vec<&'a Number> {
    return numbers
        .iter()
        .filter(|num| {
            let diff = num.row as i32 - gear.row as i32;
            return -1 <= diff && diff <= 1;
        })
        .filter(|num| {
            let a = num.column_start as i32 - 1;
            let b = num.column_end as i32;
            let x = gear.column as i32;
            return a <= x && x <= b;
        })
        .collect();
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input.prod").expect("input file should exist");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut numbers: Vec<Number> = Vec::new();

    for (row_i, row) in lines.iter().enumerate() {
        numbers.extend(schematic_row_find_numbers(row_i, row));
    }

    let part_numbers: Vec<&Number> = numbers
        .iter()
        .filter(|number| part1_schematic_number_is_part(&lines, number))
        .collect::<Vec<&Number>>();
    let part1_sum: u32 = part_numbers
        .iter()
        .map(|number| schematic_get_number(&lines, number))
        .sum();

    let mut gears: Vec<Gear> = Vec::new();
    for (row_i, row) in lines.iter().enumerate() {
        gears.extend(schematic_row_find_gears(row_i, row));
    }
    let part2_sum = gears
        .iter()
        .map(|g| part2_schematic_gear_find_adjacent_numbers(&numbers, g))
        .filter(|ns| ns.len() == 2)
        .fold(0, |a, b| {
            a + schematic_get_number(&lines, &b[0]) * schematic_get_number(&lines, &b[1])
        });

    println!("{}", part1_sum);
    println!("{}", part2_sum);

    Ok(())
}

