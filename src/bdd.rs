use ogma::{given, when, then};
use ogma::mod_type;
use ogma::mod_list;
use ogma::object_query::Query;
use ogma::vm::{Context, Trap};
use ogma::bdd;
use ogma::module::{Module as ModuleTrait, ModuleList, ModuleType};

use crate::utility;

#[given(Add, "the addition of q`input` and d`b` henceforth q`out`")]
pub fn add<'a>(
    ctx: &mut Context,
    input: &Vec<Query<'static>>,
    b: i32,
    out: &Vec<Query<'a>>,
) -> Result<(), Trap> {
    let input = input.iter().next().unwrap().as_key().unwrap();
    let out = out.iter().next().unwrap().as_key().unwrap();
    let a = ctx
        .get_global::<_, i32>(input)?
        .ok_or_else(|| Trap::MissingGlobal(input.to_string()))?;
    let c = utility::addition(a.clone(), b);
    ctx.set_global::<_, i32>(out, c);
    Ok(())
}

#[when(Equals, "q`left` is equal to q`right`")]
pub fn equals(
    ctx: &mut Context,
    left: &Vec<Query<'static>>,
    right: &Vec<Query<'static>>,
) -> Result<(), Trap> {
    let left = left.iter().next().unwrap().as_key().unwrap();
    let right = right.iter().next().unwrap().as_key().unwrap();
    let a = ctx
        .get_global::<_, i32>(left)?
        .ok_or_else(|| Trap::MissingGlobal(left.to_string()))?;
    let b = ctx
        .get_global::<_, i32>(right)?
        .ok_or_else(|| Trap::MissingGlobal(right.to_string()))?;
    if a != b {
        Err(Trap::runtime("left not equal to right"))
    } else {
        Ok(())
    }
}

#[then(Noop, "do nothing")]
pub fn noop(_: &mut Context) -> Result<(), Trap> {
    Ok(())
}

pub type Module<'a> = mod_type!(Add<'a>, Equals, Noop);

pub fn module<'a>() -> ModuleList<'a, bdd::Step> {
    mod_list!(bdd::Step => Add, Equals, Noop)
}
