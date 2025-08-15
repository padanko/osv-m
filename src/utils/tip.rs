use rand;

pub fn random_tip() -> String{
    let tips = include_str!("../../misc/tips.txt").to_string();
    let tips_: Vec<String> = tips.lines().map(|tip| tip.to_string()).collect();
    if let Some(tip) = tips_.get(rand::random_range(0..tips_.len())) {
        tip.to_string()
    } else {
        String::new()
    }
}