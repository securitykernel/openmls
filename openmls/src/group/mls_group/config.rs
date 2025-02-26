//! Configuration module for [`MlsGroup`] configurations.
//!
//! ## Building an MlsGroupConfig
//! The [`MlsGroupConfigBuilder`] makes it easy to build configurations for the
//! [`MlsGroup`].
//!
//! ```
//! use openmls::prelude::*;
//!
//! let group_config = MlsGroupConfig::builder()
//!     .use_ratchet_tree_extension(true)
//!     .build();
//! ```
//!
//! See [`MlsGroupConfigBuilder`](MlsGroupConfigBuilder#implementations) for
//! all options that can be configured.
//!
//! ### Wire format policies
//! Only some combination of possible wire formats are valid within OpenMLS.
//! The [`WIRE_FORMAT_POLICIES`] lists all valid options that can be set.
//!
//! ```
//! use openmls::prelude::*;
//!
//! let group_config = MlsGroupConfig::builder()
//!     .wire_format_policy(MIXED_CIPHERTEXT_WIRE_FORMAT_POLICY)
//!     .build();
//! ```

use super::*;
use crate::{
    group::config::CryptoConfig, key_packages::Lifetime,
    tree::sender_ratchet::SenderRatchetConfiguration,
};
use serde::{Deserialize, Serialize};

/// Specifies the configuration parameters for a [`MlsGroup`]. Refer to
/// the [User Manual](https://openmls.tech/book/user_manual/group_config.html) for more information about the different configuration values.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MlsGroupConfig {
    /// Defines the wire format policy for outgoing and incoming handshake messages.
    /// Application are always encrypted regardless.
    pub(crate) wire_format_policy: WireFormatPolicy,
    /// Size of padding in bytes
    pub(crate) padding_size: usize,
    /// Maximum number of past epochs for which application messages
    /// can be decrypted. The default is 0.
    pub(crate) max_past_epochs: usize,
    /// Number of resumtion secrets to keep
    pub(crate) number_of_resumption_psks: usize,
    /// Flag to indicate the Ratchet Tree Extension should be used
    pub(crate) use_ratchet_tree_extension: bool,
    /// Required capabilities (extensions and proposal types)
    pub(crate) required_capabilities: RequiredCapabilitiesExtension,
    /// Senders authorized to send external remove proposals
    pub(crate) external_senders: ExternalSendersExtension,
    /// Sender ratchet configuration
    pub(crate) sender_ratchet_configuration: SenderRatchetConfiguration,
    /// Lifetime of the own leaf node
    pub(crate) lifetime: Lifetime,
    /// Ciphersuite and protocol version
    pub(crate) crypto_config: CryptoConfig,
}

impl MlsGroupConfig {
    /// Returns a builder for [`MlsGroupConfig`]
    pub fn builder() -> MlsGroupConfigBuilder {
        MlsGroupConfigBuilder::new()
    }

    /// Returns the [`MlsGroupConfig`] wire format policy.
    pub fn wire_format_policy(&self) -> WireFormatPolicy {
        self.wire_format_policy
    }

    /// Returns the [`MlsGroupConfig`] padding size.
    pub fn padding_size(&self) -> usize {
        self.padding_size
    }

    /// Returns the [`MlsGroupConfig`] max past epochs.
    pub fn max_past_epochs(&self) -> usize {
        self.max_past_epochs
    }

    /// Returns the [`MlsGroupConfig`] number of resumption psks.
    pub fn number_of_resumption_psks(&self) -> usize {
        self.number_of_resumption_psks
    }

    /// Returns the [`MlsGroupConfig`] boolean flag that indicates whether ratchet_tree_extension should be used.
    pub fn use_ratchet_tree_extension(&self) -> bool {
        self.use_ratchet_tree_extension
    }

    /// Returns the [`MlsGroupConfig`] required capabilities extension.
    pub fn required_capabilities(&self) -> &RequiredCapabilitiesExtension {
        &self.required_capabilities
    }

    /// Returns the [`MlsGroupConfig`] sender ratchet configuration.
    pub fn sender_ratchet_configuration(&self) -> &SenderRatchetConfiguration {
        &self.sender_ratchet_configuration
    }

    /// Returns the [`MlsGroupConfig`] external senders extension
    pub fn external_senders(&self) -> &ExternalSendersExtension {
        &self.external_senders
    }

    /// Returns the [`MlsGroupConfig`] lifetime configuration.
    pub fn lifetime(&self) -> &Lifetime {
        &self.lifetime
    }

    /// Returns the [`CryptoConfig`].
    pub fn crypto_config(&self) -> &CryptoConfig {
        &self.crypto_config
    }

    #[cfg(any(feature = "test-utils", test))]
    pub fn test_default(ciphersuite: Ciphersuite) -> Self {
        Self::builder()
            .wire_format_policy(WireFormatPolicy::new(
                OutgoingWireFormatPolicy::AlwaysPlaintext,
                IncomingWireFormatPolicy::Mixed,
            ))
            .crypto_config(CryptoConfig::with_default_version(ciphersuite))
            .build()
    }
}

/// Builder for an [`MlsGroupConfig`].
#[derive(Default)]
pub struct MlsGroupConfigBuilder {
    config: MlsGroupConfig,
}

impl MlsGroupConfigBuilder {
    /// Creates a new builder with default values.
    pub fn new() -> Self {
        MlsGroupConfigBuilder {
            config: MlsGroupConfig::default(),
        }
    }

    /// Sets the `wire_format` property of the MlsGroupConfig.
    pub fn wire_format_policy(mut self, wire_format_policy: WireFormatPolicy) -> Self {
        self.config.wire_format_policy = wire_format_policy;
        self
    }

    /// Sets the `padding_size` property of the MlsGroupConfig.
    pub fn padding_size(mut self, padding_size: usize) -> Self {
        self.config.padding_size = padding_size;
        self
    }

    /// Sets the `max_past_epochs` property of the MlsGroupConfig.
    /// This allows application messages from previous epochs to be decrypted.
    ///
    /// **WARNING**
    ///
    /// This feature enables the storage of message secrets from past epochs.
    /// It is a trade-off between functionality and forward secrecy and should only be enabled
    /// if the Delivery Service cannot guarantee that application messages will be sent in
    /// the same epoch in which they were generated. The number for `max_epochs` should be
    /// as low as possible.
    pub fn max_past_epochs(mut self, max_past_epochs: usize) -> Self {
        self.config.max_past_epochs = max_past_epochs;
        self
    }

    /// Sets the `number_of_resumption_psks` property of the MlsGroupConfig.
    pub fn number_of_resumption_psks(mut self, number_of_resumption_psks: usize) -> Self {
        self.config.number_of_resumption_psks = number_of_resumption_psks;
        self
    }

    /// Sets the `use_ratchet_tree_extension` property of the MlsGroupConfig.
    pub fn use_ratchet_tree_extension(mut self, use_ratchet_tree_extension: bool) -> Self {
        self.config.use_ratchet_tree_extension = use_ratchet_tree_extension;
        self
    }

    /// Sets the `required_capabilities` property of the MlsGroupConfig.
    pub fn required_capabilities(
        mut self,
        required_capabilities: RequiredCapabilitiesExtension,
    ) -> Self {
        self.config.required_capabilities = required_capabilities;
        self
    }

    /// Sets the `sender_ratchet_configuration` property of the MlsGroupConfig.
    /// See [`SenderRatchetConfiguration`] for more information.
    pub fn sender_ratchet_configuration(
        mut self,
        sender_ratchet_configuration: SenderRatchetConfiguration,
    ) -> Self {
        self.config.sender_ratchet_configuration = sender_ratchet_configuration;
        self
    }

    /// Sets the `lifetime` property of the MlsGroupConfig.
    pub fn lifetime(mut self, lifetime: Lifetime) -> Self {
        self.config.lifetime = lifetime;
        self
    }

    /// Sets the `crypto_config` property of the MlsGroupConfig.
    pub fn crypto_config(mut self, config: CryptoConfig) -> Self {
        self.config.crypto_config = config;
        self
    }

    /// Sets the `external_senders` property of the MlsGroupConfig.
    pub fn external_senders(mut self, external_senders: ExternalSendersExtension) -> Self {
        self.config.external_senders = external_senders;
        self
    }

    /// Finalizes the builder and retursn an `[MlsGroupConfig`].
    pub fn build(self) -> MlsGroupConfig {
        self.config
    }
}

/// Defines what wire format is acceptable for incoming handshake messages.
/// Note that application messages must always be encrypted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncomingWireFormatPolicy {
    /// Handshake messages must always be PrivateMessage
    AlwaysCiphertext,
    /// Handshake messages must always be PublicMessage
    AlwaysPlaintext,
    /// Handshake messages can either be PrivateMessage or PublicMessage
    Mixed,
}

impl IncomingWireFormatPolicy {
    pub(crate) fn is_compatible_with(&self, wire_format: WireFormat) -> bool {
        match self {
            IncomingWireFormatPolicy::AlwaysCiphertext => wire_format == WireFormat::PrivateMessage,
            IncomingWireFormatPolicy::AlwaysPlaintext => wire_format == WireFormat::PublicMessage,
            IncomingWireFormatPolicy::Mixed => {
                wire_format == WireFormat::PrivateMessage
                    || wire_format == WireFormat::PublicMessage
            }
        }
    }
}

/// Defines what wire format should be used for outgoing handshake messages.
/// Note that application messages must always be encrypted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutgoingWireFormatPolicy {
    /// Handshake messages must always be PrivateMessage
    AlwaysCiphertext,
    /// Handshake messages must always be PublicMessage
    AlwaysPlaintext,
}

/// Defines what wire format is desired for outgoing handshake messages.
/// Note that application messages must always be encrypted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WireFormatPolicy {
    outgoing: OutgoingWireFormatPolicy,
    incoming: IncomingWireFormatPolicy,
}

impl WireFormatPolicy {
    /// Creates a new wire format policy from an [`OutgoingWireFormatPolicy`]
    /// and an [`IncomingWireFormatPolicy`].
    #[cfg(any(feature = "test-utils", test))]
    pub(crate) fn new(
        outgoing: OutgoingWireFormatPolicy,
        incoming: IncomingWireFormatPolicy,
    ) -> Self {
        Self { outgoing, incoming }
    }

    /// Returns a reference to the wire format policy's outgoing wire format policy.
    pub fn outgoing(&self) -> OutgoingWireFormatPolicy {
        self.outgoing
    }

    /// Returns a reference to the wire format policy's incoming wire format policy.
    pub fn incoming(&self) -> IncomingWireFormatPolicy {
        self.incoming
    }
}

impl Default for WireFormatPolicy {
    fn default() -> Self {
        PURE_CIPHERTEXT_WIRE_FORMAT_POLICY
    }
}

impl From<OutgoingWireFormatPolicy> for WireFormat {
    fn from(outgoing: OutgoingWireFormatPolicy) -> Self {
        match outgoing {
            OutgoingWireFormatPolicy::AlwaysCiphertext => WireFormat::PrivateMessage,
            OutgoingWireFormatPolicy::AlwaysPlaintext => WireFormat::PublicMessage,
        }
    }
}

/// All valid wire format policy combinations.
/// - [`PURE_PLAINTEXT_WIRE_FORMAT_POLICY`]
/// - [`PURE_CIPHERTEXT_WIRE_FORMAT_POLICY`]
/// - [`MIXED_PLAINTEXT_WIRE_FORMAT_POLICY`]
/// - [`MIXED_CIPHERTEXT_WIRE_FORMAT_POLICY`]
pub const WIRE_FORMAT_POLICIES: [WireFormatPolicy; 4] = [
    PURE_PLAINTEXT_WIRE_FORMAT_POLICY,
    PURE_CIPHERTEXT_WIRE_FORMAT_POLICY,
    MIXED_PLAINTEXT_WIRE_FORMAT_POLICY,
    MIXED_CIPHERTEXT_WIRE_FORMAT_POLICY,
];

/// Incoming and outgoing wire formats are always plaintext.
pub const PURE_PLAINTEXT_WIRE_FORMAT_POLICY: WireFormatPolicy = WireFormatPolicy {
    outgoing: OutgoingWireFormatPolicy::AlwaysPlaintext,
    incoming: IncomingWireFormatPolicy::AlwaysPlaintext,
};

/// Incoming and outgoing wire formats are always ciphertext.
pub const PURE_CIPHERTEXT_WIRE_FORMAT_POLICY: WireFormatPolicy = WireFormatPolicy {
    outgoing: OutgoingWireFormatPolicy::AlwaysCiphertext,
    incoming: IncomingWireFormatPolicy::AlwaysCiphertext,
};

/// Incoming wire formats can be mixed while outgoing wire formats are always
/// plaintext.
pub const MIXED_PLAINTEXT_WIRE_FORMAT_POLICY: WireFormatPolicy = WireFormatPolicy {
    outgoing: OutgoingWireFormatPolicy::AlwaysPlaintext,
    incoming: IncomingWireFormatPolicy::Mixed,
};

/// Incoming wire formats can be mixed while outgoing wire formats are always
/// ciphertext.
pub const MIXED_CIPHERTEXT_WIRE_FORMAT_POLICY: WireFormatPolicy = WireFormatPolicy {
    outgoing: OutgoingWireFormatPolicy::AlwaysCiphertext,
    incoming: IncomingWireFormatPolicy::Mixed,
};
