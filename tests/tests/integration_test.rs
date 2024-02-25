use tests::sec3::external_add_two;

mod common;



#[test]
fn test_external_add() {
    common::common();
    let res = external_add_two(10);
    assert_eq!(12, res);
}