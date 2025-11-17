use rusqlite::{Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    pub challenge_id: usize,
    pub code: String,
    pub char_count: usize,
    pub passed: bool,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalBest {
    pub challenge_id: usize,
    pub code: String,
    pub char_count: usize,
    pub beat_par: bool,
}

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn new() -> SqlResult<Self> {
        let db_path = Self::get_db_path();

        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS solutions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                challenge_id INTEGER NOT NULL,
                code TEXT NOT NULL,
                char_count INTEGER NOT NULL,
                passed INTEGER NOT NULL,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS personal_bests (
                challenge_id INTEGER PRIMARY KEY,
                code TEXT NOT NULL,
                char_count INTEGER NOT NULL,
                beat_par INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(Storage { conn })
    }

    fn get_db_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".code_golf_game");
        path.push("solutions.db");
        path
    }

    pub fn save_solution(&self, solution: &Solution) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO solutions (challenge_id, code, char_count, passed, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                solution.challenge_id,
                &solution.code,
                solution.char_count,
                if solution.passed { 1 } else { 0 },
                solution.timestamp,
            ),
        )?;

        if solution.passed {
            self.update_personal_best(solution)?;
        }

        Ok(())
    }

    fn update_personal_best(&self, solution: &Solution) -> SqlResult<()> {
        let current_best = self.get_personal_best(solution.challenge_id)?;

        let should_update = match current_best {
            None => true,
            Some(best) => solution.char_count < best.char_count,
        };

        if should_update {
            self.conn.execute(
                "INSERT OR REPLACE INTO personal_bests (challenge_id, code, char_count, beat_par)
                 VALUES (?1, ?2, ?3, ?4)",
                (
                    solution.challenge_id,
                    &solution.code,
                    solution.char_count,
                    0, // Will be updated when we know par score
                ),
            )?;
        }

        Ok(())
    }

    pub fn get_personal_best(&self, challenge_id: usize) -> SqlResult<Option<PersonalBest>> {
        let mut stmt = self.conn.prepare(
            "SELECT challenge_id, code, char_count, beat_par
             FROM personal_bests
             WHERE challenge_id = ?1"
        )?;

        let mut rows = stmt.query([challenge_id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(PersonalBest {
                challenge_id: row.get(0)?,
                code: row.get(1)?,
                char_count: row.get(2)?,
                beat_par: row.get::<_, i32>(3)? != 0,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn update_beat_par(&self, challenge_id: usize, beat_par: bool) -> SqlResult<()> {
        self.conn.execute(
            "UPDATE personal_bests SET beat_par = ?1 WHERE challenge_id = ?2",
            (if beat_par { 1 } else { 0 }, challenge_id),
        )?;
        Ok(())
    }

    pub fn get_all_personal_bests(&self) -> SqlResult<Vec<PersonalBest>> {
        let mut stmt = self.conn.prepare(
            "SELECT challenge_id, code, char_count, beat_par
             FROM personal_bests
             ORDER BY challenge_id"
        )?;

        let bests = stmt.query_map([], |row| {
            Ok(PersonalBest {
                challenge_id: row.get(0)?,
                code: row.get(1)?,
                char_count: row.get(2)?,
                beat_par: row.get::<_, i32>(3)? != 0,
            })
        })?;

        bests.collect()
    }

    pub fn get_total_score(&self) -> SqlResult<i64> {
        let bests = self.get_all_personal_bests()?;

        let score: i64 = bests.iter()
            .map(|best| {
                if best.beat_par {
                    100 // Base points for beating par
                } else {
                    0
                }
            })
            .sum();

        Ok(score)
    }
}
