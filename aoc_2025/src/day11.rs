use common::input::Linewise;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub struct Node {
    name: u32,
    targets: Vec<u32>,
}

impl FromStr for Node {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = name_to_id(&s[..3]);
        let targets = s[5..].split(' ').map(name_to_id).collect();
        Ok(Self { name, targets })
    }
}

fn name_to_id(name: &str) -> u32 {
    debug_assert_eq!(name.len(), 3);
    debug_assert!(name.is_ascii());
    let mut bytes = [0; 4];
    bytes[..3].copy_from_slice(name.as_bytes());
    u32::from_le_bytes(bytes)
}

pub fn task1(input: Linewise<Node>) -> Result<u64, Error> {
    let map = input
        .map(|node| {
            let x = node.unwrap();
            (x.name, x.targets)
        })
        .collect::<HashMap<_, _>>();

    let you = name_to_id("you");
    let out = name_to_id("out");
    let total = find_total_paths(&map, you, out);
    Ok(total)
}

fn find_total_paths(map: &HashMap<u32, Vec<u32>>, start: u32, end: u32) -> u64 {
    fn dfs(map: &HashMap<u32, Vec<u32>>, node: u32, end: u32, memo: &mut HashMap<u32, u64>) -> u64 {
        if node == end {
            return 1;
        }
        if let Some(&cached) = memo.get(&node) {
            return cached;
        }

        let mut total = 0;
        if let Some(targets) = map.get(&node) {
            for &next in targets {
                total += dfs(map, next, end, memo);
            }
        }

        memo.insert(node, total);
        total
    }

    let mut memo = HashMap::new();
    dfs(map, start, end, &mut memo)
}

pub fn task2(input: Linewise<Node>) -> Result<u64, Error> {
    let svr = name_to_id("svr");
    let fft = name_to_id("fft");
    let dac = name_to_id("dac");
    let out = name_to_id("out");

    let map = input
        .map(|node| {
            let x = node.unwrap();
            (x.name, x.targets)
        })
        .chain([(out, vec![])])
        .collect::<HashMap<_, _>>();

    let svr_fft = find_total_paths(&map, svr, fft);
    let fft_dac = find_total_paths(&map, fft, dac);
    let dac_out = find_total_paths(&map, dac, out);
    let paths1 = svr_fft * fft_dac * dac_out;

    let svr_dac = find_total_paths(&map, svr, dac);
    let dac_fft = find_total_paths(&map, dac, fft);
    let fft_out = find_total_paths(&map, fft, out);
    let paths2 = svr_dac * dac_fft * fft_out;

    Ok(paths1 + paths2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input::Input;

    #[test]
    fn test_task1() {
        const INPUT: &[u8] = b"\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

        let buf = std::io::BufReader::new(INPUT);
        let result = task1(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 5);
    }
    #[test]
    fn test_task2() {
        const INPUT: &[u8] = b"\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

        let buf = std::io::BufReader::new(INPUT);
        let result = task2(Input::parse(buf).unwrap());
        let val = result.unwrap();
        assert_eq!(val, 2);
    }
}
