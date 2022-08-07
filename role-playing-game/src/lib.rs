use std::cmp::min;
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0 {
            return None
        }
        let revieved = Player {
            health: 100,
            mana: if self.level >= 10 { Some(100)} else {None},
            ..*self
        };
        Some(revieved)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            // player has no ability to cast
            None => {
                self.health -= min(mana_cost, self.health);
                0
            },
            // not enough mana
            Some(mana_count) if mana_cost > mana_count => 0,

            Some(mana_count) => {
                self.mana = Some(mana_count - mana_cost);
                mana_cost * 2
            }  
            
        }
    }
}
