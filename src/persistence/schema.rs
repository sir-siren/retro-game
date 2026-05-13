// table and index creation SQL -- run once on first open
pub const CREATE_HIGH_SCORES_TABLE: &str = "\
    CREATE TABLE IF NOT EXISTS high_scores (\
        id        INTEGER PRIMARY KEY AUTOINCREMENT,\
        game      TEXT    NOT NULL,\
        score     INTEGER NOT NULL,\
        level     INTEGER NOT NULL DEFAULT 1,\
        played_at TEXT    NOT NULL DEFAULT (datetime('now'))\
    )";

pub const CREATE_GAME_SCORE_INDEX: &str = "\
    CREATE INDEX IF NOT EXISTS idx_game_score \
    ON high_scores(game, score DESC)";
