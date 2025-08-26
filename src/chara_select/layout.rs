use super::*;

const ORDER_TOML: &str = "ui/param/menu/chara_icon_order.toml";
const RANDOM_IDX_TOML: &str = "ui/param/menu/chara_random_idx.toml";

#[derive(Debug, Deserialize)]
#[serde(default)]
struct CharaConfig {
    enabled: bool,
    order: String,
    #[serde(flatten)]
    schemas: HashMap<String, CharaSchema>
}
impl Default for CharaConfig {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("series".into(), CharaSchema::default());
        CharaConfig {
            enabled: true,
            order: "series".into(),
            schemas: map
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
struct CharaSchema {
    centered_random: bool,
    order: Vec<String>
}
impl Default for CharaSchema {
    fn default() -> Self {
        CharaSchema {
            centered_random: true,
            order: [
                // default order of fighters in the CSS for when tourney mode is enabled, or config is invalid
                "mario", "luigi", "mariod", "peach", "daisy", "rosetta", "koopa", "koopajr", "packun", "yoshi", "wario", "donkey", "diddy", "krool", "buddy", "murabito", "shizue",
                "link", "younglink", "toonlink", "zelda", "sheik", "ganon", "samus", "szerosuit", "ridley", "samusd", "kirby", "metaknight", "dedede", "fox", "falco", "wolf", "ness", "lucas",
                "pikachu", "pichu", "purin", "mewtwo", "ptrainer", "lucario", "gekkouga", "gaogaen", "marth", "roy", "ike", "reflet", "chrom", "lucina", "kamui", "master",
                "captain", "ice_climber", "gamewatch", "pit", "pitb", "palutena", "pikmin", "robot", "wiifit", "littlemac", "shulk", "element", "duckhunt", "inkling", "tantan", "miifighter", "miiswordsman", "miigunner",
                "snake", "simon", "richter", "sonic", "bayonetta", "jack", "rockman", "ryu", "ken", "dolly", "demon", "pacman", "cloud", "edge", "trail", "brave", "pickel"
            ]
            .map(|x| x.to_string()).to_vec()
        }
    }
}