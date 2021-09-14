use async_graphql::Object;

pub struct Mutations;

#[Object]
impl Mutations {
    async fn signup(&self, id: uuid::Uuid) -> uuid::Uuid {
        //let res = models::record::Record::id().await;
        //res.expect("")
        id
    }
}
