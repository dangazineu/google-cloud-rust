// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/// A [Secret][google.cloud.secretmanager.v1.Secret] is a logical secret whose
/// value and versions can be accessed.
///
/// A [Secret][google.cloud.secretmanager.v1.Secret] is made up of zero or more
/// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] that represent
/// the secret data.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Secret {
    /// Output only. The resource name of the
    /// [Secret][google.cloud.secretmanager.v1.Secret] in the format
    /// `projects/*/secrets/*`.
    pub name: String,

    /// Optional. Immutable. The replication policy of the secret data attached to
    /// the [Secret][google.cloud.secretmanager.v1.Secret].
    ///
    /// The replication policy cannot be changed after the Secret has been created.
    pub replication: Option<crate::model::Replication>,

    /// Output only. The time at which the
    /// [Secret][google.cloud.secretmanager.v1.Secret] was created.
    pub create_time: Option<wkt::Timestamp>,

    /// The labels assigned to this Secret.
    ///
    /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
    /// of maximum 128 bytes, and must conform to the following PCRE regular
    /// expression: `[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}`
    ///
    /// Label values must be between 0 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, and must conform to the following PCRE
    /// regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`
    ///
    /// No more than 64 labels can be assigned to a given resource.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub labels: std::collections::HashMap<String, String>,

    /// Optional. A list of up to 10 Pub/Sub topics to which messages are published
    /// when control plane operations are called on the secret or its versions.
    pub topics: Vec<crate::model::Topic>,

    /// Optional. Etag of the currently stored
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    pub etag: String,

    /// Optional. Rotation policy attached to the
    /// [Secret][google.cloud.secretmanager.v1.Secret]. May be excluded if there is
    /// no rotation policy.
    pub rotation: Option<crate::model::Rotation>,

    /// Optional. Mapping from version alias to version name.
    ///
    /// A version alias is a string with a maximum length of 63 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`)
    /// and underscore ('_') characters. An alias string must start with a
    /// letter and cannot be the string 'latest' or 'NEW'.
    /// No more than 50 aliases can be assigned to a given secret.
    ///
    /// Version-Alias pairs will be viewable via GetSecret and modifiable via
    /// UpdateSecret. Access by alias is only be supported on
    /// GetSecretVersion and AccessSecretVersion.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde_as(as = "std::collections::HashMap<_, serde_with::DisplayFromStr>")]
    pub version_aliases: std::collections::HashMap<String, i64>,

    /// Optional. Custom metadata about the secret.
    ///
    /// Annotations are distinct from various forms of labels.
    /// Annotations exist to allow client tools to store their own state
    /// information without requiring a database.
    ///
    /// Annotation keys must be between 1 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, begin and end with an alphanumeric character
    /// ([a-z0-9A-Z]), and may have dashes (-), underscores (_), dots (.), and
    /// alphanumerics in between these symbols.
    ///
    /// The total size of annotation keys and values must be less than 16KiB.
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub annotations: std::collections::HashMap<String, String>,

    /// Optional. Secret Version TTL after destruction request
    ///
    /// This is a part of the Delayed secret version destroy feature.
    /// For secret with TTL>0, version destruction doesn't happen immediately
    /// on calling destroy instead the version goes to a disabled state and
    /// destruction happens after the TTL expires.
    pub version_destroy_ttl: Option<wkt::Duration>,

    /// Optional. The customer-managed encryption configuration of the Regionalised
    /// Secrets. If no configuration is provided, Google-managed default encryption
    /// is used.
    ///
    /// Updates to the [Secret][google.cloud.secretmanager.v1.Secret] encryption
    /// configuration only apply to
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] added
    /// afterwards. They do not apply retroactively to existing
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
    pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryption>,

    /// Expiration policy attached to the
    /// [Secret][google.cloud.secretmanager.v1.Secret]. If specified the
    /// [Secret][google.cloud.secretmanager.v1.Secret] and all
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] will be
    /// automatically deleted at expiration. Expired secrets are irreversibly
    /// deleted.
    ///
    /// Expiration is *not* the recommended way to set time-based permissions. [IAM
    /// Conditions](https://cloud.google.com/secret-manager/docs/access-control#conditions)
    /// is recommended for granting time-based permissions because the operation
    /// can be reversed.
    pub expiration: Option<crate::model::secret::Expiration>,
}

impl Secret {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `replication`.
    pub fn set_replication<T: Into<Option<crate::model::Replication>>>(mut self, v: T) -> Self {
        self.replication = v.into();
        self
    }

    /// Sets the value of `create_time`.
    pub fn set_create_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of `labels`.
    pub fn set_labels<T: Into<std::collections::HashMap<String, String>>>(mut self, v: T) -> Self {
        self.labels = v.into();
        self
    }

    /// Sets the value of `topics`.
    pub fn set_topics<T: Into<Vec<crate::model::Topic>>>(mut self, v: T) -> Self {
        self.topics = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of `rotation`.
    pub fn set_rotation<T: Into<Option<crate::model::Rotation>>>(mut self, v: T) -> Self {
        self.rotation = v.into();
        self
    }

    /// Sets the value of `version_aliases`.
    pub fn set_version_aliases<T: Into<std::collections::HashMap<String, i64>>>(
        mut self,
        v: T,
    ) -> Self {
        self.version_aliases = v.into();
        self
    }

    /// Sets the value of `annotations`.
    pub fn set_annotations<T: Into<std::collections::HashMap<String, String>>>(
        mut self,
        v: T,
    ) -> Self {
        self.annotations = v.into();
        self
    }

    /// Sets the value of `version_destroy_ttl`.
    pub fn set_version_destroy_ttl<T: Into<Option<wkt::Duration>>>(mut self, v: T) -> Self {
        self.version_destroy_ttl = v.into();
        self
    }

    /// Sets the value of `customer_managed_encryption`.
    pub fn set_customer_managed_encryption<
        T: Into<Option<crate::model::CustomerManagedEncryption>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.customer_managed_encryption = v.into();
        self
    }

    /// Sets the value of `expiration`.
    pub fn set_expiration<T: Into<Option<crate::model::secret::Expiration>>>(
        mut self,
        v: T,
    ) -> Self {
        self.expiration = v.into();
        self
    }
}

/// Defines additional types related to Secret
pub mod secret {

    /// Expiration policy attached to the
    /// [Secret][google.cloud.secretmanager.v1.Secret]. If specified the
    /// [Secret][google.cloud.secretmanager.v1.Secret] and all
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] will be
    /// automatically deleted at expiration. Expired secrets are irreversibly
    /// deleted.
    ///
    /// Expiration is *not* the recommended way to set time-based permissions. [IAM
    /// Conditions](https://cloud.google.com/secret-manager/docs/access-control#conditions)
    /// is recommended for granting time-based permissions because the operation
    /// can be reversed.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase", untagged)]
    #[non_exhaustive]
    pub enum Expiration {
        /// Optional. Timestamp in UTC when the
        /// [Secret][google.cloud.secretmanager.v1.Secret] is scheduled to expire.
        /// This is always provided on output, regardless of what was sent on input.
        ExpireTime(wkt::Timestamp),
        /// Input only. The TTL for the
        /// [Secret][google.cloud.secretmanager.v1.Secret].
        Ttl(wkt::Duration),
    }
}

/// A secret version resource in the Secret Manager API.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SecretVersion {
    /// Output only. The resource name of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*`.
    ///
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] IDs in a
    /// [Secret][google.cloud.secretmanager.v1.Secret] start at 1 and are
    /// incremented for each subsequent version of the secret.
    pub name: String,

    /// Output only. The time at which the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] was created.
    pub create_time: Option<wkt::Timestamp>,

    /// Output only. The time this
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] was destroyed.
    /// Only present if [state][google.cloud.secretmanager.v1.SecretVersion.state]
    /// is
    /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED].
    pub destroy_time: Option<wkt::Timestamp>,

    /// Output only. The current state of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub state: crate::model::secret_version::State,

    /// The replication status of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub replication_status: Option<crate::model::ReplicationStatus>,

    /// Output only. Etag of the currently stored
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub etag: String,

    /// Output only. True if payload checksum specified in
    /// [SecretPayload][google.cloud.secretmanager.v1.SecretPayload] object has
    /// been received by
    /// [SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService]
    /// on
    /// [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion].
    pub client_specified_payload_checksum: bool,

    /// Optional. Output only. Scheduled destroy time for secret version.
    /// This is a part of the Delayed secret version destroy feature. For a
    /// Secret with a valid version destroy TTL, when a secert version is
    /// destroyed, the version is moved to disabled state and it is scheduled for
    /// destruction. The version is destroyed only after the
    /// `scheduled_destroy_time`.
    pub scheduled_destroy_time: Option<wkt::Timestamp>,

    /// Output only. The customer-managed encryption status of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. Only
    /// populated if customer-managed encryption is used and
    /// [Secret][google.cloud.secretmanager.v1.Secret] is a Regionalised Secret.
    pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryptionStatus>,
}

impl SecretVersion {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `create_time`.
    pub fn set_create_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of `destroy_time`.
    pub fn set_destroy_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.destroy_time = v.into();
        self
    }

    /// Sets the value of `state`.
    pub fn set_state<T: Into<crate::model::secret_version::State>>(mut self, v: T) -> Self {
        self.state = v.into();
        self
    }

    /// Sets the value of `replication_status`.
    pub fn set_replication_status<T: Into<Option<crate::model::ReplicationStatus>>>(
        mut self,
        v: T,
    ) -> Self {
        self.replication_status = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of `client_specified_payload_checksum`.
    pub fn set_client_specified_payload_checksum<T: Into<bool>>(mut self, v: T) -> Self {
        self.client_specified_payload_checksum = v.into();
        self
    }

    /// Sets the value of `scheduled_destroy_time`.
    pub fn set_scheduled_destroy_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.scheduled_destroy_time = v.into();
        self
    }

    /// Sets the value of `customer_managed_encryption`.
    pub fn set_customer_managed_encryption<
        T: Into<Option<crate::model::CustomerManagedEncryptionStatus>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.customer_managed_encryption = v.into();
        self
    }
}

/// Defines additional types related to SecretVersion
pub mod secret_version {

    /// The state of a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion], indicating if
    /// it can be accessed.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct State(String);

    impl State {
        /// Sets the enum value.
        pub fn set_value<T: Into<String>>(mut self, v: T) -> Self {
            self.0 = v.into();
            self
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [State](State)
    pub mod state {

        /// Not specified. This value is unused and invalid.
        pub const STATE_UNSPECIFIED: &str = "STATE_UNSPECIFIED";

        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may be
        /// accessed.
        pub const ENABLED: &str = "ENABLED";

        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may not
        /// be accessed, but the secret data is still available and can be placed
        /// back into the
        /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]
        /// state.
        pub const DISABLED: &str = "DISABLED";

        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] is
        /// destroyed and the secret data is no longer stored. A version may not
        /// leave this state once entered.
        pub const DESTROYED: &str = "DESTROYED";
    }
}

/// A policy that defines the replication and encryption configuration of data.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Replication {
    /// The replication policy for this secret.
    pub replication: Option<crate::model::replication::Replication>,
}

impl Replication {
    /// Sets the value of `replication`.
    pub fn set_replication<T: Into<Option<crate::model::replication::Replication>>>(
        mut self,
        v: T,
    ) -> Self {
        self.replication = v.into();
        self
    }
}

/// Defines additional types related to Replication
pub mod replication {

    /// A replication policy that replicates the
    /// [Secret][google.cloud.secretmanager.v1.Secret] payload without any
    /// restrictions.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct Automatic {
        /// Optional. The customer-managed encryption configuration of the
        /// [Secret][google.cloud.secretmanager.v1.Secret]. If no configuration is
        /// provided, Google-managed default encryption is used.
        ///
        /// Updates to the [Secret][google.cloud.secretmanager.v1.Secret] encryption
        /// configuration only apply to
        /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] added
        /// afterwards. They do not apply retroactively to existing
        /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
        pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryption>,
    }

    impl Automatic {
        /// Sets the value of `customer_managed_encryption`.
        pub fn set_customer_managed_encryption<
            T: Into<Option<crate::model::CustomerManagedEncryption>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.customer_managed_encryption = v.into();
            self
        }
    }

    /// A replication policy that replicates the
    /// [Secret][google.cloud.secretmanager.v1.Secret] payload into the locations
    /// specified in [Secret.replication.user_managed.replicas][]
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct UserManaged {
        /// Required. The list of Replicas for this
        /// [Secret][google.cloud.secretmanager.v1.Secret].
        ///
        /// Cannot be empty.
        pub replicas: Vec<crate::model::replication::user_managed::Replica>,
    }

    impl UserManaged {
        /// Sets the value of `replicas`.
        pub fn set_replicas<T: Into<Vec<crate::model::replication::user_managed::Replica>>>(
            mut self,
            v: T,
        ) -> Self {
            self.replicas = v.into();
            self
        }
    }

    /// Defines additional types related to UserManaged
    pub mod user_managed {

        /// Represents a Replica for this
        /// [Secret][google.cloud.secretmanager.v1.Secret].
        #[serde_with::serde_as]
        #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        #[non_exhaustive]
        pub struct Replica {
            /// The canonical IDs of the location to replicate data.
            /// For example: `"us-east1"`.
            pub location: String,

            /// Optional. The customer-managed encryption configuration of the
            /// [User-Managed Replica][Replication.UserManaged.Replica]. If no
            /// configuration is provided, Google-managed default encryption is used.
            ///
            /// Updates to the [Secret][google.cloud.secretmanager.v1.Secret]
            /// encryption configuration only apply to
            /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] added
            /// afterwards. They do not apply retroactively to existing
            /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
            pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryption>,
        }

        impl Replica {
            /// Sets the value of `location`.
            pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
                self.location = v.into();
                self
            }

            /// Sets the value of `customer_managed_encryption`.
            pub fn set_customer_managed_encryption<
                T: Into<Option<crate::model::CustomerManagedEncryption>>,
            >(
                mut self,
                v: T,
            ) -> Self {
                self.customer_managed_encryption = v.into();
                self
            }
        }
    }

    /// The replication policy for this secret.
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase", untagged)]
    #[non_exhaustive]
    pub enum Replication {
        /// The [Secret][google.cloud.secretmanager.v1.Secret] will automatically be
        /// replicated without any restrictions.
        Automatic(crate::model::replication::Automatic),
        /// The [Secret][google.cloud.secretmanager.v1.Secret] will only be
        /// replicated into the locations specified.
        UserManaged(crate::model::replication::UserManaged),
    }
}

/// Configuration for encrypting secret payloads using customer-managed
/// encryption keys (CMEK).
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CustomerManagedEncryption {
    /// Required. The resource name of the Cloud KMS CryptoKey used to encrypt
    /// secret payloads.
    ///
    /// For secrets using the
    /// [UserManaged][google.cloud.secretmanager.v1.Replication.UserManaged]
    /// replication policy type, Cloud KMS CryptoKeys must reside in the same
    /// location as the [replica location][Secret.UserManaged.Replica.location].
    ///
    /// For secrets using the
    /// [Automatic][google.cloud.secretmanager.v1.Replication.Automatic]
    /// replication policy type, Cloud KMS CryptoKeys must reside in `global`.
    ///
    /// The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    pub kms_key_name: String,
}

impl CustomerManagedEncryption {
    /// Sets the value of `kms_key_name`.
    pub fn set_kms_key_name<T: Into<String>>(mut self, v: T) -> Self {
        self.kms_key_name = v.into();
        self
    }
}

/// The replication status of a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReplicationStatus {
    /// The replication status of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub replication_status: Option<crate::model::replication_status::ReplicationStatus>,
}

impl ReplicationStatus {
    /// Sets the value of `replication_status`.
    pub fn set_replication_status<
        T: Into<Option<crate::model::replication_status::ReplicationStatus>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.replication_status = v.into();
        self
    }
}

/// Defines additional types related to ReplicationStatus
pub mod replication_status {

    /// The replication status of a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] using
    /// automatic replication.
    ///
    /// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret]
    /// has an automatic replication policy.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct AutomaticStatus {
        /// Output only. The customer-managed encryption status of the
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. Only
        /// populated if customer-managed encryption is used.
        pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryptionStatus>,
    }

    impl AutomaticStatus {
        /// Sets the value of `customer_managed_encryption`.
        pub fn set_customer_managed_encryption<
            T: Into<Option<crate::model::CustomerManagedEncryptionStatus>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.customer_managed_encryption = v.into();
            self
        }
    }

    /// The replication status of a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] using
    /// user-managed replication.
    ///
    /// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret]
    /// has a user-managed replication policy.
    #[serde_with::serde_as]
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    #[non_exhaustive]
    pub struct UserManagedStatus {
        /// Output only. The list of replica statuses for the
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        pub replicas: Vec<crate::model::replication_status::user_managed_status::ReplicaStatus>,
    }

    impl UserManagedStatus {
        /// Sets the value of `replicas`.
        pub fn set_replicas<
            T: Into<Vec<crate::model::replication_status::user_managed_status::ReplicaStatus>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.replicas = v.into();
            self
        }
    }

    /// Defines additional types related to UserManagedStatus
    pub mod user_managed_status {

        /// Describes the status of a user-managed replica for the
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        #[serde_with::serde_as]
        #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        #[non_exhaustive]
        pub struct ReplicaStatus {
            /// Output only. The canonical ID of the replica location.
            /// For example: `"us-east1"`.
            pub location: String,

            /// Output only. The customer-managed encryption status of the
            /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. Only
            /// populated if customer-managed encryption is used.
            pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryptionStatus>,
        }

        impl ReplicaStatus {
            /// Sets the value of `location`.
            pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
                self.location = v.into();
                self
            }

            /// Sets the value of `customer_managed_encryption`.
            pub fn set_customer_managed_encryption<
                T: Into<Option<crate::model::CustomerManagedEncryptionStatus>>,
            >(
                mut self,
                v: T,
            ) -> Self {
                self.customer_managed_encryption = v.into();
                self
            }
        }
    }

    /// The replication status of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(rename_all = "camelCase", untagged)]
    #[non_exhaustive]
    pub enum ReplicationStatus {
        /// Describes the replication status of a
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] with
        /// automatic replication.
        ///
        /// Only populated if the parent
        /// [Secret][google.cloud.secretmanager.v1.Secret] has an automatic
        /// replication policy.
        Automatic(crate::model::replication_status::AutomaticStatus),
        /// Describes the replication status of a
        /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] with
        /// user-managed replication.
        ///
        /// Only populated if the parent
        /// [Secret][google.cloud.secretmanager.v1.Secret] has a user-managed
        /// replication policy.
        UserManaged(crate::model::replication_status::UserManagedStatus),
    }
}

/// Describes the status of customer-managed encryption.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CustomerManagedEncryptionStatus {
    /// Required. The resource name of the Cloud KMS CryptoKeyVersion used to
    /// encrypt the secret payload, in the following format:
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/versions/*`.
    pub kms_key_version_name: String,
}

impl CustomerManagedEncryptionStatus {
    /// Sets the value of `kms_key_version_name`.
    pub fn set_kms_key_version_name<T: Into<String>>(mut self, v: T) -> Self {
        self.kms_key_version_name = v.into();
        self
    }
}

/// A Pub/Sub topic which Secret Manager will publish to when control plane
/// events occur on this secret.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Topic {
    /// Required. The resource name of the Pub/Sub topic that will be published to,
    /// in the following format: `projects/*/topics/*`. For publication to succeed,
    /// the Secret Manager service agent must have the `pubsub.topic.publish`
    /// permission on the topic. The Pub/Sub Publisher role
    /// (`roles/pubsub.publisher`) includes this permission.
    pub name: String,
}

impl Topic {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

/// The rotation time and period for a
/// [Secret][google.cloud.secretmanager.v1.Secret]. At next_rotation_time, Secret
/// Manager will send a Pub/Sub notification to the topics configured on the
/// Secret. [Secret.topics][google.cloud.secretmanager.v1.Secret.topics] must be
/// set to configure rotation.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Rotation {
    /// Optional. Timestamp in UTC at which the
    /// [Secret][google.cloud.secretmanager.v1.Secret] is scheduled to rotate.
    /// Cannot be set to less than 300s (5 min) in the future and at most
    /// 3153600000s (100 years).
    ///
    /// [next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time]
    /// MUST  be set if
    /// [rotation_period][google.cloud.secretmanager.v1.Rotation.rotation_period]
    /// is set.
    pub next_rotation_time: Option<wkt::Timestamp>,

    /// Input only. The Duration between rotation notifications. Must be in seconds
    /// and at least 3600s (1h) and at most 3153600000s (100 years).
    ///
    /// If
    /// [rotation_period][google.cloud.secretmanager.v1.Rotation.rotation_period]
    /// is set,
    /// [next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time]
    /// must be set.
    /// [next_rotation_time][google.cloud.secretmanager.v1.Rotation.next_rotation_time]
    /// will be advanced by this period when the service automatically sends
    /// rotation notifications.
    pub rotation_period: Option<wkt::Duration>,
}

impl Rotation {
    /// Sets the value of `next_rotation_time`.
    pub fn set_next_rotation_time<T: Into<Option<wkt::Timestamp>>>(mut self, v: T) -> Self {
        self.next_rotation_time = v.into();
        self
    }

    /// Sets the value of `rotation_period`.
    pub fn set_rotation_period<T: Into<Option<wkt::Duration>>>(mut self, v: T) -> Self {
        self.rotation_period = v.into();
        self
    }
}

/// A secret payload resource in the Secret Manager API. This contains the
/// sensitive secret payload that is associated with a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SecretPayload {
    /// The secret data. Must be no larger than 64KiB.
    #[serde_as(as = "serde_with::base64::Base64")]
    pub data: bytes::Bytes,

    /// Optional. If specified,
    /// [SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService]
    /// will verify the integrity of the received
    /// [data][google.cloud.secretmanager.v1.SecretPayload.data] on
    /// [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion]
    /// calls using the crc32c checksum and store it to include in future
    /// [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion]
    /// responses. If a checksum is not provided in the
    /// [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion]
    /// request, the
    /// [SecretManagerService][google.cloud.secretmanager.v1.SecretManagerService]
    /// will generate and store one for you.
    ///
    /// The CRC32C value is encoded as a Int64 for compatibility, and can be
    /// safely downconverted to uint32 in languages that support this type.
    /// https://cloud.google.com/apis/design/design_patterns#integer_types
    #[serde_as(as = "Option<serde_with::DisplayFromStr>")]
    pub data_crc32c: Option<i64>,
}

impl SecretPayload {
    /// Sets the value of `data`.
    pub fn set_data<T: Into<bytes::Bytes>>(mut self, v: T) -> Self {
        self.data = v.into();
        self
    }

    /// Sets the value of `data_crc32c`.
    pub fn set_data_crc32c<T: Into<Option<i64>>>(mut self, v: T) -> Self {
        self.data_crc32c = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretsRequest {
    /// Required. The resource name of the project associated with the
    /// [Secrets][google.cloud.secretmanager.v1.Secret], in the format `projects/*`
    /// or `projects/*/locations/*`
    pub parent: String,

    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    pub page_size: i32,

    /// Optional. Pagination token, returned earlier via
    /// [ListSecretsResponse.next_page_token][google.cloud.secretmanager.v1.ListSecretsResponse.next_page_token].
    pub page_token: String,

    /// Optional. Filter string, adhering to the rules in
    /// [List-operation
    /// filtering](https://cloud.google.com/secret-manager/docs/filtering). List
    /// only secrets matching the filter. If filter is empty, all secrets are
    /// listed.
    pub filter: String,
}

impl ListSecretsRequest {
    /// Sets the value of `parent`.
    pub fn set_parent<T: Into<String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

/// Response message for
/// [SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretsResponse {
    /// The list of [Secrets][google.cloud.secretmanager.v1.Secret] sorted in
    /// reverse by create_time (newest first).
    pub secrets: Vec<crate::model::Secret>,

    /// A token to retrieve the next page of results. Pass this value in
    /// [ListSecretsRequest.page_token][google.cloud.secretmanager.v1.ListSecretsRequest.page_token]
    /// to retrieve the next page.
    pub next_page_token: String,

    /// The total number of [Secrets][google.cloud.secretmanager.v1.Secret] but 0
    /// when the
    /// [ListSecretsRequest.filter][google.cloud.secretmanager.v1.ListSecretsRequest.filter]
    /// field is set.
    pub total_size: i32,
}

impl ListSecretsResponse {
    /// Sets the value of `secrets`.
    pub fn set_secrets<T: Into<Vec<crate::model::Secret>>>(mut self, v: T) -> Self {
        self.secrets = v.into();
        self
    }

    /// Sets the value of `next_page_token`.
    pub fn set_next_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of `total_size`.
    pub fn set_total_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.total_size = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.CreateSecret][google.cloud.secretmanager.v1.SecretManagerService.CreateSecret].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CreateSecretRequest {
    /// Required. The resource name of the project to associate with the
    /// [Secret][google.cloud.secretmanager.v1.Secret], in the format `projects/*`
    /// or `projects/*/locations/*`.
    pub parent: String,

    /// Required. This must be unique within the project.
    ///
    /// A secret ID is a string with a maximum length of 255 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and
    /// underscore (`_`) characters.
    pub secret_id: String,

    /// Required. A [Secret][google.cloud.secretmanager.v1.Secret] with initial
    /// field values.
    pub secret: Option<crate::model::Secret>,
}

impl CreateSecretRequest {
    /// Sets the value of `parent`.
    pub fn set_parent<T: Into<String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of `secret_id`.
    pub fn set_secret_id<T: Into<String>>(mut self, v: T) -> Self {
        self.secret_id = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<Option<crate::model::Secret>>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AddSecretVersionRequest {
    /// Required. The resource name of the
    /// [Secret][google.cloud.secretmanager.v1.Secret] to associate with the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*` or `projects/*/locations/*/secrets/*`.
    pub parent: String,

    /// Required. The secret payload of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub payload: Option<crate::model::SecretPayload>,
}

impl AddSecretVersionRequest {
    /// Sets the value of `parent`.
    pub fn set_parent<T: Into<String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of `payload`.
    pub fn set_payload<T: Into<Option<crate::model::SecretPayload>>>(mut self, v: T) -> Self {
        self.payload = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.GetSecret][google.cloud.secretmanager.v1.SecretManagerService.GetSecret].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetSecretRequest {
    /// Required. The resource name of the
    /// [Secret][google.cloud.secretmanager.v1.Secret], in the format
    /// `projects/*/secrets/*` or `projects/*/locations/*/secrets/*`.
    pub name: String,
}

impl GetSecretRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretVersionsRequest {
    /// Required. The resource name of the
    /// [Secret][google.cloud.secretmanager.v1.Secret] associated with the
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] to list, in
    /// the format `projects/*/secrets/*` or `projects/*/locations/*/secrets/*`.
    pub parent: String,

    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    pub page_size: i32,

    /// Optional. Pagination token, returned earlier via
    /// ListSecretVersionsResponse.next_page_token][].
    pub page_token: String,

    /// Optional. Filter string, adhering to the rules in
    /// [List-operation
    /// filtering](https://cloud.google.com/secret-manager/docs/filtering). List
    /// only secret versions matching the filter. If filter is empty, all secret
    /// versions are listed.
    pub filter: String,
}

impl ListSecretVersionsRequest {
    /// Sets the value of `parent`.
    pub fn set_parent<T: Into<String>>(mut self, v: T) -> Self {
        self.parent = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<String>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

/// Response message for
/// [SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretVersionsResponse {
    /// The list of [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]
    /// sorted in reverse by create_time (newest first).
    pub versions: Vec<crate::model::SecretVersion>,

    /// A token to retrieve the next page of results. Pass this value in
    /// [ListSecretVersionsRequest.page_token][google.cloud.secretmanager.v1.ListSecretVersionsRequest.page_token]
    /// to retrieve the next page.
    pub next_page_token: String,

    /// The total number of
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] but 0 when
    /// the
    /// [ListSecretsRequest.filter][google.cloud.secretmanager.v1.ListSecretsRequest.filter]
    /// field is set.
    pub total_size: i32,
}

impl ListSecretVersionsResponse {
    /// Sets the value of `versions`.
    pub fn set_versions<T: Into<Vec<crate::model::SecretVersion>>>(mut self, v: T) -> Self {
        self.versions = v.into();
        self
    }

    /// Sets the value of `next_page_token`.
    pub fn set_next_page_token<T: Into<String>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of `total_size`.
    pub fn set_total_size<T: Into<i32>>(mut self, v: T) -> Self {
        self.total_size = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.GetSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.GetSecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetSecretVersionRequest {
    /// Required. The resource name of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*` or
    /// `projects/*/locations/*/secrets/*/versions/*`.
    ///
    /// `projects/*/secrets/*/versions/latest` or
    /// `projects/*/locations/*/secrets/*/versions/latest` is an alias to the most
    /// recently created
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub name: String,
}

impl GetSecretVersionRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.UpdateSecret][google.cloud.secretmanager.v1.SecretManagerService.UpdateSecret].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UpdateSecretRequest {
    /// Required. [Secret][google.cloud.secretmanager.v1.Secret] with updated field
    /// values.
    pub secret: Option<crate::model::Secret>,

    /// Required. Specifies the fields to be updated.
    pub update_mask: Option<wkt::FieldMask>,
}

impl UpdateSecretRequest {
    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<Option<crate::model::Secret>>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `update_mask`.
    pub fn set_update_mask<T: Into<Option<wkt::FieldMask>>>(mut self, v: T) -> Self {
        self.update_mask = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccessSecretVersionRequest {
    /// Required. The resource name of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*` or
    /// `projects/*/locations/*/secrets/*/versions/*`.
    ///
    /// `projects/*/secrets/*/versions/latest` or
    /// `projects/*/locations/*/secrets/*/versions/latest` is an alias to the most
    /// recently created
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    pub name: String,
}

impl AccessSecretVersionRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

/// Response message for
/// [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccessSecretVersionResponse {
    /// The resource name of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*` or
    /// `projects/*/locations/*/secrets/*/versions/*`.
    pub name: String,

    /// Secret payload
    pub payload: Option<crate::model::SecretPayload>,
}

impl AccessSecretVersionResponse {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `payload`.
    pub fn set_payload<T: Into<Option<crate::model::SecretPayload>>>(mut self, v: T) -> Self {
        self.payload = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.DeleteSecret][google.cloud.secretmanager.v1.SecretManagerService.DeleteSecret].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteSecretRequest {
    /// Required. The resource name of the
    /// [Secret][google.cloud.secretmanager.v1.Secret] to delete in the format
    /// `projects/*/secrets/*`.
    pub name: String,

    /// Optional. Etag of the [Secret][google.cloud.secretmanager.v1.Secret]. The
    /// request succeeds if it matches the etag of the currently stored secret
    /// object. If the etag is omitted, the request succeeds.
    pub etag: String,
}

impl DeleteSecretRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.DisableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DisableSecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DisableSecretVersionRequest {
    /// Required. The resource name of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to disable in
    /// the format `projects/*/secrets/*/versions/*` or
    /// `projects/*/locations/*/secrets/*/versions/*`.
    pub name: String,

    /// Optional. Etag of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. The request
    /// succeeds if it matches the etag of the currently stored secret version
    /// object. If the etag is omitted, the request succeeds.
    pub etag: String,
}

impl DisableSecretVersionRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.EnableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.EnableSecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EnableSecretVersionRequest {
    /// Required. The resource name of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to enable in
    /// the format `projects/*/secrets/*/versions/*` or
    /// `projects/*/locations/*/secrets/*/versions/*`.
    pub name: String,

    /// Optional. Etag of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. The request
    /// succeeds if it matches the etag of the currently stored secret version
    /// object. If the etag is omitted, the request succeeds.
    pub etag: String,
}

impl EnableSecretVersionRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }
}

/// Request message for
/// [SecretManagerService.DestroySecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DestroySecretVersion].
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DestroySecretVersionRequest {
    /// Required. The resource name of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to destroy in
    /// the format `projects/*/secrets/*/versions/*` or
    /// `projects/*/locations/*/secrets/*/versions/*`.
    pub name: String,

    /// Optional. Etag of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. The request
    /// succeeds if it matches the etag of the currently stored secret version
    /// object. If the etag is omitted, the request succeeds.
    pub etag: String,
}

impl DestroySecretVersionRequest {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<String>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }
}
