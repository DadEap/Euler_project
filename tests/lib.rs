extern crate euler_project;

use euler_project::utils::*;

#[test]
pub fn is_prim_test() {
    assert_eq!(true, prim::is_prim(2));
    //assert_eq!(true, true);
}
