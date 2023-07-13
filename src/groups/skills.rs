#![allow(unused)]

use crate::prelude::*;

/// Endpoints for Skills
pub struct SkillsGroup<'a> {
    pub(crate) esi: &'a Esi,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct CharacterSkills {
    pub skills: Vec<Skill>,
    pub total_sp: i64,
    pub unallocated_sp: i64,
}

#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Skill {
    pub active_skill_level: i8,
    pub skill_id: i32,
    pub skillpoints_in_skill: i64,
    pub trained_skill_level: i8,
}

impl<'a> SkillsGroup<'a> {
    api_get!(
        /// get a character's known skills
        get_skills,
        "get_characters_character_id_skills",
        RequestType::Authenticated,
        CharacterSkills,
        (character_id: u64) => "{character_id}"
    );
}
