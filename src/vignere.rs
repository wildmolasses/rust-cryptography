pub fn vignere_encrypt(m: &str, k: &str) -> String {
    if k.chars().any(|char| !char.is_ascii_alphabetic()) {
        panic!("non ascii alphabetic keyword")
    };
    let k = k.to_ascii_uppercase().as_bytes().to_owned();
    let mut ciphertext: Vec<char> = Vec::new();
    for (i, char) in m
        .chars()
        .filter(|char| char.is_ascii_alphabetic())
        .enumerate()
    {
        let char = char.to_ascii_uppercase();
        let i: u32 = k[i % k.len()].into();
        let char = shift(char, i);
        ciphertext.push(char);
    }
    ciphertext.into_iter().collect()
}

pub fn shift(char: char, n: u32) -> char {
    let char = char as u32 - 65;
    let n = n - 65;
    char::from_u32(((n + char) % 26) + 65).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(vignere_encrypt("attackatdawn", "LEMON"), "LXFOPVEFRNHR");
        assert_eq!(vignere_encrypt("DO YOU KNOW THE LAND WHERE THE ORANGE TREE BLOSSOMS?
        THE COUNTRY OF GOLDEN FRUITS AND MARVELOUS ROSES,
        WHERE THE BREEZE IS SOFTER AND BIRDS LIGHTER,
        WHERE BEES GATHER POLLEN IN EVERY SEASON,
        AND WHERE SHINES AND SMILES, LIKE A GIFT FROM GOD,
        AN ETERNAL SPRINGTIME UNDER AN EVER-BLUE SKY!
        ALAS! BUT I CANNOT FOLLOW YOU
        TO THAT HAPPY SHORE FROM WHICH FATE HAS EXILED ME!
        THERE! IT IS THERE THAT I SHOULD LIKE TO LIVE,
        TO LOVE, TO LOVE, AND TO DIE!
        IT IS THERE THAT I SHOULD LIKE TO LIVE, IT IS THERE, YES, THERE!", "AMBROISETHOMAS"), "DAZFISFSPAVQLSNPXYSZWXALCDAFGQUISMTPHZGAMKTTFTCCFXKFCRGGLPFETZMMMZOZDEADWVZWMWKVGQSOHQSVHPWFKLSLEASEPWHMJEGKPURVSXJXVBWVPOSDETEQTXOBZIKWCXLWNUOVJMJCLLOEOFAZENVMJILOWZEKAZEJAQDILSWWESGUGKTZGQZVRMNWTQSEOTKTKPBSTAMQVERMJEGLJQRTLGFJYGSPTZPGTACMOECBXSESCIYGUFPKVILLTWDKSZODFWFWEAAPQTFSTQIRGMPMELRYELHQSVWBAWMOSDELHMUZGPGYEKZUKWTAMZJMLSEVJQTGLAWVOVVXHKWQILIEUYSZWXAHHUSZOGMUZQCIMVZUVWIFJJHPWVXFSETZEDF");
    }
}
