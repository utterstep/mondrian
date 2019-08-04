use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    schema::{task_assignments, tasks},
    users::User,
};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Associations, Identifiable)]
#[belongs_to(User, foreign_key = "author_id")]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub author_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[belongs_to(User)]
#[belongs_to(Task)]
pub struct TaskAssignment {
    pub id: i32,
    pub task_id: i32,
    pub user_id: Option<i32>,
    pub start_time: NaiveDateTime,
    pub duration: i32,
    pub completed: bool,
    pub urgent: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
