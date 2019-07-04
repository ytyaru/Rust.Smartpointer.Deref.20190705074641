/*
 * Rustのスマートポインタ（Derefトレイト）
 * CreatedAt: 2019-07-05
 */
fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, y);
}

