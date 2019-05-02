use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    entry::Entry,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    validation::EntryValidationData
};

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Pos {
	x: u32,
	y: u32
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Move {
	game: Address,
	author: Address,
	move_type: String,
	from: Pos,
	to: Pos,
	previous_move: Address
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "move",
        description: "A move by an agent in an game",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | validation_data: hdk::EntryValidationData<Move>| {
            match validation_data {
                EntryValidationData::Create{entry, validation_data} => {
                    Ok(())
                },
                _ => {
                    Err("Cannot modify or delete a move".into())
                }
            }
        }
    )
}