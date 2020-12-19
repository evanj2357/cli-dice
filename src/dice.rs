use nom::{
    char,
    character::complete::{digit0, digit1},
    complete,
    error::Error,
    map, map_res, opt, parse_to, terminated, verify, Finish, IResult,
};
use rusty_dice::*;
use std::str::FromStr;

pub struct Dice {
    die: RangeDie<i32>,
    count: usize,
    modifier: i32,
}

impl Dice {
    pub fn new(count: usize, max_die_val: i32, modifier: i32) -> Self {
        Dice {
            die: RangeDie::new(1, max_die_val),
            count: count,
            modifier: modifier,
        }
    }

    pub fn roll(&self) -> i32 {
        n_rolls(self.count, &self.die).iter().sum::<i32>() + self.modifier
    }

    // TODO: implement "roll at advantage" feature to use this
    #[allow(dead_code)]
    pub fn roll_n_take_highest(&self, n: usize) -> i32 {
        (0..n)
            .map(|_| *n_rolls(self.count, &self.die).iter().max().unwrap_or(&0) + self.modifier)
            .max()
            .unwrap_or(0)
    }

    // TODO: implement "roll at disadvantage" feature to use this
    #[allow(dead_code)]
    pub fn roll_n_take_lowest(&self, n: usize) -> i32 {
        (0..n)
            .map(|_| *n_rolls(self.count, &self.die).iter().min().unwrap_or(&0) + self.modifier)
            .min()
            .unwrap_or(0)
    }
}

impl FromStr for Dice {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        complete!(s, parse_dice_spec)
            .finish()
            .map(|(_, (n_dice, max_die_val, modifier))| Dice::new(n_dice, max_die_val, modifier))
            .map_err(|e| Error {
                input: e.input.to_string(),
                code: e.code,
            })
    }
}

fn parse_dice_spec(s: &str) -> IResult<&str, (usize, i32, i32)> {
    let (i1, count) = terminated!(s, opt!(map_res!(digit0, |s1: &str| s1.parse())), char!('d'))?;
    let (i2, max_die_val) = verify!(i1, map!(digit1, |s| s.parse().unwrap()), |&v| v > 0)?;
    let (i3, modifier) = opt!(i2, parse_to!(i32))?;

    let n_dice = if let Some(n) = count {
        if n < 1 {
            1
        } else {
            n
        }
    } else {
        1
    };
    Ok((i3, (n_dice, max_die_val, modifier.unwrap_or(0))))
}
