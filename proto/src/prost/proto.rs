#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resources {
    #[prost(double, tag="1")]
    pub cpu: f64,
    #[prost(int64, tag="2")]
    pub ram: i64,
    #[prost(int64, tag="3")]
    pub disk: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryDetails {
    #[prost(int32, tag="1")]
    pub max_message_retries: i32,
    #[prost(string, tag="2")]
    pub dead_letter_topic: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionDetails {
    #[prost(string, tag="1")]
    pub tenant: std::string::String,
    #[prost(string, tag="2")]
    pub namespace: std::string::String,
    #[prost(string, tag="3")]
    pub name: std::string::String,
    #[prost(string, tag="4")]
    pub class_name: std::string::String,
    #[prost(string, tag="5")]
    pub log_topic: std::string::String,
    #[prost(enumeration="ProcessingGuarantees", tag="6")]
    pub processing_guarantees: i32,
    #[prost(string, tag="7")]
    pub user_config: std::string::String,
    #[prost(string, tag="16")]
    pub secrets_map: std::string::String,
    #[prost(enumeration="function_details::Runtime", tag="8")]
    pub runtime: i32,
    #[prost(bool, tag="9")]
    pub auto_ack: bool,
    #[prost(int32, tag="10")]
    pub parallelism: i32,
    #[prost(message, optional, tag="11")]
    pub source: ::std::option::Option<SourceSpec>,
    #[prost(message, optional, tag="12")]
    pub sink: ::std::option::Option<SinkSpec>,
    #[prost(message, optional, tag="13")]
    pub resources: ::std::option::Option<Resources>,
    ///present only if function submitted with package-url
    #[prost(string, tag="14")]
    pub package_url: std::string::String,
    #[prost(message, optional, tag="15")]
    pub retry_details: ::std::option::Option<RetryDetails>,
    #[prost(string, tag="17")]
    pub runtime_flags: std::string::String,
    #[prost(enumeration="function_details::ComponentType", tag="18")]
    pub component_type: i32,
    #[prost(string, tag="19")]
    pub custom_runtime_options: std::string::String,
    /// If specified, this will refer to an archive that is
    /// already present in the server 
    #[prost(string, tag="20")]
    pub builtin: std::string::String,
    #[prost(bool, tag="21")]
    pub retain_ordering: bool,
    #[prost(bool, tag="22")]
    pub retain_key_ordering: bool,
    #[prost(enumeration="SubscriptionPosition", tag="23")]
    pub subscription_position: i32,
}
pub mod function_details {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Runtime {
        Java = 0,
        Python = 1,
        Go = 3,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ComponentType {
        Unknown = 0,
        Function = 1,
        Source = 2,
        Sink = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsumerSpec {
    #[prost(string, tag="1")]
    pub schema_type: std::string::String,
    #[prost(string, tag="2")]
    pub serde_class_name: std::string::String,
    #[prost(bool, tag="3")]
    pub is_regex_pattern: bool,
    #[prost(message, optional, tag="4")]
    pub receiver_queue_size: ::std::option::Option<consumer_spec::ReceiverQueueSize>,
    #[prost(map="string, string", tag="5")]
    pub schema_properties: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(map="string, string", tag="6")]
    pub consumer_properties: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(message, optional, tag="7")]
    pub crypto_spec: ::std::option::Option<CryptoSpec>,
}
pub mod consumer_spec {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReceiverQueueSize {
        #[prost(int32, tag="1")]
        pub value: i32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProducerSpec {
    #[prost(int32, tag="1")]
    pub max_pending_messages: i32,
    #[prost(int32, tag="2")]
    pub max_pending_messages_across_partitions: i32,
    #[prost(bool, tag="3")]
    pub use_thread_local_producers: bool,
    #[prost(message, optional, tag="4")]
    pub crypto_spec: ::std::option::Option<CryptoSpec>,
    #[prost(string, tag="5")]
    pub batch_builder: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoSpec {
    #[prost(string, tag="1")]
    pub crypto_key_reader_class_name: std::string::String,
    #[prost(string, tag="2")]
    pub crypto_key_reader_config: std::string::String,
    /// key names used by producer to encrypt data
    #[prost(string, repeated, tag="3")]
    pub producer_encryption_key_name: ::std::vec::Vec<std::string::String>,
    /// define the action if producer fail to encrypt data
    /// one of FAIL, SEND
    #[prost(enumeration="crypto_spec::FailureAction", tag="4")]
    pub producer_crypto_failure_action: i32,
    /// define the action if consumer fail to decrypt data
    /// one of FAIL, DISCARD, CONSUME
    #[prost(enumeration="crypto_spec::FailureAction", tag="5")]
    pub consumer_crypto_failure_action: i32,
}
pub mod crypto_spec {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FailureAction {
        Fail = 0,
        Discard = 1,
        Consume = 2,
        Send = 10,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceSpec {
    #[prost(string, tag="1")]
    pub class_name: std::string::String,
    /// map in json format
    #[prost(string, tag="2")]
    pub configs: std::string::String,
    #[prost(string, tag="5")]
    pub type_class_name: std::string::String,
    /// configs used only when source feeds into functions
    #[prost(enumeration="SubscriptionType", tag="3")]
    pub subscription_type: i32,
    /// @deprecated -- use topicsToSchema
    #[prost(map="string, string", tag="4")]
    pub topics_to_ser_de_class_name: ::std::collections::HashMap<std::string::String, std::string::String>,
    ///*
    ///
    #[prost(map="string, message", tag="10")]
    pub input_specs: ::std::collections::HashMap<std::string::String, ConsumerSpec>,
    #[prost(uint64, tag="6")]
    pub timeout_ms: u64,
    #[prost(string, tag="7")]
    pub topics_pattern: std::string::String,
    /// If specified, this will refer to an archive that is
    /// already present in the server 
    #[prost(string, tag="8")]
    pub builtin: std::string::String,
    #[prost(string, tag="9")]
    pub subscription_name: std::string::String,
    #[prost(bool, tag="11")]
    pub cleanup_subscription: bool,
    #[prost(enumeration="SubscriptionPosition", tag="12")]
    pub subscription_position: i32,
    #[prost(uint64, tag="13")]
    pub negative_ack_redelivery_delay_ms: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SinkSpec {
    #[prost(string, tag="1")]
    pub class_name: std::string::String,
    /// map in json format
    #[prost(string, tag="2")]
    pub configs: std::string::String,
    #[prost(string, tag="5")]
    pub type_class_name: std::string::String,
    /// configs used only when functions output to sink
    #[prost(string, tag="3")]
    pub topic: std::string::String,
    #[prost(message, optional, tag="11")]
    pub producer_spec: ::std::option::Option<ProducerSpec>,
    #[prost(string, tag="4")]
    pub ser_de_class_name: std::string::String,
    /// If specified, this will refer to an archive that is
    /// already present in the server 
    #[prost(string, tag="6")]
    pub builtin: std::string::String,
    ///*
    /// Builtin schema type or custom schema class name
    #[prost(string, tag="7")]
    pub schema_type: std::string::String,
    #[prost(bool, tag="8")]
    pub forward_source_message_property: bool,
    #[prost(map="string, string", tag="9")]
    pub schema_properties: ::std::collections::HashMap<std::string::String, std::string::String>,
    #[prost(map="string, string", tag="10")]
    pub consumer_properties: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageLocationMetaData {
    #[prost(string, tag="1")]
    pub package_path: std::string::String,
    #[prost(string, tag="2")]
    pub original_file_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionMetaData {
    #[prost(message, optional, tag="1")]
    pub function_details: ::std::option::Option<FunctionDetails>,
    #[prost(message, optional, tag="2")]
    pub package_location: ::std::option::Option<PackageLocationMetaData>,
    #[prost(uint64, tag="3")]
    pub version: u64,
    #[prost(uint64, tag="4")]
    pub create_time: u64,
    #[prost(map="int32, enumeration(FunctionState)", tag="5")]
    pub instance_states: ::std::collections::HashMap<i32, i32>,
    #[prost(message, optional, tag="6")]
    pub function_auth_spec: ::std::option::Option<FunctionAuthenticationSpec>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionAuthenticationSpec {
    ///*
    /// function authentication related data that the function authentication provider
    /// needs to cache/distribute to all workers support function authentication.
    /// Depending on the function authentication provider implementation, this can be the actual auth credentials
    /// or a pointer to the auth credentials that this function should use
    #[prost(bytes, tag="1")]
    pub data: std::vec::Vec<u8>,
    ///*
    /// classname of the function auth provicer this data is relevant to
    #[prost(string, tag="2")]
    pub provider: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    #[prost(message, optional, tag="1")]
    pub function_meta_data: ::std::option::Option<FunctionMetaData>,
    #[prost(int32, tag="2")]
    pub instance_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assignment {
    #[prost(message, optional, tag="1")]
    pub instance: ::std::option::Option<Instance>,
    #[prost(string, tag="2")]
    pub worker_id: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProcessingGuarantees {
    /// [default value]
    AtleastOnce = 0,
    AtmostOnce = 1,
    EffectivelyOnce = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionType {
    Shared = 0,
    Failover = 1,
    KeyShared = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionPosition {
    Latest = 0,
    Earliest = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FunctionState {
    Running = 0,
    Stopped = 1,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceRequest {
    #[prost(enumeration="service_request::ServiceRequestType", tag="1")]
    pub service_request_type: i32,
    #[prost(string, tag="2")]
    pub request_id: std::string::String,
    #[prost(message, optional, tag="3")]
    pub function_meta_data: ::std::option::Option<FunctionMetaData>,
    #[prost(string, tag="4")]
    pub worker_id: std::string::String,
}
pub mod service_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ServiceRequestType {
        Update = 0,
        Delete = 1,
        Initialize = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionStatus {
    #[prost(bool, tag="1")]
    pub running: bool,
    #[prost(string, tag="2")]
    pub failure_exception: std::string::String,
    #[prost(int64, tag="3")]
    pub num_restarts: i64,
    /// int64 numProcessed = 4;
    #[prost(int64, tag="17")]
    pub num_received: i64,
    #[prost(int64, tag="5")]
    pub num_successfully_processed: i64,
    #[prost(int64, tag="6")]
    pub num_user_exceptions: i64,
    #[prost(message, repeated, tag="7")]
    pub latest_user_exceptions: ::std::vec::Vec<function_status::ExceptionInformation>,
    #[prost(int64, tag="8")]
    pub num_system_exceptions: i64,
    #[prost(message, repeated, tag="9")]
    pub latest_system_exceptions: ::std::vec::Vec<function_status::ExceptionInformation>,
    #[prost(int64, tag="18")]
    pub num_source_exceptions: i64,
    #[prost(message, repeated, tag="19")]
    pub latest_source_exceptions: ::std::vec::Vec<function_status::ExceptionInformation>,
    #[prost(int64, tag="20")]
    pub num_sink_exceptions: i64,
    #[prost(message, repeated, tag="21")]
    pub latest_sink_exceptions: ::std::vec::Vec<function_status::ExceptionInformation>,
    /// map from topic name to number of deserialization exceptions
    ///    map<string, int64> deserializationExceptions = 10;
    /// number of serialization exceptions on the output
    ///    int64 serializationExceptions = 11;
    /// average latency
    #[prost(double, tag="12")]
    pub average_latency: f64,
    /// When was the last time the function was invoked.
    /// expressed in ms since epoch
    #[prost(int64, tag="13")]
    pub last_invocation_time: i64,
    #[prost(string, tag="14")]
    pub instance_id: std::string::String,
    ///    MetricsData metrics = 15 [deprecated=true];
    /// owner of function-instance
    #[prost(string, tag="16")]
    pub worker_id: std::string::String,
}
pub mod function_status {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExceptionInformation {
        #[prost(string, tag="1")]
        pub exception_string: std::string::String,
        #[prost(int64, tag="2")]
        pub ms_since_epoch: i64,
    }
}
/// Deprecated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionStatusList {
    #[prost(string, tag="2")]
    pub error: std::string::String,
    #[prost(message, repeated, tag="1")]
    pub function_status_list: ::std::vec::Vec<FunctionStatus>,
}
///    message DataDigest {
///        double count = 1;
///        double sum = 2;
///        double max = 3;
///        double min = 4;
///    }
///    map<string, DataDigest> metrics = 1 [deprecated=true];
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricsData {
    /// Total number of records function received from source
    #[prost(int64, tag="2")]
    pub received_total: i64,
    #[prost(int64, tag="10")]
    pub received_total_1min: i64,
    // Total number of records processed
    // No longer used because processedSuccessfullyTotal and userExceptionsTotal add to it
    // int64 processedTotal = 3;

    // int64 processedTotal_1min = 11;

    /// Total number of records successfully processed by user function
    #[prost(int64, tag="4")]
    pub processed_successfully_total: i64,
    #[prost(int64, tag="12")]
    pub processed_successfully_total_1min: i64,
    /// Total number of system exceptions thrown
    #[prost(int64, tag="5")]
    pub system_exceptions_total: i64,
    #[prost(int64, tag="13")]
    pub system_exceptions_total_1min: i64,
    /// Total number of user exceptions thrown
    #[prost(int64, tag="6")]
    pub user_exceptions_total: i64,
    #[prost(int64, tag="14")]
    pub user_exceptions_total_1min: i64,
    /// Average process latency for function
    #[prost(double, tag="7")]
    pub avg_process_latency: f64,
    #[prost(double, tag="15")]
    pub avg_process_latency_1min: f64,
    /// Timestamp of when the function was last invoked
    #[prost(int64, tag="8")]
    pub last_invocation: i64,
    /// User defined metrics
    #[prost(map="string, double", tag="9")]
    pub user_metrics: ::std::collections::HashMap<std::string::String, f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckResult {
    #[prost(bool, tag="1")]
    pub success: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metrics {
    #[prost(message, repeated, tag="1")]
    pub metrics: ::std::vec::Vec<metrics::InstanceMetrics>,
}
pub mod metrics {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceMetrics {
        #[prost(string, tag="1")]
        pub name: std::string::String,
        #[prost(int32, tag="2")]
        pub instance_id: i32,
        #[prost(message, optional, tag="3")]
        pub metrics_data: ::std::option::Option<super::MetricsData>,
    }
}
