use std::io::stdin;

use cpu::Cpu;
use parser::parse_command;

mod cpu;
mod parser;

fn main() {
    let lines = stdin().lines().map(|l| l.unwrap());
    let mut sum = 0i64;
    let mut crt = [['.'; 40]; 6];

    let mut cpu = Cpu::new(|cycle, x| {
        if (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }

        let crt_cycle = cycle - 1;
        let (crt_y, crt_x) = (crt_cycle / 40, crt_cycle % 40);
        if (crt_x - x).abs() <= 1 {
            crt[crt_y as usize][crt_x as usize] = '█';
        }
    });

    for line in lines {
        let (_, cmd) = parse_command(&line).unwrap();
        cpu.exec_command(cmd)
    }

    drop(cpu);

    println!("Part 1: {}", sum);
    println!(
        "Part 2:\n{}",
        crt.map(|l| l.iter().collect::<String>()).join("\n")
    );
}
