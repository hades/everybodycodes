use std::{
    collections::{HashMap, HashSet},
    iter::{repeat, repeat_with},
};

use itertools::{Itertools, izip};
use log::debug;
use regex::Regex;

pub fn solve_part_one(input: &str) -> String {
    let re = Regex::new(r"(\d+): faces=\[([^\]]+)\] seed=(\d+)").unwrap();
    let mut faces: Vec<Vec<_>> = vec![];
    let mut seed = vec![];
    for (_, [_, faces_str, seed_str]) in re.captures_iter(input).map(|c| c.extract()) {
        faces.push(
            faces_str
                .split(',')
                .map(|f| f.parse::<i64>().unwrap())
                .collect(),
        );
        seed.push(seed_str.parse::<i64>().unwrap());
    }
    let mut pulse = seed.clone();
    let mut face = Vec::from_iter(repeat(0usize).take(faces.len()));
    let mut total_points = 0;
    let mut roll_count = 0;
    while total_points < 10000 {
        roll_count += 1;
        for (faces, seed, current_pulse, current_face) in
            izip!(faces.iter(), seed.iter(), pulse.iter_mut(), face.iter_mut())
        {
            let spin = roll_count * *current_pulse;
            *current_face = (current_face.wrapping_add_signed(spin as isize)) % faces.len();
            total_points += faces[*current_face];
            *current_pulse = (*current_pulse + spin) % *seed + 1 + roll_count + *seed;
        }
    }
    roll_count.to_string()
}

pub fn solve_part_two(input: &str) -> String {
    let re = Regex::new(r"(\d+): faces=\[([^\]]+)\] seed=(\d+)").unwrap();
    let (dice, racetrack) = input.split_once("\n\n").unwrap();
    let mut faces: Vec<Vec<_>> = vec![];
    let mut seed = vec![];
    for (_, [_, faces_str, seed_str]) in re.captures_iter(dice).map(|c| c.extract()) {
        faces.push(
            faces_str
                .split(',')
                .map(|f| f.parse::<i64>().unwrap())
                .collect(),
        );
        seed.push(seed_str.parse::<i64>().unwrap());
    }
    let racetrack: Vec<_> = racetrack.chars().map(|ch| ch as i64 - '0' as i64).collect();
    let mut pulse = seed.clone();
    let mut face = Vec::from_iter(repeat(0usize).take(faces.len()));
    let mut roll_count = 0;
    let mut finishing_order = vec![];
    let mut player_positions = Vec::from_iter(repeat(0usize).take(faces.len()));
    while finishing_order.len() < faces.len() {
        roll_count += 1;
        for (player_idx, (faces, seed, current_pulse, current_face, player_position)) in izip!(
            faces.iter(),
            seed.iter(),
            pulse.iter_mut(),
            face.iter_mut(),
            player_positions.iter_mut()
        )
        .enumerate()
        {
            let spin = roll_count * *current_pulse;
            *current_face = (current_face.wrapping_add_signed(spin as isize)) % faces.len();
            if *player_position < racetrack.len() {
                if faces[*current_face] == racetrack[*player_position] {
                    *player_position += 1;
                    if *player_position == racetrack.len() {
                        finishing_order.push(player_idx + 1);
                    }
                }
            }
            *current_pulse = (*current_pulse + spin) % *seed + 1 + roll_count + *seed;
        }
    }
    finishing_order.into_iter().map(|p| p.to_string()).join(",")
}

pub fn solve_part_three(input: &str) -> String {
    let re = Regex::new(r"(\d+): faces=\[([^\]]+)\] seed=(\d+)").unwrap();
    let (dice, field_str) = input.split_once("\n\n").unwrap();
    let mut faces: Vec<Vec<_>> = vec![];
    let mut seed = vec![];
    for (_, [_, faces_str, seed_str]) in re.captures_iter(dice).map(|c| c.extract()) {
        faces.push(
            faces_str
                .split(',')
                .map(|f| f.parse::<i64>().unwrap())
                .collect(),
        );
        seed.push(seed_str.parse::<i64>().unwrap());
    }
    let mut field = HashMap::new();
    for (i, line) in field_str.lines().enumerate() {
        for (j, label) in line.chars().enumerate() {
            field
                .entry(label as i64 - '0' as i64)
                .or_insert(Vec::new())
                .push((i, j));
        }
    }
    let mut pulse = seed.clone();
    let mut face = Vec::from_iter(repeat(0usize).take(faces.len()));
    let mut roll_count = 0;
    let mut is_first_iteration = true;
    let mut front = Vec::from_iter(repeat_with(|| HashSet::new()).take(faces.len()));
    let mut accessible_spaces = HashSet::new();
    while is_first_iteration || front.iter().map(|f| f.len()).sum::<usize>() > 0 {
        let mut next_front = Vec::from_iter(repeat_with(|| HashSet::new()).take(faces.len()));
        roll_count += 1;
        for (player_idx, (faces, seed, current_pulse, current_face)) in
            izip!(faces.iter(), seed.iter(), pulse.iter_mut(), face.iter_mut()).enumerate()
        {
            let spin = roll_count * *current_pulse;
            *current_face = (current_face.wrapping_add_signed(spin as isize)) % faces.len();
            field[&faces[*current_face]]
                .iter()
                .filter(|&position| is_first_iteration || front[player_idx].contains(position))
                .for_each(|&(i, j)| {
                    accessible_spaces.insert((i, j));
                    for (di, dj) in [(0, 0), (-1, 0), (0, -1), (1, 0), (0, 1)] {
                        next_front[player_idx]
                            .insert((i.wrapping_add_signed(di), j.wrapping_add_signed(dj)));
                    }
                });
            *current_pulse = (*current_pulse + spin) % *seed + 1 + roll_count + *seed;
        }
        front = next_front;
        is_first_iteration = false;
    }
    accessible_spaces.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_part_one() {
        assert_eq!(
            "844",
            solve_part_one(
                "1: faces=[1,2,3,4,5,6] seed=7
2: faces=[-1,1,-1,1,-1] seed=13
3: faces=[9,8,7,8,9] seed=17"
            )
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            "1,3,4,2",
            solve_part_two(
                "1: faces=[1,2,3,4,5,6,7,8,9] seed=13
2: faces=[1,2,3,4,5,6,7,8,9] seed=29
3: faces=[1,2,3,4,5,6,7,8,9] seed=37
4: faces=[1,2,3,4,5,6,7,8,9] seed=43

51257284"
            )
        );
    }

    #[test]
    fn test_part_three() {
        assert_eq!(
            "33",
            solve_part_three(
                "1: faces=[1,2,3,4,5,6,7,8,9] seed=13

1523758297
4822941583
7627997892
4397697132
1799773472"
            )
        );
        assert_eq!(
            "1125",
            solve_part_three(
                "1: faces=[1,2,3,4,5,6,7,8,9] seed=339211
2: faces=[1,2,3,4,5,6,7,8,9] seed=339517
3: faces=[1,2,3,4,5,6,7,8,9] seed=339769
4: faces=[1,2,3,4,5,6,7,8,9] seed=339049
5: faces=[1,2,3,4,5,6,7,8,9] seed=338959
6: faces=[1,2,3,4,5,6,7,8,9] seed=340111
7: faces=[1,2,3,4,5,6,7,8,9] seed=339679
8: faces=[1,2,3,4,5,6,7,8,9] seed=339121
9: faces=[1,2,3,4,5,6,7,8,9] seed=338851

94129478611916584144567479397512595367821487689499329543245932151
45326719759656232865938673559697851227323497148536117267854241288
44425936468288462848395149959678842215853561564389485413422813386
64558359733811767982282485122488769592428259771817485135798694145
17145764554656647599363636643624443394141749674594439266267914738
89687344812176758317288229174788352467288242171125512646356965953
72436836424726621961424876248346712363842529736689287535527512173
18295771348356417112646514812963612341591986162693455745689374361
56445661964557624561727322332461348422854112571195242864151143533
77537797151985578367895335725777225518396231453691496787716283477
37666899356978497489345173784484282858559847597424967325966961183
26423131974661694562195955939964966722352323745667498767153191712
99821139398463125478734415536932821142852955688669975837535594682
17768265895455681847771319336534851247125295119363323122744953158
25655579913247189643736314385964221584784477663153155222414634387
62881693835262899543396571369125158422922821541597516885389448546
71751114798332662666694134456689735288947441583123159231519473489
94932859392146885633942828174712588132581248183339538341386944937
53828883514868969493559487848248847169557825166338328352792866332
54329673374115668178556175692459528276819221245996289611868492731
97799599164121988455613343238811122469229423272696867686953891233
56249752581283778997317243845187615584225693829653495119532543712
39171354221177772498317826968247939792845866251456175433557619425
56425749216121421458547849142439211299266255482219915528173596421
48679971256541851497913572722857258171788611888347747362797259539
32676924489943265499379145361515824954991343541956993467914114579
45733396847369746189956225365375253819969643711633873473662833395
42291594527499443926636288241672629499242134451937866578992236427
47615394883193571183931424851238451485822477158595936634849167455
16742896921499963113544858716552428241241973653655714294517865841
57496921774277833341488566199458567884285639693339942468585269698
22734249697451127789698862596688824444191118289959746248348491792
28575193613471799766369217455617858422158428235521423695479745656
74234343226976999161289522983885254212712515669681365845434541257
43457237419516813368452247532764649744546181229533942414983335895"
            )
        );
    }
}
