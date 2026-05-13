use std::path::PathBuf;

use rusqlite::{Connection, params};

use crate::persistence::schema;
use crate::types::error::AppError;

/// Single row from the `high_scores` table.
#[derive(Debug, Clone)]
pub struct ScoreEntry {
    pub game: String,
    pub score: u32,
    pub level: u8,
    pub played_at: String,
}

/// Thin wrapper around a `SQLite` connection for high-score persistence.
pub struct ScoreDb {
    conn: Connection,
}

impl ScoreDb {
    /// Open (or create) the database file at the platform data directory.
    ///
    /// # Errors
    ///
    /// Returns an error when the data directory cannot be created or `SQLite`
    /// cannot open or initialize the database.
    pub fn open() -> Result<Self, AppError> {
        let path = db_path()?;
        let conn = Connection::open(&path).map_err(|e| AppError::Database {
            reason: format!("failed to open {}: {e}", path.display()),
        })?;

        conn.execute_batch(schema::CREATE_HIGH_SCORES_TABLE)
            .map_err(|e| AppError::Database {
                reason: format!("table creation failed: {e}"),
            })?;

        conn.execute_batch(schema::CREATE_GAME_SCORE_INDEX)
            .map_err(|e| AppError::Database {
                reason: format!("index creation failed: {e}"),
            })?;

        Ok(Self { conn })
    }

    /// Persist a score after a game-over.
    ///
    /// # Errors
    ///
    /// Returns an error when the insert fails.
    pub fn save_score(&self, game: &str, score: u32, level: u8) -> Result<(), AppError> {
        self.conn
            .execute(
                "INSERT INTO high_scores (game, score, level) VALUES (?1, ?2, ?3)",
                params![game, score, level],
            )
            .map_err(|e| AppError::Database {
                reason: format!("insert failed: {e}"),
            })?;
        Ok(())
    }

    /// Return the all-time best score for a game, or 0 if none recorded.
    ///
    /// # Errors
    ///
    /// Returns an error when the query fails.
    pub fn high_score(&self, game: &str) -> Result<u32, AppError> {
        let result: Result<u32, _> = self.conn.query_row(
            "SELECT COALESCE(MAX(score), 0) FROM high_scores WHERE game = ?1",
            params![game],
            |row| row.get(0),
        );
        result.map_err(|e| AppError::Database {
            reason: format!("high score query failed: {e}"),
        })
    }

    /// Return the top N scores for a game, newest first when tied.
    ///
    /// # Errors
    ///
    /// Returns an error when preparing, querying, or reading rows fails.
    pub fn top_scores(&self, game: &str, limit: usize) -> Result<Vec<ScoreEntry>, AppError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT game, score, level, played_at \
                 FROM high_scores WHERE game = ?1 \
                 ORDER BY score DESC, played_at DESC LIMIT ?2",
            )
            .map_err(|e| AppError::Database {
                reason: format!("prepare failed: {e}"),
            })?;

        let rows = stmt
            .query_map(params![game, limit], |row| {
                Ok(ScoreEntry {
                    game: row.get(0)?,
                    score: row.get(1)?,
                    level: row.get(2)?,
                    played_at: row.get(3)?,
                })
            })
            .map_err(|e| AppError::Database {
                reason: format!("query failed: {e}"),
            })?;

        let mut entries = Vec::with_capacity(limit);
        for row in rows {
            let entry = row.map_err(|e| AppError::Database {
                reason: format!("row read failed: {e}"),
            })?;
            entries.push(entry);
        }
        Ok(entries)
    }
}

/// Platform-aware path: ~/.local/share/terminal-arcade/arcade.db (linux)
/// or %APPDATA%\terminal-arcade\arcade.db (windows).
fn db_path() -> Result<PathBuf, AppError> {
    let base = dirs::data_dir().ok_or_else(|| AppError::Database {
        reason: "could not determine data directory".into(),
    })?;
    let dir = base.join("terminal-arcade");
    std::fs::create_dir_all(&dir).map_err(|e| AppError::Database {
        reason: format!("could not create {}: {e}", dir.display()),
    })?;
    Ok(dir.join("arcade.db"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory_db() -> ScoreDb {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(schema::CREATE_HIGH_SCORES_TABLE)
            .unwrap();
        conn.execute_batch(schema::CREATE_GAME_SCORE_INDEX).unwrap();
        ScoreDb { conn }
    }

    #[test]
    fn high_score_returns_zero_when_no_scores_exist() {
        let db = in_memory_db();
        let result = db.high_score("snake").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn save_and_retrieve_high_score() {
        let db = in_memory_db();
        db.save_score("runner", 500, 3).unwrap();
        db.save_score("runner", 1200, 5).unwrap();
        db.save_score("runner", 800, 4).unwrap();

        let best = db.high_score("runner").unwrap();
        assert_eq!(best, 1200);
    }

    #[test]
    fn top_scores_returns_ordered_entries() {
        let db = in_memory_db();
        db.save_score("bricks", 100, 1).unwrap();
        db.save_score("bricks", 300, 2).unwrap();
        db.save_score("bricks", 200, 1).unwrap();

        let top = db.top_scores("bricks", 2).unwrap();
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].score, 300);
        assert_eq!(top[1].score, 200);
    }

    #[test]
    fn scores_are_isolated_per_game() {
        let db = in_memory_db();
        db.save_score("snake", 999, 5).unwrap();
        db.save_score("dino", 50, 1).unwrap();

        assert_eq!(db.high_score("snake").unwrap(), 999);
        assert_eq!(db.high_score("dino").unwrap(), 50);
        assert_eq!(db.high_score("runner").unwrap(), 0);
    }
}
