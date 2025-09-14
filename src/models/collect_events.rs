use crate::models::types::{CollectConfig, ScanData};
use rand::Rng;

pub struct CollectCreditsEvent<'a> {
    pub scan_data: &'a ScanData,
    pub player_bonus: i32,
    pub config: &'a CollectConfig,
}

impl<'a> CollectCreditsEvent<'a> {
    pub fn attempt(&self) -> Option<u32> {
        let mut chance = self.config.base_chance + self.player_bonus;

        if let Some(modifier) = self
            .config
            .weather_mods
            .get(self.scan_data.weather.as_str())
        {
            chance += *modifier;
        }

        chance -= (self.scan_data.threat_level / 2) as i32;

        if chance < 1 {
            chance = 1;
        }

        let roll = rand::rng().random_range(0..100);

        if roll < chance {
            Some(self.scan_data.scrap_value)
        } else {
            None
        }
    }
}
