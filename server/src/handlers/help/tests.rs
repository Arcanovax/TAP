use super::*;
use std::collections::HashSet;
use strum::IntoEnumIterator;

/// Désérialise le payload JSON de HELP en `(command, description)`.
fn help_entries() -> Vec<(String, String)> {
    match help_request() {
        Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Json(value),
        } => value
            .as_array()
            .expect("le payload HELP doit être un tableau JSON")
            .iter()
            .map(|entry| {
                (
                    entry["command"].as_str().unwrap().to_string(),
                    entry["description"].as_str().unwrap().to_string(),
                )
            })
            .collect(),
        other => panic!("réponse HELP inattendue: {other:?}"),
    }
}

#[test]
fn help_returns_one_entry_per_command() {
    assert_eq!(help_entries().len(), Command::iter().count());
}

#[test]
fn help_covers_every_command_name() {
    let names: HashSet<String> = help_entries().into_iter().map(|(c, _)| c).collect();
    for command in Command::iter() {
        assert!(
            names.contains(&format!("{command:?}")),
            "commande absente du HELP: {command:?}"
        );
    }
}

#[test]
fn help_includes_itself() {
    let names: Vec<String> = help_entries().into_iter().map(|(c, _)| c).collect();
    assert!(names.contains(&"HELP".to_string()));
}

#[test]
fn every_description_is_non_empty() {
    for (command, description) in help_entries() {
        assert!(!description.is_empty(), "description vide pour {command}");
    }
}

#[test]
fn descriptions_are_all_distinct() {
    let entries = help_entries();
    let distinct: HashSet<&String> = entries.iter().map(|(_, d)| d).collect();
    assert_eq!(
        distinct.len(),
        entries.len(),
        "au moins deux commandes partagent la même description"
    );
}

#[test]
fn command_parses_help_case_insensitively() {
    assert_eq!(Command::parse("HELP"), Some(Command::HELP));
    assert_eq!(Command::parse("help"), Some(Command::HELP));
}
