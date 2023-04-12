use std::collections::HashMap;

pub fn get_colors() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("red", "\x1b[31m");
    map.insert("green", "\x1b[32m");
    map.insert("yellow", "\x1b[33m");
    map.insert("blue", "\x1b[34m");
    map.insert("reset", "\x1b[0m");
    map
}
pub static ARROW_TEXT: &[&str] = &[
    "As you scour the forest floor, your eyes land on a glinting object - an arrow lying at your feet.",
    "As your torch flickers, you catch a glimmer of metal on the cave floor - an arrow lies at your feet, abandoned by its owner.",
    "As you descend deeper into the darkness of the cave, the only sound you hear is the soft drip of water - until you notice a discarded arrow, illuminated by the light of your torch."
];
pub static WUMPUS_TEXT: &[&str] = &[
"Suddenly, you spot the elusive wumpus ahead, but before you can even think of a plan, it stomps towards you, crushing you under its massive weight.",
"As you wander deeper into the twisting caverns, the hairs on the back of your neck begin to rise. Suddenly, an enormous shape appears before you, and you realize with a start that it is the dreaded Wumpus! With a deafening roar, it charges towards you, its massive bulk crushing you underfoot before you can even draw your bow.",
"As you cautiously make your way through the dimly lit maze, a sudden tremor shakes the ground beneath your feet. Before you can react, the monstrous Wumpus appears, its eyes blazing with fury as it charges towards you, its crushing weight sending you tumbling to the ground."

];
