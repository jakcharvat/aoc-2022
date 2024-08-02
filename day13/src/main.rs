use std::{
    fmt::Display,
    io::{stdin, Read},
};

use parser::{parse_packet, parse_packet_pairs, Packet};

mod parser;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (_, packet_pairs) = parse_packet_pairs(&input).unwrap();
    println!(
        "Part 1: {}",
        packet_pairs
            .iter()
            .enumerate()
            .filter(|(_, (l, r))| l < r)
            .map(|(i, _)| i + 1)
            .sum::<usize>()
    );

    let mut packets: Vec<_> = packet_pairs
        .into_iter()
        .map(|(l, r)| [l, r])
        .flatten()
        .collect();

    let dividers = [
        parse_packet("[[2]]").unwrap().1,
        parse_packet("[[6]]").unwrap().1,
    ];

    packets.push(dividers[0].clone());
    packets.push(dividers[1].clone());
    packets.sort();

    let packets = packets;
    for packet in packets.iter() {
        eprintln!("{}", packet);
    }

    let divider_indices = dividers.map(|d| packets.iter().position(|p| p == &d).unwrap() + 1);
    println!("Part 2: {}", divider_indices.iter().product::<usize>())
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(ll), Packet::List(rl)) => ll.cmp(rl),
            (lp @ Packet::List(_), rp @ Packet::Atom(_)) => lp.cmp(&Packet::List(vec![rp.clone()])),
            (lp @ Packet::Atom(_), rp @ Packet::List(_)) => Packet::List(vec![lp.clone()]).cmp(rp),
            (Packet::Atom(l), Packet::Atom(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(l) => {
                write!(f, "[")?;
                for (idx, p) in l.iter().enumerate() {
                    if idx != 0 {
                        write!(f, ",")?;
                    }

                    write!(f, "{}", p)?;
                }
                write!(f, "]")?;
            }
            Packet::Atom(v) => write!(f, "{}", v)?,
        }

        Ok(())
    }
}
