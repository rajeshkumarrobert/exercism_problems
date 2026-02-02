pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut res = String::new();
    let mut initial = start_bottles;
    for _ in 0..take_down {
        let lyrics = match initial {
            10 => String::from(
                "Ten green bottles hanging on the wall,\nTen green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be nine green bottles hanging on the wall.\n",
            ),
            9 => String::from(
                "Nine green bottles hanging on the wall,\nNine green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be eight green bottles hanging on the wall.\n",
            ),
            8 => String::from(
                "Eight green bottles hanging on the wall,\nEight green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be seven green bottles hanging on the wall.\n",
            ),
            7 => String::from(
                "Seven green bottles hanging on the wall,\nSeven green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be six green bottles hanging on the wall.\n",
            ),
            6 => String::from(
                "Six green bottles hanging on the wall,\nSix green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be five green bottles hanging on the wall.\n",
            ),
            5 => String::from(
                "Five green bottles hanging on the wall,\nFive green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be four green bottles hanging on the wall.\n",
            ),
            4 => String::from(
                "Four green bottles hanging on the wall,\nFour green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be three green bottles hanging on the wall.\n",
            ),
            3 => String::from(
                "Three green bottles hanging on the wall,\nThree green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be two green bottles hanging on the wall.\n",
            ),
            2 => String::from(
                "Two green bottles hanging on the wall,\nTwo green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be one green bottle hanging on the wall.\n",
            ),
            1 => String::from(
                "One green bottle hanging on the wall,\nOne green bottle hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be no green bottles hanging on the wall.\n",
            ),
            _ => String::new(),
        };
        initial-=1;
        res.push_str(&lyrics);
        res.push('\n')
    }
    res
}
