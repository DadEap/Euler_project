extern crate euler_project;

use euler_project::utils::*;

#[test]
pub fn is_prim_test() {
    assert_eq!(true, prim::is_prim(2));
    assert_eq!(true, prim::is_prim(3));
    assert_eq!(true, prim::is_prim(5));
    assert_eq!(true, prim::is_prim(7));
    assert_eq!(true, prim::is_prim(11));
    assert_eq!(true, prim::is_prim(13));
    assert_eq!(false, prim::is_prim(81));
}
