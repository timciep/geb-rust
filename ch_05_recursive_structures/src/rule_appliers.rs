pub fn apply_g(x: i32) -> i32 {
    if x == 0 {
        0
    } else {
        let y = x - apply_g(apply_g(x - 1));
        y
    }
}

// Found solution in: https://arxiv.org/pdf/1509.02479.pdf ... idk how I could have come up w/ this on my own.
pub fn apply_g_flip(x: i32) -> i32 {
    if x == 0 {
        0
    } else if x >= 1 && x <= 2 {
        1
    } else if x == 3 {
      2  
    } else {
        let y = x + 1 - apply_g_flip(1 + apply_g_flip(x - 1));
        y
    }
}