/// # Seq Play (Sequenced Play)

use crate::prelude::*;

pub enum Turn {

}

impl Applicable for Turn {
    fn apply_to(self, state: State) -> CmdResult<State> {
        todo!()
    }
    fn apply_to_default(self) -> CmdResult<State> {
        self.apply_to( State::default() )
    }
}