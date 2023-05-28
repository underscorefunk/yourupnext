use crate::prelude::*;

pub type QueryError = String;

pub type QueryResult<QueriedData> = Result<(State, QueriedData), QueryError>;

pub trait Queryable<QueryData, F> {
    fn query(self, query_fn: F) -> QueryResult<QueryData>;
}

impl<QueryData, F: Fn(&State) -> Result<QueryData, QueryError>> Queryable<QueryData, F> for State {
    /// State queries with Result<Data,QryErr> return types
    ///
    /// Queries pull data out of State. In order for queries to be chainable we
    /// need to use a consistent return type that will carry the queried State forward.
    /// The idea of "this".query() let's us keep that single piece of working state!
    /// This presents a problem though. We want our query to refer to State and not
    /// take ownership. But, if we are to pass the QryResult along for further chaining,
    /// we have to embed the owned State. Recall that the Ok value of QryResult is a
    /// tuple with (State, QueryResponse). To solve this problem we allow a query function
    /// (query_fn) to return a generic Result type with a QryResult style error "QryErr".
    /// This means that we can bubble up QryErr. It also allows us to wrap the self (State)
    /// and Ok(data) in a proper QryResult. This gives us a properly formatted QryResult
    /// with Ok( (State, data ) ) or Err (QryErr). And, it still upholds the idea of
    /// using references for queries.
    ///
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .query(|state|Ok("hooray".to_string()));
    ///
    /// assert!(state.is_ok());
    /// assert_eq!(state.unwrap(), (State::default(), "hooray".to_string()));
    /// ```
    fn query(self, query_fn: F) -> QueryResult<QueryData> {
        let response = query_fn(&self)?;
        Ok((self, response))
    }
}

pub trait ApplicableQueryResult<QueriedData> {
    fn apply_with<Applicator: Applicable, F: Fn(QueriedData) -> Applicator>(self, make_applicable: F) -> CmdResult<State>;
}

impl<QueriedData> ApplicableQueryResult<QueriedData> for QueryResult<QueriedData> {

    /// Allows a chainable command on a query which consumes the result of the query
    /// ```
    /// use yourupnext::prelude::*;
    ///
    /// let state = State::default()
    ///     .query( |state| Ok("character name") )
    ///     .apply_with( |character_name| Character::Add(100, character_name) );
    ///
    /// assert!(state.is_ok());
    /// ```
    fn apply_with<Applicator: Applicable, F: Fn(QueriedData) -> Applicator>(
        self,
        applicator_factory: F,
    ) -> CmdResult<State> {
        match self {
            Ok((state, queried_data)) => applicator_factory(queried_data).apply_to(state),
            Err(e) => CmdResult::Err(e),
        }
    }
}
