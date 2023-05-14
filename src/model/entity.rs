/// # Entity Model
/// The entity model includes **Commands** and **Queries** to interact with generic
/// entities and retrieve data respectively. It is expected that this module's
/// functionality will be hidden behind a facade of more specific models.
///
/// For example, character::rename() would call entity::rename() providing
/// the opportunity to filter name values in the context of characters.

use crate::prelude::*;

/// ## Entity > Command (cmd)
/// **Commands** allow us to changes to the `State` in the context of an entity's
/// most fundamental/foundational components. All commands take an owned state
/// as the first argument and return a CommandResult with an Ok(State). This
/// allows us to guard command paths, bubbling up errors to be handled at the
/// io boundary, where the top level commands are issued. For that reason,
/// these commands are almost always called from a match branch in `Cmd::apply()`
/// which links a Cmd variant (the API command that the system that respond to)
/// with the entity::cmd (the implementation of fulfilling/applying the command
///  to State).

pub mod cmd {
    use super::*;

    /// COMMAND > Add an entity
    /// ```
    /// use yourupnext::prelude::*;
    ///     let state = entity::cmd::add( State::default(), 100).unwrap();
    ///     assert_eq!(entity::qry::id(&state,100), 1)
    /// ```
    pub fn add(state: State, pub_id: PubId) -> CommandResult<State> {
        registry::register(state, pub_id)
    }

    /// COMMAND > Remove an entity
    /// ```
    /// use yourupnext::prelude::*;
    ///     let state = entity::cmd::add( State::default(), 100).unwrap();
    ///     let removed_state = entity::cmd::remove( state, 100).unwrap();
    ///     assert_eq!(entity::qry::id(&removed_state,100), 0)
    /// ```
    pub fn remove(mut state: State, pub_id: PubId) -> CommandResult<State> {
        let id = qry::id(&state, pub_id);
        registry::deregister(state, id)
    }

    /// COMMAND > Apply a classification (type) to an entity
    /// ```
    /// use yourupnext::prelude::*;
    /// let state = entity::cmd::add( State::default(), 100).unwrap();
    /// let classified_state = entity::cmd::classify( state, 100, EntityType::Player ).unwrap();
    /// assert!(entity::qry::is(&classified_state,100, EntityType::Player));
    /// ```
    pub fn classify(state: State, entity_pub_id: PubId, entity_type: EntityType) -> CommandResult<State> {
        let entity_id = qry::id(&state, entity_pub_id);
        entity_type::cmd::classify(state, entity_id, entity_type)
    }

    /// COMMAND > Rename an entity
    /// ```
    /// use yourupnext::prelude::*;
    /// let state = entity::cmd::add( State::default(), 100).unwrap();
    /// let renamed_state = entity::cmd::name( state, 100, "AName".to_string() ).unwrap();
    /// assert_eq!(entity::qry::name(&renamed_state,100), "AName".to_string() )
    /// ```
    pub fn name(state: State, entity_pub_id: PubId, new_name: String) -> CommandResult<State> {
        let entity_id = qry::id(&state, entity_pub_id);
        name::cmd::set(state, entity_id, new_name)
    }

}

/// ## Entity > Query (qry)
/// **Queries** allow us to retrieve data related to a model without requiring the data
/// to be associated with a specific struct.

pub mod qry {
    use super::*;

    /// QUERY > Check if an entity exists
    /// ```
    /// use yourupnext::prelude::*;
    /// let state = entity::cmd::add( State::default(), 100).unwrap();
    /// assert!(entity::qry::exists(&state,100));
    /// assert!( ! entity::qry::exists(&state,1));
    /// ```
    pub fn exists(state: &State, entity_pub_id: PubId) -> bool {
        id(state, entity_pub_id) != 0
    }

    /// QUERY > Get the Id of an entity or 0 if it doesn't exist
    /// ```
    /// use yourupnext::prelude::*;
    /// let state = entity::cmd::add( State::default(), 100).unwrap();
    /// assert_eq!(entity::qry::id(&state,100), 1);
    /// assert_eq!(entity::qry::id(&state,1), 0);
    /// ```
    pub fn id(state: &State, entity_pub_id: PubId) -> Id {
        registry::id(state, entity_pub_id)
    }

    /// QUERY > Check of an entity is of a specific type
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default();
    /// assert_eq!(entity::qry::kind(&state,100), EntityType::Missing);
    ///
    /// let state = entity::cmd::add( state, 100).unwrap();
    /// assert_eq!(entity::qry::kind(&state,100), EntityType::Generic);
    ///
    /// let state = entity::cmd::classify( state, 100, EntityType::Player ).unwrap();
    /// assert_eq!(entity::qry::kind(&state,100), EntityType::Player);
    ///
    /// ```
    pub fn kind(state: &State, entity_pub_id: Id) -> EntityType {
        let entity_id = id(state, entity_pub_id);
        entity_type::qry::get(state, entity_id)
    }

    /// QUERY > Check of an entity is of a specific type
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default();
    /// assert!(entity::qry::is(&state, 100, EntityType::Missing));
    ///
    /// let state = entity::cmd::add( state, 100).unwrap();
    /// assert!(entity::qry::is(&state, 100, EntityType::Generic));
    ///
    /// let state = entity::cmd::classify( state, 100, EntityType::Player ).unwrap();
    /// assert!(entity::qry::is(&state, 100, EntityType::Player));
    ///
    /// ```
    pub fn is(state: &State, entity_pub_id: Id, entity_type: EntityType) -> bool {
        let entity_id = id(state, entity_pub_id);
        entity_type::qry::is(state, entity_id, entity_type)
    }

    /// QUERY > Get the Name of an entity or any empty string
    /// if it doesn't exist
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default();
    /// assert_eq!(entity::qry::name(&state, 100), "".to_string());
    ///
    /// let state = entity::cmd::add( state, 100).unwrap();
    /// let state = entity::cmd::name( state, 100, "Named".to_string()).unwrap();
    /// assert_eq!(entity::qry::name(&state, 100), "Named".to_string());
    /// ```
    pub fn name(state: &State, entity_pub_id: PubId) -> String {
        name::qry::get(state, entity_pub_id)
    }
}