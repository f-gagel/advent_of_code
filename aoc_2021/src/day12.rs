use std::str::FromStr;

use common::input::Linewise;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Parsing(#[from] pattern_parse::ParseError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cave {
    Start,
    End,
    Big(u16),
    Small(u16),
}

impl pattern_parse::PatternParse for Cave {
    type Error = Error;

    fn parse(input: &str) -> Result<(Self, usize), Self::Error> {
        if input.starts_with("start") {
            return Ok((Self::Start, 5));
        } else if input.starts_with("end") {
            return Ok((Self::End, 3));
        }

        let is_upper = input.chars().next().unwrap().is_uppercase();
        let value: u16 = {
            let mut bytes = [0; 2];
            bytes.copy_from_slice(&input.as_bytes()[0..2]);
            u16::from_le_bytes(bytes)
        };
        match is_upper {
            true => Ok((Self::Big(value), 2)),
            false => Ok((Self::Small(value), 2)),
        }
    }
}

pub struct CavePair(Cave, Cave);

impl FromStr for CavePair {
    type Err = pattern_parse::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        pattern_parse::parse_fn! {
            parse_pair,
            "{Cave}-{Cave}"
        }

        let (a, b) = parse_pair(s)?;
        Ok(CavePair(a, b))
    }
}

type CaveConnections = ahash::HashMap<Cave, Vec<Cave>>;

fn build_connections(input: Linewise<CavePair>) -> Result<CaveConnections, Error> {
    let mut connections = CaveConnections::default();
    for i in input {
        let i = i?;
        connections.entry(i.0).or_insert(Vec::new()).push(i.1);
        connections.entry(i.1).or_insert(Vec::new()).push(i.0);
    }
    Ok(connections)
}

pub fn paths(path: &mut Vec<Cave>, connections: &CaveConnections, allow_revisit_small: bool) -> u32 {
    let mut result = 0;

    // check where we are and where we can go
    let head = path.last().unwrap();
    for next in connections[head].iter() {
        match next {
            // looping back to the start is not a valid path
            Cave::Start => {},
            Cave::End => {
                // found a valid path to the end
                result += 1;
            },
            Cave::Big(_) => {
                // plan a further path
                path.push(*next);
                result += paths(path, connections, allow_revisit_small);
                path.pop();
            },
            Cave::Small(_) => {
                // make sure we don't double cross
                if path.contains(next) {
                    // ..unless we can revisit a small
                    if allow_revisit_small {
                        path.push(*next);
                        result += paths(path, connections, false);
                        path.pop();
                    }
                } else {
                    path.push(*next);
                    result += paths(path, connections, allow_revisit_small);
                    path.pop();
                }
            },
        }
    }

    result
}

pub fn task1(input: Linewise<CavePair>) -> Result<u32, Error> {
    let connections = build_connections(input)?;

    let mut path = vec![Cave::Start];
    let result = paths(&mut path, &connections, false);

    Ok(result)
}

pub fn task2(input: Linewise<CavePair>) -> Result<u32, Error> {
    let connections = build_connections(input)?;

    let mut path = vec![Cave::Start];
    let result = paths(&mut path, &connections, true);

    Ok(result)

}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    const INPUT: &[u8] = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
        .as_bytes();

    #[test]
    fn test_task1() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 226);
    }
    #[test]
    fn test_task2() {
        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 3509);
    }
}
