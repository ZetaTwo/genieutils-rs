mod civ;
mod common;
mod datfile;
mod effect;
mod graphic;
mod playercolour;
mod randommaps;
mod sound;
mod task;
mod tech;
mod techtree;
mod terrainblock;
mod terrainrestrictions;
mod unit;
mod unitheaders;
mod versions;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
