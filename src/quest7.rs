use itertools::Itertools;

pub fn solve_part_1(input: &str) -> String {
    let mut devices: Vec<_> = input.lines().map(|line| {
        let (name, instructions) = line.split_at(line.find(":").unwrap());
        let instructions: Vec<_> = instructions[1..].split(",").collect();
        let mut power = 10;
        let mut total = 0;
        for step in 0..10 {
            match instructions[step % instructions.len()] {
                "+" => { power += 1; },
                "-" => { power -= 1; },
                "=" => {},
                i => panic!("unexpected instruction: {}", i),
            }
            total += power;
        }
        (name, total)
    }).collect();
    devices.sort_by_key(|d| -d.1);
    devices.iter().map(|d| d.0.chars().next().unwrap()).join("")
}

pub fn solve_part_2(input: &str) -> String {
    let track = "-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=---=++==--+++==++=+=--==++==+++=++=+++=--=+=-=+=-+=-+=-+-=+=-=+=-+++=+==++++==---=+=+=-S";
    solve_part_2_with_track(input, track)
}
    
fn solve_part_2_with_track(input: &str, track: &str) -> String {
    let mut devices: Vec<_> = input.lines().map(|line| {
        let (name, instructions) = line.split_at(line.find(":").unwrap());
        let instructions: Vec<_> = instructions[1..].split(",").collect();
        let mut power = 10;
        let mut total = 0;
        for step in 0..(track.len()*10) {
            let track_pos = step % track.len();
            match (&track[track_pos..track_pos+1], instructions[step % instructions.len()]) {
                ("+", _) | ("=", "+") | ("S", "+") => { power += 1; },
                ("-", _) | ("=", "-") | ("S", "-") => { power -= 1; },
                _ => {},
            }
            total += power;
        }
        (name, total)
    }).collect();
    devices.sort_by_key(|d| -d.1);
    devices.iter().map(|d| d.0.chars().next().unwrap()).join("")
}

fn flatten_track(input: &str) -> String {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut x = 1;
    let mut y = 0;
    let mut result = String::new();
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dir = 0;
    while chars[y][x] != 'S' {
        result.push(chars[y][x]);
        for possible_dir in 0..4 {
            if dir != possible_dir && (dir % 2) == (possible_dir % 2) {
                continue;
            }
            let (nx, ny) = (x as isize + directions[possible_dir].0, y as isize + directions[possible_dir].1);
            if nx < 0 || ny < 0 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if ny >= chars.len() || nx >= chars[ny].len() {
                continue;
            }
            if chars[ny][nx] == ' ' {
                continue;
            }
            (x, y) = (nx, ny);
            dir = possible_dir;
            break;
        }
    }
    result.push('S');
    result
}

fn run_n_loops(n: usize, instructions: &[char], track: &str) -> i64 {
    let mut power = 10;
    let mut total = 0;
    for step in 0..(track.len()*n) {
        let track_pos = step % track.len();
        match (&track[track_pos..track_pos+1], instructions[step % instructions.len()]) {
            ("+", _) | ("=", '+') | ("S", '+') => { power += 1; },
            ("-", _) | ("=", '-') | ("S", '-') => { power -= 1; },
            _ => {},
        }
        if power < 0 { power = 0; }
        total += power;
    }
    total
}

fn count_winning_strategies(track: &str, strategy_prefix: &mut Vec<char>, threshold: i64, remaining_plus: i8, remaining_minus: i8, remaining_equals: i8) -> i64 {
    if strategy_prefix.len() >= 11 {
        return if run_n_loops(2024, &strategy_prefix, track) > threshold { 1 } else { 0 }
    }
    let mut result = 0;
    if remaining_plus > 0 {
        strategy_prefix.push('+');
        result += count_winning_strategies(track, strategy_prefix, threshold, remaining_plus - 1, remaining_minus, remaining_equals);
        strategy_prefix.pop();
    }
    if remaining_equals > 0 {
        strategy_prefix.push('=');
        result += count_winning_strategies(track, strategy_prefix, threshold, remaining_plus, remaining_minus, remaining_equals - 1);
        strategy_prefix.pop();
    }
    if remaining_minus > 0 {
        strategy_prefix.push('-');
        result += count_winning_strategies(track, strategy_prefix, threshold, remaining_plus, remaining_minus - 1, remaining_equals);
        strategy_prefix.pop();
    }
    result
}

pub fn solve_part_3(input: &str) -> String {
    let track = "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-";
    let track = flatten_track(&track);
    let enemy_strategy: Vec<char> = input[2..].split(",").map(|s| s.chars().next().unwrap()).collect();
    let enemy_result = run_n_loops(2024, &enemy_strategy.as_slice(), track.as_str());
    let mut strategy_buffer = Vec::new();
    count_winning_strategies(track.as_str(), &mut strategy_buffer, enemy_result, 5, 3, 3).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_log::test;

    #[test]
    fn test_solve_part_1() {
        assert_eq!("BDCA", solve_part_1("A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+"));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!("DCBA", solve_part_2_with_track("A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+", "+===++-=+=-S"));
    }

    #[test]
    fn test_flatten_track() {
        assert_eq!("+=+++===-+++++=-==+--+=+===-++=====+--===++=-==+=++====-==-===+=+=--==++=+========-=======++--+++=-++=-+=+==-=++=--+=-====++--+=-==++======+=++=-+==+=-==++=-=-=---++=-=++==++===--==+===++===---+++==++=+=-=====+==++===--==-==+++==+++=++=+===--==++--===+=====-=++====-+=-+--=+++=-+-===++====+++--=++====+=-=+===+=====-+++=+==++++==----=+=+=-S", flatten_track("S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-"));
    }

    #[test]
    fn test_solve_part_3() {
        assert_eq!("?", solve_part_3("A:+,-,=,=
B:+,=,-,+
C:=,-,+,+
D:=,=,=,+"));
    }
}
