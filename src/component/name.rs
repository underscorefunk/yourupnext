use crate::prelude::*;

pub type Name = String;

pub fn rename(mut state: State, id: Id, new_name: String) -> ActionResult<State> {
    state.name.update(id, new_name)?;
    Ok(state)
}