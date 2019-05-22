use super::rand::{thread_rng, Rng};

static MISSTEPS: &'static [&str] = &[
    "(⌒,_ゝ⌒ ) 何やこの厨パァ！！！",
    "(⌒,_ゝ⌒ ) 厨ポケの５、６匹でガタガタ言うなよ～！",
    "(⌒,_ゝ⌒ ) 重くないか？その名前。",
    "(⌒,_ゝ⌒ ) 何やこの深海魚ォ！",
    "(⌒,_ゝ⌒ ) 見たかお前！降参読み降参じゃ！",
];

pub fn mokou() {
    let mut rng = thread_rng();
    let idx: usize = rng.gen_range(0, MISSTEPS.len());
    println!("{}", MISSTEPS[idx]);
}
