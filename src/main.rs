use std::str;

fn main() {
    println!(
        "Hello, {}! Welcome to your first adventure. \n",
        WARRIOR.name
    );

    battle(WARRIOR, FIRST_BOSS);
}

fn battle(warrior: Character, boss: Boss) {
    println!(
        "Lets get Ready to Rumble!\n
  Tonights fight will be {} vs {}",
        warrior.name, boss.name
    )
}

#[allow(dead_code)]
fn get_char_stats(warrior: Character) {
    print!(
        "Character Stats: \n Name: {} \n Health: {} \n Damage: {} \n Money: {} \n \n",
        warrior.name, warrior.health, warrior.hit_damage, warrior.coins_balance
    );
}

#[allow(dead_code)]
fn get_boss_stats(boss: Boss) {
    print!("Boss Stats: \n Name: {}, \n Difficulty: {:?},\n Health: {},\n Hitdamage: {}, \n Fail Reward: {}, \n Success Reward: {} \n \n",
    boss.name, boss.diffulty, boss.health, boss.hit_damage, boss.fail_reward, boss.success_reward);
}

#[allow(dead_code)]
struct Boss<'a> {
    name: &'a str,
    diffulty: BossDifficulty,
    health: i32,
    hit_damage: i32,
    fail_reward: i32,
    success_reward: i32,
}

#[derive(Debug)] // Why is this different than #[allow(dead_code)] ?
#[allow(dead_code)]
enum BossDifficulty {
    Easy,
    Moderate,
    Hard,
    Legendary,
}

#[allow(dead_code)]
struct Character<'a> {
    name: &'a str,
    health: i32,
    hit_damage: i32,
    coins_balance: i32,
}

#[allow(dead_code)]
const WARRIOR: Character = Character {
    name: "Mad Villain",
    health: 100,
    hit_damage: 100,
    coins_balance: 0,
};

#[allow(dead_code)]
const FIRST_BOSS: Boss = Boss {
    name: "Biggie",
    diffulty: BossDifficulty::Easy,
    health: 1000,
    fail_reward: 150,
    hit_damage: 40,
    success_reward: 500,
};
