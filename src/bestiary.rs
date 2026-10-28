//! Stable wave identities: art can vary without changing the saved battle state.
pub const NAMES: [&str; 8] = [
    "Moonwisp fox",
    "Sakura slime",
    "Jade kodama",
    "Astral moth",
    "Clockwork lantern",
    "Jade colossus",
    "Ember oni",
    "Lunar dragon",
];

pub fn enemy_for_wave(stage: u32) -> usize {
    let stage = stage.max(1);
    if stage.is_multiple_of(10) {
        return 5 + ((stage / 10 - 1) % 3) as usize;
    }
    let realm = (((stage - 1) / 20) as usize).min(4);
    const POOLS: [[usize; 3]; 5] = [[0, 1, 2], [2, 1, 3], [3, 4, 0], [4, 2, 1], [3, 4, 0]];
    POOLS[realm][((stage - 1) % 3) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn each_wave_rotates_and_guardians_have_their_own_art() {
        assert_eq!(
            [enemy_for_wave(1), enemy_for_wave(2), enemy_for_wave(3)],
            [0, 1, 2]
        );
        assert_eq!(
            [enemy_for_wave(10), enemy_for_wave(20), enemy_for_wave(30)],
            [5, 6, 7]
        );
        let mut seen = [false; 8];
        for wave in 1..=500 {
            let i = enemy_for_wave(wave);
            assert!(i < NAMES.len());
            assert_eq!(i >= 5, wave.is_multiple_of(10));
            seen[i] = true;
        }
        assert!(seen.into_iter().all(|v| v));
    }
    #[test]
    fn realm_rosters_and_invalid_wave_have_known_identities() {
        for (wave, expected) in [
            (21, 3),
            (22, 2),
            (23, 1),
            (41, 4),
            (61, 4),
            (81, 0),
            (499, 3),
            (500, 6),
        ] {
            assert_eq!(enemy_for_wave(wave), expected);
        }
        assert_eq!(enemy_for_wave(0), enemy_for_wave(1));
    }
}
