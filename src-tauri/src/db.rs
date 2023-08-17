use std::path::{Path};
use chrono::Utc;
use rusqlite::{Connection, Row};
use crate::{EditableTaskData, Tag, EditableTagData, GeneratedTagData, TagId, Task, TaskId, GeneratedTaskData, FinishedTaskData, ModifiedTaskData};
use crate::my_date_time::MyDateTime;

#[derive(Debug, PartialEq)]
/// Errors that can occur during operation of the database.
pub enum DbError {
    /// Error that occurred due to underlying sqlite library.
    RusqliteError { error: rusqlite::Error, },
    /// Error that occurred due to a command to the database with an invalid tag `id`.
    TagDoesNotExistError { id: TagId, },
    /// Error that occurred due to a command to the database with an invalid task `id`.
    TaskDoesNotExistError { id: TaskId, },
    /// Error that occurred due to a command to the database that attempted to change the done
    /// status of the task with `id` to the status it is already in.
    TaskStatusError { id: TaskId, actual_status: bool }
}

impl<'a> From<rusqlite::Error> for DbError {
    fn from(value: rusqlite::Error) -> Self {
        DbError::RusqliteError { error: value }
    }
}

type DbResult<T> = Result<T, DbError>;

pub struct Db {
    conn: Connection, // note connection implements Drop
}

impl Db {
    const TAG_TABLE: &'static str = "tags";
    const TASK_TABLE: &'static str = "tasks";
    const TAG_TASK_TABLE: &'static str = "tags_tasks";

    /// Creates a database instance from either an empty/ nonexistent file or an existing database.
    pub fn new<P: AsRef<Path>>(database_file: P) -> DbResult<Self> {
        let connection = Connection::open(database_file)?;
        connection.execute(&format!(r#"
            create table if not exists {} (
                "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                "name" TEXT NOT NULL,
                "color" INTEGER NOT NULL,
                "active" INTEGER NOT NULL,
                "create_time" STRING NOT NULL
            );
        "#, Db::TAG_TABLE), ()).unwrap();
        connection.execute(&format!(r#"
            create table if not exists {} (
                "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                "title" TEXT NOT NULL,
                "body" TEXT NOT NULL,
                "difficulty" INTEGER NOT NULL,
                "create_time" STRING NOT NULL,
                "last_edit_time" STRING NOT NULL,
                "due_time" STRING,
                "target_time" STRING,
                "done_time" STRING,
                "paused" INTEGER
            );
        "#, Db::TASK_TABLE), ()).unwrap();
        connection.execute(&format!(r#"
            create table if not exists {} (
                "task_id" INTEGER NOT NULL,
                "tag_id" INTEGER NOT NULL,
                PRIMARY KEY (task_id, tag_id)
            );
        "#, Db::TAG_TASK_TABLE), ()).unwrap();
        Ok(Self {
            conn: connection
        })
    }

    // // note: see https://users.rust-lang.org/t/closure-accepting-an-iterator-as-a-parameter/77905/4
    // /// Exposes an iterator over all tags in this database, in order of insertion.
    // /// Accepts a function which you can use to do whatever with the iterator,
    // /// since the iterator can't be directly returned.
    // pub fn tags_iterator<R>(&self, iterator_op: impl FnOnce(&mut dyn Iterator<Item = Tag>) -> R) -> DbResult<R> {
    //     let mut stmt = self.conn.prepare(
    //         &format!("SELECT * FROM {}", Db::TAG_TABLE)
    //     ).unwrap();
    //     let mut iter = stmt
    //         .query_map([], Db::tag_from_row)?
    //         .map(|tag_result| tag_result.unwrap());
    //     Ok(iterator_op(&mut iter))
    // }

    /// Convenience method to retrieve all tags stored in this database
    /// in order of insertion.
    pub fn all_tags(&self) -> DbResult<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
            &format!("SELECT * FROM {}", Db::TAG_TABLE)
        ).unwrap();
        let iter = stmt
            .query_map([], Db::tag_from_row)?
            .map(|tag_result| tag_result.unwrap());
        Ok(iter.collect())
    }

    /// Add a new tag to the database, initializing:
    /// * its unique id.
    /// * its create time to now.
    ///
    /// Returns the fields generated for this tag.
    pub fn add_new_tag(&mut self, data: &EditableTagData) -> DbResult<GeneratedTagData> {
        let now = Utc::now();
        let tx = self.conn.transaction()?;
        tx.execute(&format!(
            "INSERT INTO {} (name, color, active, create_time) values (?1, ?2, ?3, ?4);", Db::TAG_TABLE
        ), (&data.name, &data.color, data.active, &now))?;
        let new_id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(GeneratedTagData {
            id: new_id,
            create_time: now,
        })
    }

    /// Retrieve the tag with this id, or `None` if this tag doesn't exist in the database.
    pub fn tag_by_id(&self, id: TagId) -> DbResult<Option<Tag>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT * FROM {} WHERE id = ?1", Db::TAG_TABLE
        )).unwrap();
        match stmt.query_row((id,), Db::tag_from_row ) {
            Ok(tag) => Ok(Some(tag)),
            Err(e) => match e {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                other => Err(DbError::from(other)),
            }
        }
    }

    /// Modifies an existing tag in the database.
    ///
    /// # Failure
    /// Returns `DbError::TagDoesNotExistError` if the tag being modified doesn't exist in the database.
    pub fn modify_tag(&mut self, id: TagId, modify: &EditableTagData) -> DbResult<()> {
        let tx = self.conn.transaction()?;
        let rows = tx.execute(&format!(r#"
                UPDATE {} SET
                    name = ?2,
                    color = ?3,
                    active = ?4
                WHERE id = ?1;
            "#, Db::TAG_TABLE), (id, &modify.name, &modify.color, modify.active))?;
        tx.commit()?;
        match rows {
            0 => Err(DbError::TagDoesNotExistError { id }),
            1 => Ok(()),
            other => panic!("Modify tag changed {} rows!", other),
        }
    }

    /// Delete a tag by its id in the database (and removes it from any tasks that have this tag).
    ///
    /// # Failure
    /// Returns `DbError::TagDoesNotExistError` if the tag being deleted doesn't exist in the database.
    pub fn delete_tag(&mut self, id: TagId) -> DbResult<()> {
        let tx = self.conn.transaction()?;
        let rows = tx.execute(&format!(
            "DELETE FROM {} WHERE id = ?1", {Db::TAG_TABLE}
        ), (id,))?;
        if rows == 0 {
            return Err(DbError::TagDoesNotExistError { id });
        } else if rows > 1 {
            panic!("Delete tag changed {} rows!", rows);
        }
        tx.execute(&format!(
            "DELETE FROM {} WHERE tag_id = ?1",
            Db::TAG_TASK_TABLE
        ), (id,))?;

        tx.commit()?;
        Ok(())
    }

    /// Applies a filter function to the tags in this database, returning
    /// the tags that pass it.
    pub fn filter_tags<P>(&self, predicate: P) -> DbResult<Vec<Tag>>
        where P: Fn(&Tag) -> bool {
        let mut stmt = self.conn.prepare(
            &format!("SELECT * FROM {}", Db::TAG_TABLE)
        ).unwrap();
        let iter = stmt
            .query_map([], Db::tag_from_row)?
            .map(|tag_result| tag_result.unwrap())
            .filter(predicate);
        Ok(iter.collect())
    }

    /// Convenience method to retrieves all tasks stored in this database
    /// in order of insertion.
    pub fn all_tasks(&self) -> DbResult<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            &format!("SELECT * FROM {}", Db::TASK_TABLE)
        ).unwrap();
        let iter = stmt.query_map([], |row| self.task_from_row(row))?;
        Ok(iter.map(|task| task.unwrap()).collect())
    }

    /// Add a new tag to the database, initializing:
    /// * Its id
    /// * Its create time to now
    /// * Its last edit time to now
    /// * Its done time to None
    ///
    /// Returns the fields generated for this task.
    pub fn add_new_task(&mut self, data: &EditableTaskData) -> DbResult<GeneratedTaskData> {
        if let Some(tag_id) = data.tag {
            if self.tag_by_id(tag_id).unwrap().is_none() {
                return Err(DbError::TagDoesNotExistError {id: tag_id});
            }
        }

        let now = Utc::now();
        let tx = self.conn.transaction()?;
        tx.execute(&format!(r#"
            INSERT INTO {}
            (title, body, difficulty, create_time, last_edit_time, due_time, target_time, paused) values
            (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);
        "#, Db::TASK_TABLE), (&data.title, &data.body, data.difficulty, &now, &now, &data.due_time, &data.target_time, data.paused))?;
        let new_id = tx.last_insert_rowid();
        if let Some(tag) = data.tag {
            tx.execute(&format!(
                "INSERT INTO {} (task_id, tag_id) values (?1, ?2);", Db::TAG_TASK_TABLE
            ), (new_id, tag))?;
        }
        tx.commit()?;
        Ok(GeneratedTaskData {
            id: new_id,
            create_time: now.clone(),
            last_edit_time: now,
            done_time: None,
        })
    }

    /// Retrieve the task with this id, or `None` if the task doesn't exist in the database.
    pub fn task_by_id(&self, id: TaskId) -> DbResult<Option<Task>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT * FROM {} WHERE id = ?1", {Db::TASK_TABLE}
        )).unwrap();
        match stmt.query_row((id,), |row| self.task_from_row(row)) {
            Ok(task) => Ok(Some(task)),
            Err(e) => match e {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                other => Err(DbError::from(other))
            }
        }
    }

    /// Modifies an existing task in the database, updating the last edit time to now.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task being modified doesn't exist in the database.
    /// Returns `DbError::TagDoesNotExistError` if attempted to add a tag that doesn't exist.
    pub fn modify_task(&mut self, id: TaskId, data: &EditableTaskData) -> DbResult<ModifiedTaskData> {
        if let Some(tag_id) = data.tag {
            if self.tag_by_id(tag_id).unwrap().is_none() {
                return Err(DbError::TagDoesNotExistError {id: tag_id});
            }
        }

        let now = Utc::now();
        let tx = self.conn.transaction()?;
        let rows = tx.execute(&format!(r#"
                UPDATE {} SET
                    title = ?2,
                    body = ?3,
                    difficulty = ?4,
                    last_edit_time = ?5,
                    due_time = ?6,
                    target_time = ?7,
                    paused = ?8
                WHERE id = ?1;
        "#, Db::TASK_TABLE),
      (id, &data.title, &data.body, data.difficulty, &now, &data.due_time, &data.target_time, data.paused))?;

        if rows == 0 {
            return Err(DbError::TaskDoesNotExistError { id });
        } else if rows > 1 {
            panic!("Modify task changed {} rows!", rows);
        }

        tx.execute(&format!(
            "DELETE FROM {} WHERE task_id = ?1", Db::TAG_TASK_TABLE
        ), (id,))?;

        if let Some(tag_id) = data.tag {
            tx.execute(&format!(r#"
                    INSERT INTO {} (task_id, tag_id) values (?1, ?2)
                    ON CONFLICT (task_id, tag_id) DO NOTHING;
                "#, Db::TAG_TASK_TABLE
            ), (id, tag_id))?;
        }
        tx.commit()?;

        Ok(ModifiedTaskData {
            last_edit_time: now,
        })
    }

    /// Delete a task by its id in the database.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task being deleted doesn't exist in the database.
    pub fn delete_task(&mut self, id: TaskId) -> DbResult<()> {
        let tx = self.conn.transaction()?;
        let rows = tx.execute(&format!(
            "DELETE FROM {} WHERE id = ?1;", Db::TASK_TABLE
        ), (id,))?;
        if rows == 0 {
            return Err(DbError::TaskDoesNotExistError { id });
        } else if rows > 1 {
            panic!("Delete task changed {} rows!", rows);
        }
        tx.execute(&format!(
            "DELETE FROM {} WHERE task_id = ?1;", Db::TAG_TASK_TABLE
        ), (id,))?;
        tx.commit()?;
        Ok(())
    }

    /// Mark a task as done, updating the done time of this task. Returns the new done time.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task doesn't exist in the database.
    /// Returns `DbError::TaskStatusError` if the task is already finished.
    pub fn finish_task(&mut self, id: TaskId) -> DbResult<FinishedTaskData> {
        let task = match self.task_by_id(id)? {
            Some(task) => task,
            None => return Err(DbError::TaskDoesNotExistError {id}),
        };
        if task.done_time.is_some() {
            return Err(DbError::TaskStatusError { id, actual_status: true });
        }
        let done_time = Some(Utc::now());
        let tx = self.conn.transaction()?;
        tx.execute(&format!(
            "UPDATE {} SET done_time = ?2 WHERE id = ?1;", Db::TASK_TABLE
        ), (id, &done_time))?;
        tx.commit()?;
        Ok(FinishedTaskData {
            done_time,
        })
    }

    /// Mark a task as not done, updating the done time of this task. Returns the new done time.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task doesn't exist in the database.
    /// Returns `DbError::TaskStatusError` if the task is already not finished.
    pub fn unfinish_task(&mut self, id: TaskId) -> DbResult<FinishedTaskData> {
        let task = match self.task_by_id(id)? {
            Some(task) => task,
            None => return Err(DbError::TaskDoesNotExistError {id}),
        };
        if task.done_time.is_none() {
            return Err(DbError::TaskStatusError { id, actual_status: false });
        }
        let done_time = None;
        let tx = self.conn.transaction()?;
        tx.execute(&format!(
            "UPDATE {} SET done_time = ?2 WHERE id = ?1;", Db::TASK_TABLE
        ), (id, &done_time))?;
        tx.commit()?;
        Ok(FinishedTaskData {
            done_time,
        })
    }

    /// Applies a filter function to the tasks in this database, returning
    /// the tasks that pass it.
    pub fn filter_tasks<P>(&self, predicate: P) -> DbResult<Vec<Task>>
        where P: Fn(&Task) -> bool {
        let mut stmt = self.conn.prepare(
            &format!("SELECT * FROM {}", Db::TASK_TABLE)
        ).unwrap();
        let iter = stmt
            .query_map([], |row| self.task_from_row(row))?
            .map(|tag_result| tag_result.unwrap())
            .filter(predicate);
        Ok(iter.collect())
    }

    fn tag_from_row(row: &Row) -> rusqlite::Result<Tag> {
        Ok(Tag {
            id: row.get("id")?,
            name: row.get("name")?,
            color: row.get("color")?,
            active: row.get("active")?,
            create_time: row.get("create_time")?,
        })
    }

    fn get_task_tag(&self, id: TaskId) -> Option<TagId> {
        let mut stmt = self.conn.prepare(
            &format!("SELECT tag_id FROM {} WHERE task_id = ?1", {Db::TAG_TASK_TABLE})
        ).unwrap();
        let tag_id_result: rusqlite::Result<TagId> = stmt.query_row(
            [id], |result| Ok(result.get("tag_id")?)
        );
        tag_id_result.ok()
    }

    fn task_from_row(&self, row: &Row) -> rusqlite::Result<Task> {
        let id = row.get("id")?;
        Ok(Task {
            id,
            title: row.get("title")?,
            tag: self.get_task_tag(id),
            body: row.get("body")?,
            difficulty: row.get("difficulty")?,
            create_time: row.get("create_time")?,
            last_edit_time: row.get("last_edit_time")?,
            due_time: row.get("due_time")?,
            target_time: row.get("target_time")?,
            done_time: row.get("done_time")?,
            paused: row.get("paused")?,
        })
    }

}