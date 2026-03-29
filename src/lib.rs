// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! wowasticker lib. ai (f119,f120,f134,f137,f138), db (f121–f128,f135,f136,f140–f146), audio (f129,f130), report (f147).
//! P13: compressed identifiers per kova convention.

#![allow(non_camel_case_types, non_snake_case)]

pub mod ai;
pub mod audio;
pub mod db;
#[cfg(feature = "jni")]
pub mod jni;
pub mod report;
#[cfg(feature = "wasm")]
pub mod wasm;
