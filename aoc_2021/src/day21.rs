use ahash::{AHashMap, HashMap};
use common::input::Linewise;
use common::iter_ext::TryIterator;
use pattern_parse::ParseError;
use std::cmp::max;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Pattern(#[from] ParseError),
}

pattern_parse::parse_fn! {
    parse_line,
    "Player {u8} starting position: {u8}"
}

#[derive(Debug, Default, Copy, Clone)]
struct Player {
    id: u8,
    position: u8,
    score: u16,
}

pub fn task1(input: Linewise<String>) -> Result<u32, Error> {
    let mut players: Vec<Player> = input
        .into_iter()
        .map(|s| {
            parse_line(&s.unwrap()).map(|(id, position)| Player {
                id,
                position,
                score: 0,
            })
        })
        .try_collect2()?;

    let winner_id;
    let mut rolls = 0u32;
    'outer: loop {
        for player in &mut players {
            // rolls are always: n+1 + n+2 + n+3
            let rolled_move = rolls * 3 + 6;
            rolls += 3;

            update_player(player, rolled_move);

            if player.score >= 1000 {
                winner_id = player.id;
                break 'outer;
            }
        }
    }

    let looser = players.iter().find(|p| p.id != winner_id).unwrap();
    Ok(looser.score as u32 * rolls)
}

fn update_player(player: &mut Player, rolled_move: u32) {
    let sum = rolled_move + player.position as u32;
    let landing_tile = ((sum - 1) % 10) + 1;
    player.position = landing_tile as u8;
    player.score += landing_tile as u16;
}

pub fn task2(input: Linewise<String>) -> Result<u64, Error> {
    let mut players = [Player::default(); 2];
    for (i, line) in input.enumerate() {
        let (id, position) = parse_line(&line.unwrap())?;
        players[i] = Player {
            id,
            position,
            score: 0,
        }
    }

    let mut cache = AHashMap::new();
    let [a, b] = recurse_games(players, 0, &mut cache, CacheKey::default(), 0);
    Ok(max(a, b))
}

/// ```text
/// possible rolls: 3..=9 => 7 = ~2^3
/// options per round = 2^6
/// max rounds = ~20 (21 points)
/// total game = 2^(6*20) = 2^120
/// total = 120 bits = 15B
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct CacheKey([u8; 16]);

impl Default for CacheKey {
    fn default() -> Self {
        Self([0; 16])
    }
}

impl CacheKey {
    fn with(mut self, roll: u32, depth: u32) -> Self {
        // remap 3..=9 to 0..=6 (importantly <8)
        let insert = roll - 3;
        let offset = depth * 3;
        for read_bit in 0..3 {
            let set_bit = (insert >> read_bit) & 1;
            let write_bit = (read_bit + offset) as usize;
            let bucket = &mut self.0[write_bit / 8];
            *bucket |= (set_bit as u8) << (write_bit % 8);
        }
        self
    }
}

fn recurse_games(
    players: [Player; 2],
    active_player: usize,
    cache: &mut HashMap<CacheKey, [u64; 2]>,
    key: CacheKey,
    depth: u32,
) -> [u64; 2] {
    // all possible rolls together with their occurrences
    let all_rolls = [
        (3,1),
        (4,3),
        (5,6),
        (6,7),
        (7,6),
        (8,3),
        (9,1),
    ];

    let mut total_wins = [0; 2];

    for (roll, factor) in all_rolls {
        let key = key.clone().with(roll, depth);
        let roll_wins = if let Some(lookup) = cache.get(&key) {
            *lookup
        } else {
            let mut copy = players.clone();
            update_player(&mut copy[active_player], roll);
            if copy[active_player].score >= 21 {
                let mut branch_wins = [0;2];
                branch_wins[active_player] += 1;
                cache.insert(key, branch_wins);

                total_wins[active_player] += factor;
                continue;
            }
            let result = recurse_games(copy, active_player ^ 1, cache, key, depth + 1);
            cache.insert(key, result);
            result
        };
        total_wins[0] += roll_wins[0] * factor;
        total_wins[1] += roll_wins[1] * factor;
    }

    total_wins
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = b"\
Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 739785);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 444356092776315);
    }
}
