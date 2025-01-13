use anyhow::*;
use itertools::Itertools;

pub static DAY: u32 = 04;
pub static EXAMPLE_INPUT: &str = "0-999999";

pub fn main(input: &str) -> Result<(usize, usize)> {
    let mut range = input
        .lines()
        .next()
        .context("Invalid input")?
        .split("-")
        .map(|s| s.parse());
    let range_min = range.next().context("Expected minimum value")??;
    let range_max = range.next().context("Expected maximum value")??;
    let mut ans1 = 0;
    let mut ans2 = 0;
    generate_passwords(|password| {
        let number = password.iter().fold(0, |base, x| base * 10 + x);
        if number < range_min || number > range_max {
            return;
        }

        let mut min_group_size = usize::MAX;
        for (_, count) in password.iter().counts() {
            if count > 1 {
                min_group_size = min_group_size.min(count);
            }
        }

        if min_group_size == 2 {
            ans2 += 1;
        } else if min_group_size != usize::MAX {
            ans1 += 1;
        }
    });
    Ok((ans1, ans2))
}

type Password = [u32; 6];

pub fn generate_passwords<F: FnMut(&Password)>(mut callback: F) {
    generate_passwords_impl([0; 6], 0, &mut callback);
}

pub fn generate_passwords_impl<F: FnMut(&Password)>(
    mut password: Password,
    offset: usize,
    callback: &mut F,
) {
    let start = if offset == 0 { 0 } else { password[offset - 1] };
    let is_last_digit = offset + 1 >= password.len();
    for n in start..10 {
        password[offset] = n;
        if is_last_digit {
            callback(&password);
        } else {
            generate_passwords_impl(password, offset + 1, callback);
        }
    }
}
