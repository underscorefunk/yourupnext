use crate::prelude::*;

pub type TurnCount = u8;

#[cfg(test)]
mod test {

    use crate::prelude::*;

    #[test]
    fn turn_count() {

        let state = State::default()
            .apply( Entity::Add(100) )
            .unwrap();

        assert_eq!( turn_count::qry::count(&state, 100), 0);

        let state = state
            .apply( |state| turn_count::cmd::count(state, 100 ))
            .apply( |state| turn_count::cmd::count(state, 100 ))
            .unwrap();

        assert_eq!( turn_count::qry::count(&state, 100), 2);

        let state = state
            .apply(|state|turn_count::cmd::reset(state, 100))
            .unwrap();

        assert_eq!( turn_count::qry::count(&state, 100), 0);
    }

}

pub mod cmd {
    use super::*;

    /// COMMAND > Increment an entity's turn count
    pub fn count(mut state: State, pub_id: PubId) -> CmdResult<State> {

        let id = entity::qry::id(&state, pub_id);
        let count: TurnCount = qry::count(&state,pub_id) + 1;
        state.turn_count.update(id, count)?;
        Ok(state)
    }

    /// COMMAND > Reset an entity's turn count
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply( Entity::Add(100) )
    ///     .apply( |state| turn_count::cmd::count(state, 100 ) )
    ///     .apply( |state| turn_count::cmd::reset(state, 100) )
    ///     .unwrap();
    ///
    /// assert_eq!( turn_count::qry::count(&state, 100), 0);
    /// ```
    pub fn reset(mut state: State, pub_id: PubId) -> CmdResult<State> {
        let id = entity::qry::id(&state, pub_id);
        state.turn_count.delete(id)?;
        Ok(state)
    }

}

pub mod qry {
    use super::*;

    /// QUERY > Get the state of an entity's turn
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .apply( Entity::Add(100) )
    ///     .apply( |state| turn_count::cmd::count(state, 100 ) )
    ///     .unwrap();
    ///
    /// assert_eq!( turn_count::qry::count(&state, 100), 1);
    /// ```
    pub fn count(state: &State, pub_id: PubId) -> TurnCount {
        let id = entity::qry::id(state, pub_id);
        state.turn_count.get(id).unwrap_or(0) as TurnCount
    }
}