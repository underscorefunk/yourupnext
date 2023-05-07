#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    pub fn add_character() {
        let commands = vec![
            Cmd::AddCharacter(1337, "Original Name".to_string())
        ];
        let state = Cmd::apply_all(commands, State::default()).unwrap();

        // Entity is added to the registry
        let id = state.registry.pub_dict.get(&1337).unwrap().to_owned();
        assert_eq!(id, 1 as Id);

        // Public ID matches what we stored to associated PublicID with the internal Id (entity_id)
        let pub_id = state.registry.id_dict.get(&1).unwrap().to_owned();
        assert_eq!(pub_id, 1337 as PubId);

        // Characters should be of entity type Character
        let entity_type = state.entity_type.get(id).unwrap();
        assert_eq!(entity_type, EntityType::Character);

        // Character should be named with their initial provided name
        let name = state.name.get(id).unwrap();
        assert_eq!(name, "Original Name".to_string());
    }

    #[test]
    pub fn rename_character() {
        let commands = vec![
            Cmd::AddCharacter(1337, "Original Name".to_string()),
            Cmd::RenameCharacter(1337, "Renamed Name".to_string()),
        ];
        let state = Cmd::apply_all(commands, State::default()).unwrap();

        let id = state.registry.pub_dict.get(&1337).unwrap().to_owned();
        let name = state.name.get(id).unwrap();

        assert_eq!(name, "Renamed Name".to_string());
    }

    #[test]
    pub fn remove_character() {
        let commands = vec![
            Cmd::AddCharacter(1337, "Original Name".to_string()),
            Cmd::RemoveCharacter(1337),
        ];
        let state = Cmd::apply_all(commands, State::default()).unwrap();

        let id = state.registry.pub_dict.get(&1337);
        assert!(id.is_none());
    }
}