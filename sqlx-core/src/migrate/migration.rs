use std::borrow::Cow;

use sha2::{Digest, Sha384};

use super::MigrationType;

#[derive(Debug, Clone)]
pub struct Migration {
    pub version: i64,
    pub description: Cow<'static, str>,
    pub migration_type: MigrationType,
    pub sql: Cow<'static, str>,
    pub checksum: Cow<'static, [u8]>,
    pub alternative_checksum: Cow<'static, [u8]>,
    pub no_tx: bool,
}

impl Migration {
    pub fn new(
        version: i64,
        description: Cow<'static, str>,
        migration_type: MigrationType,
        sql: Cow<'static, str>,
        no_tx: bool,
    ) -> Self {
        let checksum = Cow::Owned(Vec::from(Sha384::digest(sql.as_bytes()).as_slice()));

        let (line1, line2) = if sql.contains("\r\n") {
            ("\r\n", "\n")
        } else {
            ("\n", "\r\n")
        };

        let alternative_checksum = Cow::Owned(Vec::from(
            Sha384::digest(sql.replace(line1, line2).as_bytes()).as_slice()));

        Migration {
            version,
            description,
            migration_type,
            sql,
            checksum,
            alternative_checksum,
            no_tx,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppliedMigration {
    pub version: i64,
    pub checksum: Cow<'static, [u8]>,
}
