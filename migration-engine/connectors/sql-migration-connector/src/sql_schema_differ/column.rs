use crate::{database_info::DatabaseInfo, flavour::SqlFlavour};
use prisma_value::PrismaValue;
use sql_schema_describer::{walkers::ColumnWalker, DefaultValue};

#[derive(Debug)]
pub(crate) struct ColumnDiffer<'a> {
    pub(crate) flavour: &'a dyn SqlFlavour,
    pub(crate) database_info: &'a DatabaseInfo,
    pub(crate) previous: ColumnWalker<'a>,
    pub(crate) next: ColumnWalker<'a>,
}

impl<'a> ColumnDiffer<'a> {
    pub(crate) fn name(&self) -> &'a str {
        debug_assert_eq!(self.previous.name(), self.next.name());

        self.previous.name()
    }

    pub(crate) fn differs_in_something(&self) -> bool {
        self.all_changes().iter().count() > 0
    }

    pub(crate) fn all_changes(&self) -> ColumnChanges {
        let renaming = if self.previous.name() != self.next.name() {
            Some(ColumnChange::Renaming)
        } else {
            None
        };

        let arity = if self.previous.arity() != self.next.arity() {
            Some(ColumnChange::Arity)
        } else {
            None
        };

        let r#type = if self.column_type_changed() {
            Some(ColumnChange::Type)
        } else {
            None
        };

        let default = if !self.defaults_match() {
            Some(ColumnChange::Default)
        } else {
            None
        };

        let sequence = if self.previous.is_autoincrement() != self.next.is_autoincrement() {
            Some(ColumnChange::Sequence)
        } else {
            None
        };

        ColumnChanges {
            changes: [renaming, r#type, arity, default, sequence],
        }
    }

    fn column_type_changed(&self) -> bool {
        self.flavour.column_type_changed(self)
    }

    /// There are workarounds to cope with current migration and introspection limitations.
    ///
    /// - We bail on a number of cases that are too complex to deal with right now or underspecified.
    fn defaults_match(&self) -> bool {
        // JSON defaults on MySQL should be ignored.
        if self.flavour.sql_family().is_mysql()
            && (self.previous.column_type_family().is_json() || self.next.column_type_family().is_json())
        {
            return true;
        }

        match (&self.previous.default(), &self.next.default()) {
            // Avoid naive string comparisons for JSON defaults.
            (
                Some(DefaultValue::VALUE(PrismaValue::Json(prev_json))),
                Some(DefaultValue::VALUE(PrismaValue::Json(next_json))),
            )
            | (
                Some(DefaultValue::VALUE(PrismaValue::String(prev_json))),
                Some(DefaultValue::VALUE(PrismaValue::Json(next_json))),
            )
            | (
                Some(DefaultValue::VALUE(PrismaValue::Json(prev_json))),
                Some(DefaultValue::VALUE(PrismaValue::String(next_json))),
            ) => json_defaults_match(prev_json, next_json),

            (Some(DefaultValue::VALUE(prev)), Some(DefaultValue::VALUE(next))) => prev == next,
            (Some(DefaultValue::VALUE(_)), Some(DefaultValue::NOW)) => false,
            (Some(DefaultValue::VALUE(_)), None) => false,

            (Some(DefaultValue::NOW), Some(DefaultValue::NOW)) => true,
            (Some(DefaultValue::NOW), None) => false,
            (Some(DefaultValue::NOW), Some(DefaultValue::VALUE(_))) => false,

            (Some(DefaultValue::DBGENERATED(_)), Some(DefaultValue::VALUE(_))) => false,
            (Some(DefaultValue::DBGENERATED(_)), Some(DefaultValue::NOW)) => false,
            (Some(DefaultValue::DBGENERATED(_)), None) => false,

            (Some(DefaultValue::SEQUENCE(_)), None) => false,
            (Some(DefaultValue::SEQUENCE(_)), Some(DefaultValue::VALUE(_))) => false,
            (Some(DefaultValue::SEQUENCE(_)), Some(DefaultValue::NOW)) => false,

            (None, None) => true,
            (None, Some(DefaultValue::VALUE(_))) => false,
            (None, Some(DefaultValue::NOW)) => false,

            // We can never migrate to @dbgenerated
            (_, Some(DefaultValue::DBGENERATED(_))) => true,
            // Sequence migrations are handled separately.
            (_, Some(DefaultValue::SEQUENCE(_))) => true,
        }
    }
}

fn json_defaults_match(previous: &str, next: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(previous)
        .and_then(|previous| serde_json::from_str::<serde_json::Value>(next).map(|next| (previous, next)))
        .map(|(previous, next)| previous == next)
        .unwrap_or(true)
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ColumnChange {
    Renaming,
    Arity,
    Default,
    Type,
    Sequence,
}

#[derive(Debug, Clone)]
pub(crate) struct ColumnChanges {
    changes: [Option<ColumnChange>; 5],
}

impl ColumnChanges {
    pub(crate) fn iter<'a>(&'a self) -> impl Iterator<Item = ColumnChange> + 'a {
        self.changes.iter().filter_map(|c| c.as_ref().cloned())
    }

    pub(crate) fn type_changed(&self) -> bool {
        self.changes.iter().any(|c| c.as_ref() == Some(&ColumnChange::Type))
    }

    pub(crate) fn arity_changed(&self) -> bool {
        self.changes.iter().any(|c| c.as_ref() == Some(&ColumnChange::Arity))
    }

    pub(crate) fn only_default_changed(&self) -> bool {
        matches!(self.changes, [None, None, None, Some(ColumnChange::Default), None])
    }

    pub(crate) fn only_type_changed(&self) -> bool {
        matches!(self.changes, [None, Some(ColumnChange::Type), None, None, None])
    }

    pub(crate) fn column_was_renamed(&self) -> bool {
        matches!(self.changes, [Some(ColumnChange::Renaming), _, _, _, _])
    }
}
