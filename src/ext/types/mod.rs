use tokio::time::Duration;
use typed_builder::TypedBuilder;

pub mod lang_schema;

#[derive(TypedBuilder)]
pub struct MetaInfo {
    pub time: Duration,
}
