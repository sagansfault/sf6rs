extern crate regex;

use std::hash::{Hash, Hasher};
use std::sync::OnceLock;

use regex::Regex;
use crate::LazyLock;

pub static RYU: CharacterId = CharacterId::new("ryu", "Ryu", r"ryu");
pub static LUKE: CharacterId = CharacterId::new("luke", "Luke", r"luke");
pub static JAMIE: CharacterId = CharacterId::new("jamie", "Jamie", r"jamie");
pub static CHUNLI: CharacterId = CharacterId::new("chunli", "Chun-Li", r"chun(-?li)?");
pub static GUILE: CharacterId = CharacterId::new("guile", "Guile", r"guile");
pub static KIMBERLY: CharacterId = CharacterId::new("kimberly", "Kimberly", r"kim(berly)?");
pub static JURI: CharacterId = CharacterId::new("juri", "Juri", r"juri");
pub static KEN: CharacterId = CharacterId::new("ken", "Ken", r"ken");
pub static BLANKA: CharacterId = CharacterId::new("blanka", "Blanka", r"blanka");
pub static DHALSIM: CharacterId = CharacterId::new("dhalsim", "Dhalsim", r"(dh?al)?sim");
pub static EHONDA: CharacterId = CharacterId::new("ehonda", "E.Honda", r"e?honda");
pub static DEEJAY: CharacterId = CharacterId::new("deejay", "Dee_Jay", r"d(ee)?j(ay)?");
pub static MANON: CharacterId = CharacterId::new("manon", "Manon", r"manon");
pub static MARISA: CharacterId = CharacterId::new("marisa", "Marisa", r"marisa");
pub static JP: CharacterId = CharacterId::new("jp", "JP", r"jp");
pub static ZANGIEF: CharacterId = CharacterId::new("zangief", "Zangief",  r"(zan)?gief");
pub static LILY: CharacterId = CharacterId::new("lily", "Lily", r"lily");
pub static CAMMY: CharacterId = CharacterId::new("cammy", "Cammy", r"cammy");
pub static RASHID: CharacterId = CharacterId::new("rashid", "Rashid", r"rashid");
pub static AKI: CharacterId = CharacterId::new("aki", "A.K.I.", r"a\.?k\.?i\.?");
pub static ED: CharacterId = CharacterId::new("ed", "Ed", r"ed");
pub static AKUMA: CharacterId = CharacterId::new("akuma", "Akuma", r"akuma|gouki");

/// A collection of references to all the currently supported characters in this library
pub static CHARACTERS: LazyLock<Vec<&CharacterId>> = LazyLock::new(|| vec!(
    &RYU, &LUKE, &JAMIE, &CHUNLI, &GUILE, &KIMBERLY, &JURI, &KEN, &BLANKA, &DHALSIM, &EHONDA,
    &DEEJAY, &MANON, &MARISA, &JP, &ZANGIEF, &LILY, &CAMMY, &RASHID, &AKI, &ED, &AKUMA
));

/// Finds a character by matching against their regex
pub fn get_character_by_regex<'a>(input: &str) -> Option<&'a CharacterId> {
    CHARACTERS.iter().find(|c| c.regex().is_match(input)).copied()
}

/// Finds a character by matching against their id. Case sensitive.
pub fn get_character_by_id<'a>(input: &str) -> Option<&'a CharacterId> {
    CHARACTERS.iter().find(|c| c.id.eq(input)).copied()
}

/// A struct representing a character this library supports. Unique by `id`
#[derive(Clone, Debug)]
pub struct CharacterId {
    pub id: &'static str,
    pub frame_data_id: &'static str,
    regex_str: &'static str,
    regex_compiled: OnceLock<Regex>
}

impl CharacterId {
    const fn new(id: &'static str, frame_data_id: &'static str, regex: &'static str) -> Self {
        CharacterId {
            id,
            frame_data_id,
            regex_str: regex,
            regex_compiled: OnceLock::new(),
        }
    }

    /// Gets the regex for this [`CharacterId`] via a [`OnceLock`]
    pub fn regex(&self) -> &Regex {
        self.regex_compiled.get_or_init(|| Regex::new(&format!(r"(?i)^{}$", self.regex_str)).unwrap())
    }

    /// Returns the url this library scrapes the data for each character
    pub fn frame_data_url(&self) -> String {
        format!("https://wiki.supercombo.gg/w/Street_Fighter_6/{}/Data", self.frame_data_id)
    }
}

impl PartialEq<Self> for CharacterId {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(other.id)
    }
}

impl Eq for CharacterId {
    
}

impl Hash for CharacterId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.id.as_ref())
    }
}