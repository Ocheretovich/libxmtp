use diesel::prelude::*;

use super::schema::topic_refresh_state;
use crate::{impl_fetch, impl_store};

#[derive(Insertable, Identifiable, Queryable, Debug, Clone)]
#[diesel(table_name = topic_refresh_state)]
#[diesel(primary_key(topic))]
pub struct TopicRefreshState {
    pub topic: String,
    pub last_message_timestamp_ns: i64,
}

impl_fetch!(TopicRefreshState, topic_refresh_state, String);
impl_store!(TopicRefreshState, topic_refresh_state);