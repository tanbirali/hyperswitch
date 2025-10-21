//! Hyperswitch interface
#![warn(missing_docs, missing_debug_implementations)]

pub mod api;
<<<<<<< HEAD
/// API client interface module
pub mod api_client;
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
pub mod authentication;
/// Configuration related functionalities
pub mod configs;
/// Connector integration interface module
pub mod connector_integration_interface;
/// definition of the new connector integration trait
pub mod connector_integration_v2;
/// Constants used throughout the application
pub mod consts;
/// Conversion implementations
pub mod conversion_impls;
pub mod disputes;
pub mod encryption_interface;
pub mod errors;
<<<<<<< HEAD
/// Event handling interface
pub mod events;
/// helper utils
pub mod helpers;
=======
pub mod events;
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
/// connector integrity check interface
pub mod integrity;
pub mod metrics;
pub mod secrets_interface;
pub mod types;
<<<<<<< HEAD
/// ucs handlers
pub mod unified_connector_service;
=======
>>>>>>> 330eaee0f (chore(version): 2025.08.28.0-hotfix1)
pub mod webhooks;

/// Crm interface
pub mod crm;
