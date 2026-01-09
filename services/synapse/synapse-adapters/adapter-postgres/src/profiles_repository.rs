// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use async_trait::async_trait;
use module_profiles::http::ProfileDoc;
use sqlx::{Pool, Postgres, query};
use synapse_core::PersistenceError;
use synapse_core::domain::profiles::Profile;
use synapse_core::ports::profiles::profile_repository::{ProfilesDocStore, ProfilesRepository};

pub struct PostgresProfilesRepository {
    pool: Pool<Postgres>,
}

impl PostgresProfilesRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfilesRepository for PostgresProfilesRepository {
    async fn get_profile(&self, public_key: &str) -> Result<Option<Profile>, PersistenceError> {
        let row = sqlx::query!(
            r#"SELECT doc_bytes, created_at, updated_at FROM profiles WHERE public_key = $1"#,
            public_key
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;

        if let Some(row) = row {
            let mut doc = ProfileDoc::from_bytes(&row.doc_bytes);
            let display_name = doc.get_display_name();
            let handle = doc.get_handle();
            let bio = doc.get_bio();
            let location = doc.get_location();
            let avatar_url = doc.get_avatar_url();
            let profile = Profile {
                public_key: public_key.to_string(),
                handle: if handle.is_empty() {
                    None
                } else {
                    Some(handle)
                },
                display_name: if display_name.is_empty() {
                    None
                } else {
                    Some(display_name)
                },
                bio: if bio.is_empty() { None } else { Some(bio) },
                location: if location.is_empty() {
                    None
                } else {
                    Some(location)
                },
                avatar_url: if avatar_url.is_empty() {
                    None
                } else {
                    Some(avatar_url)
                },
                created_at: row.created_at,
                updated_at: row.updated_at,
                signature: None,
            };
            Ok(Some(profile))
        } else {
            Ok(None)
        }
    }

    async fn upsert_profile(&self, profile: &Profile) -> Result<Profile, PersistenceError> {
        // Build a minimal doc from the incoming Profile (extend as you add fields to ProfileDoc)
        let mut doc = ProfileDoc::new();
        if let Some(name) = &profile.display_name {
            doc.set_display_name(name);
        }
        let mut doc_for_save = doc;
        let bytes = doc_for_save.to_bytes();

        let row = sqlx::query!(
            r#"
            INSERT INTO profiles (public_key, doc_bytes, updated_at)
            VALUES ($1, $2, now())
            ON CONFLICT (public_key)
            DO UPDATE SET doc_bytes = EXCLUDED.doc_bytes, updated_at = now()
            RETURNING created_at, updated_at
            "#,
            profile.public_key,
            bytes
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;

        let stored = Profile {
            public_key: profile.public_key.clone(),
            handle: profile.handle.clone(),
            display_name: profile.display_name.clone(),
            bio: profile.bio.clone(),
            location: profile.location.clone(),
            avatar_url: profile.avatar_url.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
            signature: None,
        };
        Ok(stored)
    }

    async fn delete_profile(&self, public_key: &str) -> Result<(), PersistenceError> {
        sqlx::query!(r#"DELETE FROM profiles WHERE public_key = $1"#, public_key)
            .execute(&self.pool)
            .await
            .map_err(|e| PersistenceError::Other(e.to_string()))?;
        Ok(())
    }
}

pub struct PostgresProfilesDocStore {
    pool: Pool<Postgres>,
}

impl PostgresProfilesDocStore {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProfilesDocStore for PostgresProfilesDocStore {
    async fn get_doc(&self, public_key: &str) -> Result<Option<Vec<u8>>, PersistenceError> {
        let row = sqlx::query!(
            r#"SELECT doc_bytes FROM profiles WHERE public_key = $1"#,
            public_key
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;
        Ok(row.map(|r| r.doc_bytes))
    }

    async fn upsert_doc(&self, public_key: &str, doc: &[u8]) -> Result<(), PersistenceError> {
        sqlx::query!(
            r#"
            INSERT INTO profiles (public_key, doc_bytes, created_at, updated_at)
            VALUES ($1, $2, now(), now())
            ON CONFLICT (public_key)
            DO UPDATE SET doc_bytes = EXCLUDED.doc_bytes, updated_at = now()
            "#,
            public_key,
            doc
        )
        .execute(&self.pool)
        .await
        .map_err(|e| PersistenceError::Other(e.to_string()))?;
        Ok(())
    }

    async fn delete_doc(&self, public_key: &str) -> Result<(), PersistenceError> {
        sqlx::query!(r#"DELETE FROM profiles WHERE public_key = $1"#, public_key)
            .execute(&self.pool)
            .await
            .map_err(|e| PersistenceError::Other(e.to_string()))?;
        Ok(())
    }
}
