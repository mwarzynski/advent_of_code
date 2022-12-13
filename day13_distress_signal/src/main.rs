use std::cmp::Ordering;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    One(u8),
    Many(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Packet::Many(vec![]));
        }
        if !s.starts_with("[") {
            return Ok(Packet::One(s.trim().parse().unwrap()));
        }

        fn split_to_str_items(v: &str) -> Vec<&str> {
            let mut items = Vec::new();
            let mut start = 0;
            let mut depth = 0;
            for (i, c) in v.chars().enumerate() {
                if c == ' ' {
                    continue;
                }
                if c == '[' {
                    depth += 1;
                }
                if c == ']' {
                    depth -= 1;
                }
                if c == ',' && depth == 0 {
                    items.push(v[start..i].trim());
                    start = i + 1;
                }
                if i == v.len() - 1 {
                    items.push(v[start..i + 1].trim());
                }
            }
            items
        }

        let ss = s.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
        let vs = split_to_str_items(ss)
            .iter()
            .map(|&v| Packet::from_str(v).unwrap())
            .collect();
        Ok(Packet::Many(vs))
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Packet::Many(left) => match other {
                Packet::Many(right) => left
                    .iter()
                    .take(right.len())
                    .enumerate()
                    .find_map(|(i, left)| match left.cmp(&right[i]) {
                        std::cmp::Ordering::Less => Some(Ordering::Less),
                        std::cmp::Ordering::Greater => Some(Ordering::Greater),
                        std::cmp::Ordering::Equal => None,
                    })
                    .unwrap_or_else(|| left.len().cmp(&right.len())), // if all items equal, check if left is shorter
                Packet::One(right) => self.cmp(&Packet::Many(vec![Packet::One(*right)])),
            },
            Packet::One(left) => match other {
                Packet::One(right) => left.cmp(right),
                Packet::Many(_) => Packet::Many(vec![Packet::One(*left)]).cmp(other),
            },
        }
    }
}

struct Stream {
    packets: Vec<Packet>,
}

impl Stream {
    fn new() -> Self {
        Stream { packets: vec![] }
    }

    fn parse_line(&mut self, line: &str) {
        let packet = Packet::from_str(line).unwrap();
        self.add_packet(packet);
    }

    fn add_packet(&mut self, packet: Packet) {
        self.packets.push(packet);
    }
}

fn main() {
    let mut stream = Stream::new();

    let divider1 = Packet::Many(vec![Packet::Many(vec![Packet::One(2)])]);
    let divider2 = Packet::Many(vec![Packet::Many(vec![Packet::One(6)])]);
    stream.add_packet(divider1.clone());
    stream.add_packet(divider2.clone());

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let line = line_wrapped.unwrap();
        if !line.is_empty() {
            stream.parse_line(&line);
        }
    }

    stream.packets.sort();

    let mut v = 1;
    for (i, packet) in stream.packets.iter().enumerate() {
        if packet.eq(&divider1) {
            v *= i + 1;
        }
        if packet.eq(&divider2) {
            v *= i + 1;
        }
    }

    println!("mul: {}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_1() {
        let packet = Packet::from_str("[[[1, 9], [[1, 3], 0, 5, 8]]]")
            .expect("packet is valid, therefore should be parsed");
        assert_eq!(
            packet,
            Packet::Many(vec![Packet::Many(vec![
                Packet::Many(vec![Packet::One(1), Packet::One(9)]),
                Packet::Many(vec![
                    Packet::Many(vec![Packet::One(1), Packet::One(3)]),
                    Packet::One(0),
                    Packet::One(5),
                    Packet::One(8),
                ]),
            ]),])
        );
    }

    #[test]
    fn parse_2() {
        let packet = Packet::from_str("[[[0,[2,5,2],6,1],[[]]],[3,0,7,[5],10]]")
            .expect("packet is valid, therefore should be parsed");
        assert_eq!(
            packet,
            Packet::Many(vec![
                Packet::Many(vec![
                    Packet::Many(vec![
                        Packet::One(0),
                        Packet::Many(vec![Packet::One(2), Packet::One(5), Packet::One(2)]),
                        Packet::One(6),
                        Packet::One(1),
                    ]),
                    Packet::Many(vec![Packet::Many(vec![])]),
                ]),
                Packet::Many(vec![
                    Packet::One(3),
                    Packet::One(0),
                    Packet::One(7),
                    Packet::Many(vec![Packet::One(5)]),
                    Packet::One(10),
                ]),
            ]),
        );
    }

    #[test]
    fn cmp_1() {
        let packet1 = Packet::from_str("[]").expect("packet is valid, therefore should be parsed");
        let packet2 = Packet::from_str("[3]").expect("packet is valid, therefore should be parsed");
        let result = packet1 < packet2;
        assert_eq!(true, result);
    }

    #[test]
    fn cmp_2() {
        let packet1 =
            Packet::from_str("[0,0,0]").expect("packet is valid, therefore should be parsed");
        let packet2 = Packet::from_str("[2]").expect("packet is valid, therefore should be parsed");
        let result = packet1 < packet2;
        assert_eq!(true, result);
    }
}
