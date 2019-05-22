static MISSTEPS: &'static [&str] = &[
    "(⌒,_ゝ⌒ ) 何やこの厨パァ！！！",
    "(⌒,_ゝ⌒ ) 厨ポケの５、６匹でガタガタ言うなよ～！",
    "(⌒,_ゝ⌒ ) 重くないか？その名前。",
    "(⌒,_ゝ⌒ ) 何やこの深海魚ォ！",
];

pub fn mokou() {
    let len = MISSTEPS.len();
    let idx = len % 3;
    println!("{}", MISSTEPS[idx]);
}
