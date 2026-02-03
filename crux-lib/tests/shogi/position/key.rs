use crux_lib::shogi::position::key::Key;

#[test]
fn default() {
    assert_eq!(Key::default().value(), 0);
}

#[test]
fn xor() {
    let a = Key::from(0b1010);
    let b = Key::from(0b1100);

    let c = a ^ b;
    assert_eq!(c.value(), 0b0110);
}

#[test]
fn xor_assign() {
    let mut a = Key::from(0b1010);
    let b = Key::from(0b1100);

    a ^= b;

    assert_eq!(a.value(), 0b0110);
}
