use rand::RngExt;

const ADJECTIVES: &str = "amber, ancient, arctic, autumn, azure, blazing, bold, brave, bright, calm, cedar, celestial, charming, chill, clean, clever, cloud, coastal, cobalt, cool, copper, coral, cosmic, crisp, crystal, daring, dawn, deep, deft, dew, distant, divine, drift, dusk, eager, early, emerald, epic, eternal, faint, fierce, fleet, floating, forest, fresh, frost, gentle, gilded, glacial, glowing, golden, grand, green, hallowed, hidden, high, hollow, humble, icy, idle, ivory, jade, keen, kindred, lofty, lone, loyal, lunar, majestic, maple, marble, mellow, mighty, misty, noble, northern, oak, open, pale, patient, peaceful, pine, polar, proud, quiet, radiant, rapid, roaming, rocky, royal, rugged, rustic, sage, serene, silent, silver, sleek, solar, steady, still, stone, stormy, sunlit, swift, tidal, timid, twilight, vast, velvet, verdant, wandering, warm, wild, windy, winter, wise, woven, zeal";

const NOUNS: &str = "anchor, anvil, arch, arrow, ash, aspen, bay, beacon, birch, blade, bloom, boulder, branch, brook, canyon, cave, cedar, cliff, cloud, coast, comet, compass, creek, crest, crown, dale, dawn, delta, dune, dusk, eagle, echo, elm, ember, falcon, fern, fjord, flame, flint, fog, forge, frost, gale, glade, grove, gulf, harbor, hawk, hazel, heath, helm, hollow, horizon, inlet, iris, island, ivy, kestrel, lagoon, lake, lantern, larch, laurel, leaf, ledge, maple, marsh, meadow, mesa, mist, moon, moss, mountain, oak, orbit, peak, pine, plain, pond, prism, ravine, reed, reef, ridge, river, rock, rune, sage, shore, sierra, slate, slope, spruce, star, stone, storm, stream, summit, thorn, tide, timber, vale, valley, vapor, vault, vine, wave, willow, wind, wolf, wood";

fn get_random_el<T>(list: &[T]) -> &T {
    let mut rng = rand::rng();
    let rand_idx = rng.random_range(0..list.len());
    &list[rand_idx]
}

fn get_one<'a>(input_list: &'a str) -> String {
    let split_list: Vec<&str> = input_list.split(",").map(|z| z.trim()).collect();
    get_random_el(&split_list).to_string()
}

pub fn generate() -> String {
    format!(
        "{}-{}-{}",
        get_one(ADJECTIVES),
        get_one(NOUNS),
        rand::rng().random_range(0..=999)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_parts() {
        let id = generate();
        let parts: Vec<&str> = id.split("-").collect();
        assert_eq!(parts.len(), 3);
    }
}
