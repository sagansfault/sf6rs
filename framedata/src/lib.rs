mod loader;

pub(crate) mod selectors {}

#[derive(Debug, Clone)]
pub struct Move {
    pub identifier: String,
    pub input: String,
    pub name: String,
    pub image_link: String,
    pub damage: String,
    pub chip_damage: String,
    pub damage_scaling: String,
    pub guard: String,
    pub cancel: String,
    pub hitconfirm_window: String,
    pub startup: String,
    pub active: String,
    pub recovery: String,
    pub total: String,
    pub hitstun: String,
    pub blockstun: String,
    pub drive_damage_block: String,
    pub drive_damage_hit: String,
    pub drive_gain: String,
    pub super_gain_hit: String,
    pub super_gain_block: String,
    pub projectile_speed: String,
    pub invuln: String,
    pub armor: String,
    pub airborne: String,
    pub juggle_start: String,
    pub juggle_increase: String,
    pub juggle_limit: String,
    pub perfect_parry_advantage: String,
    pub after_dr_hit: String,
    pub after_dr_block: String,
    pub dr_cancel_hit: String,
    pub dr_cancel_block: String,
    pub punish_advantage: String,
    pub hit_advantage: String,
    pub block_advantage: String,
    pub notes: String
}

pub struct LazyLock<T, F = fn() -> T> {
    data: std::sync::OnceLock<T>,
    f: F,
}

impl<T, F> LazyLock<T, F> {
    pub const fn new(f: F) -> LazyLock<T, F> {
        Self {
            data: std::sync::OnceLock::new(),
            f,
        }
    }
}

impl<T> std::ops::Deref for LazyLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.get_or_init(self.f)
    }
}

#[tokio::test]
async fn test() {
    let load = loader::load(&character::RYU).await.unwrap();
    println!("{:#?}", load.first());
}