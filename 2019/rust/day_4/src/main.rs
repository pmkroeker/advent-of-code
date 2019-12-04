// THANKS TO https://github.com/alyti/aoc-2019
// Still learning rust and this was helpful!

fn main() {
    let input_string = include_str!("input.txt");
    let input: Vec<u64> = input_string.trim()
        .split('-')
        .map(|value: &str| {
            value.parse::<u64>().unwrap()
        })
        .collect();
    let items = (input[0], input[1]);

    println!("{}", part_1(items));
    println!("{}", part_2(items));
}

fn has_adjacent(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).any(|(c1, c2)| c1 == c2)
}

fn increased_or_same(s: &str) -> bool {
    s.chars().zip(s.chars().skip(1)).all(|(c1, c2)| c1 <= c2)
}

fn part_1(input: (u64, u64)) -> usize {
    let (from, to) = input;
    (from..to)
        .map(|n| format!("{}", n))
        .filter(|s| has_adjacent(&s))
        .filter(|s| increased_or_same(&s))
        .count()
}

fn part2_adjacents(s: &str) -> bool {
    let s = s.as_bytes();

    (0..s.len() - 3).any(|i| s[i + 1] == s[i + 2] && s[i] != s[i + 1] && s[i + 2] != s[i + 3])
        || (s[0] == s[1] && s[1] != s[2])
        || (s[s.len() - 1] == s[s.len() - 2] && s[s.len() - 2] != s[s.len() - 3])
}

fn part_2(input: (u64, u64)) -> usize {
    let (from, to) = input;
    (from..to)
        .map(|n| format!("{}", n))
        .filter(|s| part2_adjacents(&s))
        .filter(|s| increased_or_same(&s))
        .count()
}

// Below 2 functions are initial attempt, giving incorrect answer
/*
fn check_legal_password(password: u64) -> bool {
    let pass_str = password.to_string();
    let pass_chars: Vec<char> = pass_str.chars().collect();
    let mut found_duplicate = false;
    for (index, cur_char) in pass_chars.iter().enumerate() {
        if index >= 5 {
            break;
        }
        let next_idx = index + 1;
        let next_char = &pass_chars[next_idx];
        if cur_char == next_char {
            found_duplicate = !check_in_greater_group(*cur_char, index, &pass_chars);
        } else if cur_char > next_char {
            return false;
        }

    }
    found_duplicate
}

// for part 2
fn check_in_greater_group(current: char, index: usize, code: &[char]) -> bool {
    // Need to check next 2, previous 2, and one on each side
    let past_1 = index - 1;
    let past_2 = index - 2;
    let next_1 = index + 1;
    let next_2 = index + 2;

    // first character, check next 2
    if index == 0 {
        let next_1_char = code[next_1];
        let next_2_char = code[next_2];
        if current == next_1_char && current == next_2_char {
            return true;
        }
    } else if index == 5 {
        // last character check 2 past
        let past_1_char = code[past_1];
        let past_2_char = code[past_2];
        if current == past_1_char && current == past_2_char {
            return true;
        }
    } else {
        // check all 3 potential
    }
    false
}
*/
