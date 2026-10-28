use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub const OFFLINE_CAP: f64 = 8.0 * 3600.0;
pub const MAX_LEVEL: u32 = 100;

pub struct Companion {
    pub name: &'static str,
    pub title: &'static str,
    pub element: &'static str,
    pub cost: f64,
    pub dps: f64,
}

pub const COMPANIONS: [Companion; 6] = [
    Companion {
        name: "Koharu",
        title: "Sakura familiar",
        element: "BLOOM",
        cost: 20.0,
        dps: 2.0,
    },
    Companion {
        name: "Ren",
        title: "Wandering ronin",
        element: "WIND",
        cost: 140.0,
        dps: 11.0,
    },
    Companion {
        name: "Mizuki",
        title: "Tide oracle",
        element: "TIDE",
        cost: 900.0,
        dps: 65.0,
    },
    Companion {
        name: "Akari",
        title: "Lantern keeper",
        element: "EMBER",
        cost: 6000.0,
        dps: 360.0,
    },
    Companion {
        name: "Yoru",
        title: "Dream weaver",
        element: "VOID",
        cost: 40000.0,
        dps: 2100.0,
    },
    Companion {
        name: "Tsukiko",
        title: "Moon sovereign",
        element: "LUNAR",
        cost: 300000.0,
        dps: 14000.0,
    },
];

pub struct Upgrade {
    pub name: &'static str,
    pub desc: &'static str,
    pub cost: f64,
}

pub const UPGRADES: [Upgrade; 6] = [
    Upgrade {
        name: "Moonblade",
        desc: "+5 base damage per strike",
        cost: 35.0,
    },
    Upgrade {
        name: "Spirit resonance",
        desc: "+20% companion damage",
        cost: 90.0,
    },
    Upgrade {
        name: "Golden blossoms",
        desc: "+15% spirit gold from battles",
        cost: 120.0,
    },
    Upgrade {
        name: "Celestial focus",
        desc: "+1% critical strike chance",
        cost: 180.0,
    },
    Upgrade {
        name: "Moonwell",
        desc: "+3 passive gold every second",
        cost: 250.0,
    },
    Upgrade {
        name: "Eternal bond",
        desc: "+25% all damage",
        cost: 500.0,
    },
];

pub struct Relic {
    pub name: &'static str,
    pub kind: &'static str,
    pub desc: &'static str,
    pub cost: f64,
    pub rarity: &'static str,
}

pub const RELICS: [Relic; 6] = [
    Relic {
        name: "Sakura edge",
        kind: "KATANA",
        desc: "+25% strike damage / rank",
        cost: 160.0,
        rarity: "RARE",
    },
    Relic {
        name: "Moonstone",
        kind: "TALISMAN",
        desc: "+20% all damage / rank",
        cost: 400.0,
        rarity: "EPIC",
    },
    Relic {
        name: "Foxfire lantern",
        kind: "LANTERN",
        desc: "+30% companion damage / rank",
        cost: 850.0,
        rarity: "EPIC",
    },
    Relic {
        name: "Gilded ofuda",
        kind: "CHARM",
        desc: "+20% gold rewards / rank",
        cost: 1500.0,
        rarity: "RARE",
    },
    Relic {
        name: "Celestial crown",
        kind: "CROWN",
        desc: "+40% all damage / rank",
        cost: 5000.0,
        rarity: "LEGENDARY",
    },
    Relic {
        name: "Astral prism",
        kind: "RELIC",
        desc: "+50% all damage / rank",
        cost: 18000.0,
        rarity: "MYTHIC",
    },
];

pub const REALMS: [(&str, &str, u32); 5] = [
    ("Sakura sanctuary", "Where the first petals fall", 1),
    ("Jade hollow", "A forest that remembers", 21),
    (
        "Twilight archipelago",
        "Islands beyond the waking world",
        41,
    ),
    ("Ember observatory", "The stars burn a little brighter", 61),
    (
        "Palace of the moon",
        "Every journey returns to the light",
        81,
    ),
];

pub const QUESTS: [(&str, &str, f64, u32); 6] = [
    ("First light", "Strike 25 times", 120.0, 0),
    ("Spirit keeper", "Defeat 10 spirits", 250.0, 1),
    ("A little stronger", "Purchase 5 training levels", 350.0, 1),
    ("Something precious", "Forge your first relic", 500.0, 2),
    ("Beyond the gate", "Reach wave 20", 900.0, 3),
    (
        "A golden beginning",
        "Earn 1,000 total spirit gold",
        1500.0,
        3,
    ),
];

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Game {
    pub version: u32,
    pub gold: f64,
    pub earned: f64,
    pub crystals: u32,
    pub ascension: u32,
    pub stage: u32,
    pub best: u32,
    pub run_best: u32,
    pub hp: f64,
    pub companions: [u32; 6],
    pub upgrades: [u32; 6],
    pub relics: [u32; 6],
    pub claimed: [bool; 6],
    pub clicks: u64,
    pub kills: u64,
    pub saved_at: u64,
    pub playtime: f64,
    pub sound: bool,
    pub music: bool,
    pub particles: bool,
}

impl Default for Game {
    fn default() -> Self {
        let mut g = Self {
            version: 1,
            gold: 45.0,
            earned: 0.0,
            crystals: 0,
            ascension: 0,
            stage: 1,
            best: 1,
            run_best: 1,
            hp: 0.0,
            companions: [1, 0, 0, 0, 0, 0],
            upgrades: [0; 6],
            relics: [0; 6],
            claimed: [false; 6],
            clicks: 0,
            kills: 0,
            saved_at: now(),
            playtime: 0.0,
            sound: true,
            music: true,
            particles: true,
        };
        g.hp = g.max_hp();
        g
    }
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl Game {
    pub fn realm(&self) -> usize {
        (((self.stage - 1) / 20) as usize).min(4)
    }
    pub fn boss(&self) -> bool {
        self.stage.is_multiple_of(10)
    }
    pub fn max_hp(&self) -> f64 {
        28.0 * 1.145_f64.powf((self.stage.saturating_sub(1)) as f64)
            * if self.boss() { 5.0 } else { 1.0 }
    }
    pub fn multiplier(&self) -> f64 {
        (1.0 + self.ascension as f64 * 0.20)
            * (1.0 + self.upgrades[5] as f64 * 0.25)
            * (1.0
                + self.relics[1] as f64 * 0.20
                + self.relics[4] as f64 * 0.40
                + self.relics[5] as f64 * 0.50)
    }
    pub fn click_damage(&self) -> f64 {
        (6.0 + self.upgrades[0] as f64 * 5.0)
            * (1.0 + self.relics[0] as f64 * 0.25)
            * self.multiplier()
    }
    pub fn dps(&self) -> f64 {
        COMPANIONS
            .iter()
            .enumerate()
            .map(|(i, c)| c.dps * self.companions[i] as f64)
            .sum::<f64>()
            * (1.0 + self.upgrades[1] as f64 * 0.20 + self.relics[2] as f64 * 0.30)
            * self.multiplier()
    }
    pub fn critical_chance(&self) -> f64 {
        (0.05 + self.upgrades[3] as f64 * 0.01).min(0.65)
    }
    pub fn gold_multiplier(&self) -> f64 {
        1.0 + self.upgrades[2] as f64 * 0.15 + self.relics[3] as f64 * 0.20
    }
    pub fn reward(&self) -> f64 {
        10.0 * 1.12_f64.powf((self.stage.saturating_sub(1)) as f64)
            * self.gold_multiplier()
            * if self.boss() { 4.0 } else { 1.0 }
    }
    pub fn passive_gold(&self) -> f64 {
        self.upgrades[4] as f64 * 3.0 * self.gold_multiplier()
    }
    pub fn gold_per_second(&self) -> f64 {
        self.passive_gold() + self.dps() / self.max_hp() * self.reward()
    }
    pub fn companion_cost(&self, i: usize) -> f64 {
        (COMPANIONS[i].cost * 1.17_f64.powf(self.companions[i] as f64)).ceil()
    }
    pub fn upgrade_cost(&self, i: usize) -> f64 {
        (UPGRADES[i].cost * 1.62_f64.powf(self.upgrades[i] as f64)).ceil()
    }
    pub fn relic_cost(&self, i: usize) -> f64 {
        (RELICS[i].cost * 2.25_f64.powf(self.relics[i] as f64)).ceil()
    }
    pub fn earn(&mut self, amount: f64) {
        if amount.is_finite() && amount > 0.0 {
            self.gold += amount;
            self.earned += amount;
        }
    }
    pub fn buy_companion(&mut self, i: usize) -> bool {
        if i >= 6 || self.companions[i] >= MAX_LEVEL {
            return false;
        }
        let cost = self.companion_cost(i);
        if self.gold < cost {
            return false;
        }
        self.gold -= cost;
        self.companions[i] += 1;
        true
    }
    pub fn buy_upgrade(&mut self, i: usize) -> bool {
        if i >= 6 || self.upgrades[i] >= MAX_LEVEL {
            return false;
        }
        let cost = self.upgrade_cost(i);
        if self.gold < cost {
            return false;
        }
        self.gold -= cost;
        self.upgrades[i] += 1;
        true
    }
    pub fn forge(&mut self, i: usize) -> bool {
        if i >= 6 || self.relics[i] >= 25 {
            return false;
        }
        let cost = self.relic_cost(i);
        if self.gold < cost {
            return false;
        }
        self.gold -= cost;
        self.relics[i] += 1;
        true
    }
    /// Damage can defeat at most the current target, so an attack never chains unexpectedly.
    pub fn damage(&mut self, amount: f64) -> Option<(f64, bool)> {
        if !amount.is_finite() || amount <= 0.0 {
            return None;
        }
        self.hp -= amount;
        if self.hp > 0.0 {
            return None;
        }
        let reward = self.reward();
        let boss = self.boss();
        self.earn(reward);
        self.kills += 1;
        if boss {
            self.crystals = self.crystals.saturating_add(1);
        }
        self.stage = (self.stage + 1).min(500);
        self.best = self.best.max(self.stage);
        self.run_best = self.run_best.max(self.stage);
        self.hp = self.max_hp();
        Some((reward, boss))
    }
    pub fn quest_progress(&self, i: usize) -> (f64, f64) {
        match i {
            0 => (self.clicks as f64, 25.0),
            1 => (self.kills as f64, 10.0),
            2 => (self.upgrades.iter().sum::<u32>() as f64, 5.0),
            3 => (self.relics.iter().sum::<u32>() as f64, 1.0),
            4 => (self.best as f64, 20.0),
            _ => (self.earned, 1000.0),
        }
    }
    pub fn claim(&mut self, i: usize) -> bool {
        if i >= 6 || self.claimed[i] {
            return false;
        }
        let (a, b) = self.quest_progress(i);
        if a < b {
            return false;
        }
        self.claimed[i] = true;
        self.earn(QUESTS[i].2);
        self.crystals += QUESTS[i].3;
        true
    }
    pub fn available_quests(&self) -> usize {
        (0..6)
            .filter(|&i| {
                let (a, b) = self.quest_progress(i);
                !self.claimed[i] && a >= b
            })
            .count()
    }
    pub fn prestige_gain(&self) -> u32 {
        if self.run_best >= 50 {
            self.run_best / 25
        } else {
            0
        }
    }
    pub fn ascend(&mut self) -> bool {
        let gain = self.prestige_gain();
        if gain == 0 {
            return false;
        }
        self.ascension += gain;
        self.gold = 45.0;
        self.stage = 1;
        self.run_best = 1;
        self.companions = [1, 0, 0, 0, 0, 0];
        self.upgrades = [0; 6];
        self.relics = [0; 6];
        self.hp = self.max_hp();
        true
    }
    pub fn warp(&mut self, realm: usize) -> bool {
        if realm >= REALMS.len() || self.run_best < REALMS[realm].2 {
            return false;
        }
        self.stage = REALMS[realm].2;
        self.hp = self.max_hp();
        true
    }
    pub fn spend_crystals(&mut self) -> bool {
        if self.crystals < 5 {
            return false;
        }
        self.crystals -= 5;
        self.earn((self.gold_per_second() * 600.0).max(250.0));
        true
    }
    pub fn offline(&mut self, seconds: f64) -> f64 {
        // An expedition earns gold without consuming waves or granting boss crystals.
        let amount = self.gold_per_second() * seconds.clamp(0.0, OFFLINE_CAP) * 0.5;
        self.earn(amount);
        amount
    }
    pub fn sanitize(&mut self) {
        self.stage = self.stage.clamp(1, 500);
        self.best = self.best.clamp(self.stage, 500);
        self.run_best = self.run_best.clamp(self.stage, 500);
        for level in self.companions.iter_mut().chain(self.upgrades.iter_mut()) {
            *level = (*level).min(MAX_LEVEL);
        }
        for level in &mut self.relics {
            *level = (*level).min(25);
        }
        self.ascension = self.ascension.min(1_000_000);
        if !self.gold.is_finite() || self.gold < 0.0 {
            self.gold = 0.0;
        }
        if !self.earned.is_finite() || self.earned < 0.0 {
            self.earned = 0.0;
        }
        if !self.hp.is_finite() || self.hp <= 0.0 || self.hp > self.max_hp() {
            self.hp = self.max_hp();
        }
        if !self.playtime.is_finite() || self.playtime < 0.0 {
            self.playtime = 0.0;
        }
    }
}

pub fn save_path() -> PathBuf {
    if let Some(path) = std::env::var_os("MOONVEIL_SAVE_PATH") {
        return PathBuf::from(path);
    }
    let base = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    #[cfg(target_os = "macos")]
    let dir = base.join("Library/Application Support/Moonveil");
    #[cfg(not(target_os = "macos"))]
    let dir = base.join(".local/share/moonveil");
    dir.join("save.json")
}

pub fn save(g: &mut Game) -> Result<(), String> {
    let path = save_path();
    save_to(g, &path)
}

fn save_to(g: &mut Game, path: &std::path::Path) -> Result<(), String> {
    use std::io::Write;
    let parent = path.parent().ok_or("Invalid save path")?;
    std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    g.saved_at = now();
    let data = serde_json::to_vec_pretty(g).map_err(|e| e.to_string())?;
    let temp = path.with_extension("tmp");
    let mut file = std::fs::File::create(&temp).map_err(|e| e.to_string())?;
    file.write_all(&data).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);
    std::fs::rename(temp, path).map_err(|e| e.to_string())
}

pub fn load() -> (Game, f64, Option<String>) {
    let path = save_path();
    match std::fs::read(&path) {
        Ok(bytes) => match serde_json::from_slice::<Game>(&bytes) {
            Ok(mut g) => {
                g.sanitize();
                let elapsed = now().saturating_sub(g.saved_at) as f64;
                let earned = if elapsed > 60.0 {
                    g.offline(elapsed)
                } else {
                    0.0
                };
                (g, earned, None)
            }
            Err(_) => {
                let backup = path.with_extension(format!("unreadable-{}.json", now()));
                let _ = std::fs::copy(&path, backup);
                (
                    Game::default(),
                    0.0,
                    Some("An unreadable save was backed up. A new journey has begun.".into()),
                )
            }
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (Game::default(), 0.0, None),
        Err(e) => (
            Game::default(),
            0.0,
            Some(format!("Could not read save: {e}")),
        ),
    }
}

pub fn number(n: f64) -> String {
    if n >= 1e15 {
        format!("{:.2}Q", n / 1e15)
    } else if n >= 1e12 {
        format!("{:.2}T", n / 1e12)
    } else if n >= 1e9 {
        format!("{:.2}B", n / 1e9)
    } else if n >= 1e6 {
        format!("{:.2}M", n / 1e6)
    } else if n >= 10000.0 {
        format!("{:.1}K", n / 1000.0)
    } else if n >= 1000.0 {
        format!("{:.2}K", n / 1000.0)
    } else {
        format!("{:.0}", n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn purchases_are_atomic() {
        let mut g = Game::default();
        assert!(!g.buy_companion(5));
        assert_eq!(g.gold, 45.0);
        let price = g.companion_cost(0);
        assert!(g.buy_companion(0));
        assert_eq!(g.gold, 45.0 - price);
        assert_eq!(g.companions[0], 2);
        assert!(g.companion_cost(0) > price);
    }
    #[test]
    fn boss_rewards_and_next_wave() {
        let mut g = Game {
            stage: 10,
            ..Game::default()
        };
        g.hp = g.max_hp();
        let reward = g.reward();
        let gold = g.gold;
        assert_eq!(g.damage(g.hp), Some((reward, true)));
        assert_eq!(g.stage, 11);
        assert_eq!(g.crystals, 1);
        assert_eq!(g.gold, gold + reward);
        assert_eq!(g.hp, g.max_hp());
    }
    #[test]
    fn offline_progress_is_capped_and_does_not_skip_bosses() {
        let mut a = Game::default();
        let mut b = a.clone();
        assert_eq!(a.offline(OFFLINE_CAP), b.offline(OFFLINE_CAP * 10.0));
        assert_eq!(a.stage, 1);
        assert_eq!(a.crystals, 0);
        assert_eq!(a.offline(-100.0), 0.0);
    }
    #[test]
    fn quests_can_only_be_claimed_once() {
        let mut g = Game {
            clicks: 25,
            ..Game::default()
        };
        assert!(g.claim(0));
        let gold = g.gold;
        assert!(!g.claim(0));
        assert_eq!(g.gold, gold);
        assert!(!g.claim(5));
    }
    #[test]
    fn ascension_preserves_permanent_progress() {
        let mut g = Game {
            run_best: 50,
            stage: 50,
            best: 50,
            crystals: 8,
            ..Game::default()
        };
        g.relics[0] = 3;
        g.claimed[0] = true;
        assert!(g.ascend());
        assert_eq!(g.ascension, 2);
        assert_eq!(g.stage, 1);
        assert_eq!(g.best, 50);
        assert_eq!(g.crystals, 8);
        assert!(g.claimed[0]);
        assert_eq!(g.relics[0], 0);
        assert!(!g.ascend());
        assert!(!g.warp(2));
        assert!(g.click_damage() > Game::default().click_damage());
    }
    #[test]
    fn serialization_round_trip_and_sanitize() {
        let g = Game::default();
        let mut restored: Game = serde_json::from_slice(&serde_json::to_vec(&g).unwrap()).unwrap();
        restored.stage = 0;
        restored.gold = f64::NAN;
        restored.hp = f64::INFINITY;
        restored.companions[0] = u32::MAX;
        restored.sanitize();
        assert_eq!(restored.stage, 1);
        assert_eq!(restored.gold, 0.0);
        assert_eq!(restored.hp, restored.max_hp());
        assert_eq!(restored.companions[0], MAX_LEVEL);
    }
    #[test]
    fn balance_simulation_remains_finite() {
        let mut g = Game::default();
        for _ in 0..100_000 {
            g.damage(g.dps() * 0.1);
            g.earn(g.passive_gold() * 0.1);
            for i in 0..6 {
                if g.gold > g.companion_cost(i) * 1.2 {
                    g.buy_companion(i);
                }
            }
            for i in 0..6 {
                if g.gold > g.upgrade_cost(i) * 1.5 {
                    g.buy_upgrade(i);
                }
            }
            assert!(g.gold.is_finite());
            assert!(g.dps().is_finite());
            assert!(g.hp > 0.0);
        }
        assert!(g.stage > 20);
    }

    #[test]
    fn invalid_and_capped_purchases_do_not_spend_gold() {
        let mut g = Game {
            gold: 1e30,
            companions: [MAX_LEVEL; 6],
            upgrades: [MAX_LEVEL; 6],
            relics: [25; 6],
            ..Game::default()
        };
        let gold = g.gold;
        for i in [0, 5, 6, usize::MAX] {
            assert!(!g.buy_companion(i));
            assert!(!g.buy_upgrade(i));
            assert!(!g.forge(i));
        }
        assert_eq!(g.gold, gold);
        assert!(!g.claim(usize::MAX));
        assert!(!g.warp(usize::MAX));
    }

    #[test]
    fn relics_stack_and_invalid_damage_is_ignored() {
        let mut g = Game {
            gold: 1e8,
            ..Game::default()
        };
        let click = g.click_damage();
        let dps = g.dps();
        let reward = g.reward();
        for i in 0..6 {
            assert!(g.forge(i));
        }
        assert!(g.click_damage() > click);
        assert!(g.dps() > dps);
        assert!(g.reward() > reward);
        let hp = g.hp;
        for amount in [f64::NAN, f64::INFINITY, -1.0, 0.0] {
            assert_eq!(g.damage(amount), None);
        }
        assert_eq!(g.hp, hp);
    }

    #[test]
    fn atomic_save_replaces_existing_file_and_round_trips() {
        let dir = std::env::temp_dir().join(format!(
            "moonveil-save-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let path = dir.join("save.json");
        let mut g = Game {
            gold: 12345.0,
            crystals: 9,
            ..Game::default()
        };
        g.upgrades[2] = 3;
        save_to(&mut g, &path).unwrap();
        let loaded: Game = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(loaded.gold, 12345.0);
        assert_eq!(loaded.crystals, 9);
        assert_eq!(loaded.upgrades, g.upgrades);
        g.gold = 25000.0;
        save_to(&mut g, &path).unwrap();
        let loaded: Game = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        assert_eq!(loaded.gold, 25000.0);
        assert!(!path.with_extension("tmp").exists());
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(dir).unwrap();
    }
}
