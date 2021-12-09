fn can_win_nim(n: i32) -> bool {
    // you get first
    // pick the 4-x stone which x is the number of another player pick
    // your last term and remains 4 stone, win
    n % 4 != 0
}

fn main() {
    assert_eq!(can_win_nim(4), false);
    assert_eq!(can_win_nim(1), true);
}
