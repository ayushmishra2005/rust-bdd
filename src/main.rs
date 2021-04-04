
use ogma::bdd as ogma_bdd;
use ogma::module::{Module as ModuleTrait, ModuleList, ModuleType};

use rust_bdd::bdd;
use failure::Error;

fn main() {
    test_bdd();
}

fn test_bdd() -> Result<(), Error> {
    let mut ctx = ogma_bdd::Step::new();
    let script = bdd::module()
        .compile(
            &mut ctx,
            r#"
        Given the addition of the input and 4 henceforth the left
        And the difference of the input and -4 henceforth the right
        When the left is equal to the right
        Then do nothing
        "#,
        )
        .unwrap();
    let mut instance = script.instance();
    instance.ctx_mut().set_global::<_, i32>("input", 3);
    instance.exec().unwrap();
    let left = instance.ctx().get_global::<_, i32>("left").unwrap();
    assert_eq!(left, Some(&7));
    let right = instance.ctx().get_global::<_, i32>("right").unwrap();
    assert_eq!(right, Some(&7));
    Ok(())
}