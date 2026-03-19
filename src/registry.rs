use std::path::PathBuf;

use appimage_pkg::data_dir;
use rusqlite::{Connection, OptionalExtension, params};
use slug::slugify;
use time::OffsetDateTime;

#[derive(Debug)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub source_path: PathBuf,
    pub desktop_path: Option<PathBuf>,
    pub icon_path: Option<PathBuf>,
    pub installed_at: OffsetDateTime,
}

pub struct Registry {
    conn: Connection,
}

#[derive(Debug)]
pub enum RegistryError {
    DatabaseError(rusqlite::Error),
    IoError(std::io::Error),
    PackageNotFound,
    PackageAlreadyExists,
    ConfigDirNotFound,
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            RegistryError::IoError(e) => write!(f, "IO error: {}", e),
            RegistryError::PackageNotFound => write!(f, "Package not found"),
            RegistryError::PackageAlreadyExists => write!(f, "Package already exists"),
            RegistryError::ConfigDirNotFound => {
                write!(f, "Could not determine config directory")
            }
        }
    }
}

impl std::error::Error for RegistryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RegistryError::DatabaseError(e) => Some(e),
            RegistryError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for RegistryError {
    fn from(err: rusqlite::Error) -> Self {
        RegistryError::DatabaseError(err)
    }
}

impl From<std::io::Error> for RegistryError {
    fn from(err: std::io::Error) -> Self {
        RegistryError::IoError(err)
    }
}

impl Package {
    pub fn generate_id(name: &str) -> String {
        slugify(name)
    }
}

impl Registry {
    pub fn add(&self, pkg: &Package) -> Result<(), RegistryError> {
        let result = self.conn.execute(
            "INSERT INTO packages (id, name, version, source_path, desktop_path, icon_path, installed_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                pkg.id,
                pkg.name,
                pkg.version,
                pkg.source_path.to_string_lossy(),
                pkg.desktop_path.as_ref().map(|p| p.to_string_lossy().to_string()),
                pkg.icon_path.as_ref().map(|p| p.to_string_lossy().to_string()),
                pkg.installed_at,
            ],
        );

        match result {
            Ok(_) => Ok(()),
            Err(rusqlite::Error::SqliteFailure(err, _))
                if err.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_PRIMARYKEY =>
            {
                Err(RegistryError::PackageAlreadyExists)
            }
            Err(err) => Err(RegistryError::DatabaseError(err)),
        }
    }

    pub fn remove(&self, id: &str) -> Result<(), RegistryError> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM packages WHERE id = ?1", params![id])?;

        if rows_affected == 0 {
            return Err(RegistryError::PackageNotFound);
        }

        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<Option<Package>, RegistryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, version, source_path, desktop_path, icon_path, installed_at
             FROM packages WHERE id = ?1",
        )?;

        let pkg = stmt.query_row(params![id], row_to_package).optional()?;

        Ok(pkg)
    }

    pub fn get_all(&self) -> Result<Vec<Package>, RegistryError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, version, source_path, desktop_path, icon_path, installed_at
             FROM packages ORDER BY id",
        )?;

        let packages = stmt.query_map([], row_to_package)?;

        Ok(packages.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn query(&self, query: &str) -> Result<Vec<Package>, RegistryError> {
        let search_pattern = format!("%{}%", query);

        let mut stmt = self.conn.prepare(
            "SELECT id, name, version, source_path, desktop_path, icon_path, installed_at
             FROM packages WHERE name LIKE ?1 COLLATE NOCASE ORDER BY id",
        )?;

        let packages = stmt.query_map(params![search_pattern], row_to_package)?;

        Ok(packages.collect::<Result<Vec<_>, _>>()?)
    }
}

fn row_to_package(row: &rusqlite::Row) -> Result<Package, rusqlite::Error> {
    Ok(Package {
        id: row.get(0)?,
        name: row.get(1)?,
        version: row.get(2)?,
        source_path: PathBuf::from(row.get::<_, String>(3)?),
        desktop_path: row.get::<_, Option<String>>(4)?.map(PathBuf::from),
        icon_path: row.get::<_, Option<String>>(5)?.map(PathBuf::from),
        installed_at: row.get(6)?,
    })
}

pub fn load() -> Result<Registry, RegistryError> {
    let data_dir = data_dir().ok_or(RegistryError::ConfigDirNotFound)?;
    let app_dir = data_dir.join("appimage-pkg");
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("packages.db");
    let conn = Connection::open(&db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS packages (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT,
                source_path TEXT NOT NULL,
                desktop_path TEXT,
                icon_path TEXT,
                installed_at TEXT NOT NULL
            )",
        [],
    )?;

    Ok(Registry { conn })
}
