use crate::models::types::{CollectConfig, ScanData};
use rand::Rng;

pub struct CollectCreditsEvent<'scan> {
    pub scan_data: &'scan ScanData,
    pub player_bonus: i32,
    pub config: &'scan CollectConfig,
}

impl<'scan> CollectCreditsEvent<'scan> {
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
