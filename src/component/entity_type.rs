use crate::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EntityType {
    Player,
    Character,
    Item,
    Location,
    Effect,
    Generic,
    Missing,
}

impl EntityType {
    fn label(self) -> String {
        match self {
            EntityType::Player => "Player".to_string(),
            EntityType::Character => "Character".to_string(),
            EntityType::Item => "Item".to_string(),
            EntityType::Location => "Location".to_string(),
            EntityType::Effect => "Effect".to_string(),
            EntityType::Generic => "Generic".to_string(),
            EntityType::Missing => "Missing".to_string(),
        }
    }
}

/// ## Entity_Type > Command (cmd)

pub mod cmd {
    use super::*;

    /// COMMAND > Set the type of an entity. Attempting to set the `Missing` variant
    /// will cause an Error.
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id = 100;
    ///
    /// let state = State::default();
    /// let state = entity::cmd::add( state, pub_id).unwrap();
    /// let state = entity_type::cmd::classify( state, pub_id, EntityType::Player ).unwrap();
    /// assert_eq!(entity_type::qry::get(&state, pub_id), EntityType::Player);
    /// ```
    pub fn classify(mut state: State,pub_id: PubId,entity_type: EntityType) -> CmdResult<State> {
        let id = entity::qry::id(&state, pub_id);
        if entity_type == EntityType::Missing {
            return Err("Can not manually classify entities as Missing entity type".to_string() );
        }
        state.entity_type.insert(id, entity_type)?;
        Ok(state)
    }
}


/// ## Entity_Type > Query (qry)

pub mod qry {
    use super::*;

    /// QUERY > Check if the type classification of an entity is a specific type
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let pub_id = 100;
    ///
    /// let state = State::default();
    /// assert!(entity_type::qry::is(&state, pub_id, EntityType::Missing));
    ///
    /// let state = entity::cmd::add( state, 100).unwrap();
    /// assert!(entity_type::qry::is(&state, pub_id, EntityType::Generic));
    ///
    /// let state = entity_type::cmd::classify( state, pub_id, EntityType::Player ).unwrap();
    /// assert_eq!(entity_type::qry::get(&state,pub_id), EntityType::Player);
    ///
    /// ```
    pub fn is(state: &State, pub_id: PubId, entity_type: EntityType) -> bool {
        get(state, pub_id) == entity_type
    }

    /// Query > Check if the type classification of an entity is a specific type
    ///
    /// This query uses Id instead of Pub_Id as the entity identifier because it is
    /// an entity component.
    ///
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default();
    /// let pub_id = 100;
    ///
    /// // A missing entity is of type `Missing`
    /// assert_eq!(entity_type::qry::get(&state,pub_id), EntityType::Missing);
    ///
    /// let state = entity::cmd::add( state, pub_id).unwrap();
    ///
    /// // An entity exist but is untyped resulting in `Generic`
    /// assert_eq!(entity_type::qry::get(&state,pub_id), EntityType::Generic);
    ///
    /// let state = entity_type::cmd::classify( state, pub_id, EntityType::Player ).unwrap();
    ///
    /// // A classified entity is of it's classification type.
    /// assert_eq!(entity_type::qry::get(&state,pub_id), EntityType::Player);
    ///
    /// let state = entity_type::cmd::classify( state, pub_id, EntityType::Missing );
    ///
    /// // An entity can not be classified as missing
    /// assert!(state.is_err());
    ///
    /// ```
    pub fn get(state: &State, pub_id: PubId) -> EntityType {
        let id = entity::qry::id(state, pub_id);
        if !state.registry.has_id(&id) || id == 0 {
            return EntityType::Missing;
        }
        match state.entity_type.get(id) {
            Some(entity_type) => entity_type,
            _ => EntityType::Generic
        }
    }
}