use juniper::GraphQLObject;
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

#[derive(GraphQLObject)]
#[graphql(description = "A subset of the Keycloak REST UserRepresentation")]
struct User {
    id: String,
    username: String,
    first_name: String,
    last_name: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn user(_id: String) -> FieldResult<User> {
        Ok(User {
            id: "test".to_owned(),
            username: "test".to_owned(),
            first_name: "test".to_owned(),
            last_name: "test".to_owned(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
