// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0
//! Names of modules, functions, and types used by Diem System.

use diem_types::account_config;
use move_core_types::{ident_str, identifier::{IdentStr, Identifier}, language_storage::ModuleId};
use once_cell::sync::Lazy;

// Data to resolve basic account and transaction flow functions and structs
/// The ModuleId for the diem writeset manager module
/// The ModuleId for the diem block module
pub static DIEM_BLOCK_MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        account_config::CORE_CODE_ADDRESS,
        ident_str!("DiemBlock").to_owned(),
    )
});

//////// 0L ////////
// Oracle module
pub static ORACLE_MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        account_config::CORE_CODE_ADDRESS,
        ORACLE_MODULE_NAME.clone(),
    )
});
pub static UPGRADE_MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        account_config::CORE_CODE_ADDRESS,
        UPGRADE_MODULE_NAME.clone(),
    )
});

pub static DIEMCONFIG_MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        account_config::CORE_CODE_ADDRESS,
        DIEMCONFIG_MODULE_NAME.clone(),
    )
});

// Oracles
static ORACLE_MODULE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("Oracle").unwrap());
pub static CHECK_UPGRADE: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("check_upgrade").unwrap());
static UPGRADE_MODULE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("Upgrade").unwrap());

static DIEMCONFIG_MODULE_NAME: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("DiemConfig").unwrap());

pub static RESET_PAYLOAD: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("reset_payload").unwrap());

pub static UPGRADE_RECONFIG: Lazy<Identifier> =
    Lazy::new(|| Identifier::new("upgrade_reconfig").unwrap());
//////// 0L end ////////    

// Names for special functions and structs
pub const SCRIPT_PROLOGUE_NAME: &IdentStr = ident_str!("script_prologue");
pub const MULTI_AGENT_SCRIPT_PROLOGUE_NAME: &IdentStr = ident_str!("multi_agent_script_prologue");
pub const MODULE_PROLOGUE_NAME: &IdentStr = ident_str!("module_prologue");
pub const WRITESET_PROLOGUE_NAME: &IdentStr = ident_str!("writeset_prologue");
pub const WRITESET_EPILOGUE_NAME: &IdentStr = ident_str!("writeset_epilogue");
pub const USER_EPILOGUE_NAME: &IdentStr = ident_str!("epilogue");
pub const BLOCK_PROLOGUE: &IdentStr = ident_str!("block_prologue");
