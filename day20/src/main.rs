use std::fmt::Display;

#[allow(unused_imports)]
use dbg_pls::pretty;

const DECRYPTION_KEY: isize = 811589153;

struct Nums {
    nums: Vec<isize>,
    fwd: Vec<usize>,
    bck: Vec<usize>,
}

impl Nums {
    fn new(input: &str) -> Nums {
        let nums: Vec<_> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();
        let indices: Vec<_> = (0..nums.len()).collect();

        Nums {
            nums,
            fwd: indices.clone(),
            bck: indices,
        }
    }

    fn moved(&self, idx: usize, offset: isize) -> usize {
        (idx as isize + offset - 1).rem_euclid(self.nums.len() as isize - 1) as usize + 1
    }

    fn mix_element(&mut self, idx: usize) {
        let num = self.nums[idx];
        let original_idx = self.fwd[idx];
        let new_idx = self.moved(original_idx, num);

        if new_idx == original_idx {
            return;
        }

        let popped_idx = self.bck.remove(original_idx);
        assert_eq!(popped_idx, idx);
        self.bck.insert(new_idx, idx);

        self.fwd[idx] = new_idx;
        if original_idx < new_idx {
            for &i in self.bck[original_idx..new_idx].iter() {
                assert_ne!(i, idx);
                self.fwd[i] -= 1;
            }
        } else if original_idx > new_idx {
            for &i in self.bck[(new_idx + 1)..(original_idx + 1)].iter() {
                assert_ne!(i, idx);
                self.fwd[i] += 1;
            }
        } else {
            unreachable!()
        }
    }

    fn mix(&mut self) {
        for idx in 0..self.nums.len() {
            self.mix_element(idx);
        }
    }

    #[allow(dead_code)]
    fn check_integrity(&self) {
        for i in 0..self.nums.len() {
            assert_eq!(
                self.bck[self.fwd[i]], i,
                "self.bck[self.fwd[i = {}] = {}] = {} != {}",
                i, self.fwd[i], self.bck[self.fwd[i]], i
            )
        }
    }

    fn get(&self, idx: usize) -> isize {
        self.nums[self.bck[idx % self.nums.len()]]
    }

    fn calc_coord(&self) -> isize {
        let original_zero_idx = self.nums.iter().position(|&n| n == 0).unwrap();
        let actual_zero_idx = self.fwd[original_zero_idx];

        self.get(actual_zero_idx + 1000)
            + self.get(actual_zero_idx + 2000)
            + self.get(actual_zero_idx + 3000)
    }
}

impl Display for Nums {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nums: Vec<_> = self.bck.iter().map(|&i| self.nums[i]).collect();
        write!(f, "{}", pretty(&nums))?;

        Ok(())
    }
}

fn part1(input: &str) -> isize {
    let mut nums = Nums::new(input);
    nums.mix();
    nums.calc_coord()
}

fn part2(input: &str) -> isize {
    let mut nums = Nums::new(input);
    for num in nums.nums.iter_mut() {
        *num *= DECRYPTION_KEY;
    }

    for _ in 0..10 {
        nums.mix();
    }

    nums.calc_coord()
}

fn main() {
    let input = include_str!("../in.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../small-in.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 3);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 1623178306);
    }
}
