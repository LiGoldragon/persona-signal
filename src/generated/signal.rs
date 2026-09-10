#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct Presence {
    pub expected_component: ExpectedComponent,
    pub expected_kind: ExpectedKind,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
}
#[rustfmt::skip]
pub type ComponentInstanceName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct OtherPersonaEngine {
    pub engine_identifier: EngineIdentifier,
    pub host: Host,
}
#[rustfmt::skip]
pub type ExpectedComponent = ComponentName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum OwnerIdentity {
    System(SystemPrincipal),
    UnixUser(UnixUserIdentifier),
}
#[rustfmt::skip]
pub type EngineManagementProtocolVersion = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ComponentNotReadyReason {
    RecoveringFromCrash,
    NotYetBound,
    AwaitingDependency,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ComponentStatus {
    pub component_name: ComponentName,
    pub component_kind: ComponentKind,
    pub component_desired_state: ComponentDesiredState,
    pub component_health: ComponentHealth,
}
#[rustfmt::skip]
pub type ComponentReady = std::option::Option<TimestampNanoseconds>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SpawnEnvelope {
    pub engine_identifier: EngineIdentifier,
    pub component_kind: ComponentKind,
    pub component_principal: ComponentPrincipal,
    pub owner_identity: OwnerIdentity,
    pub state_directory_path: StateDirectoryPath,
    pub domain_socket_path: DomainSocketPath,
    pub domain_socket_mode: DomainSocketMode,
    pub engine_management_socket_path: EngineManagementSocketPath,
    pub engine_management_socket_mode: EngineManagementSocketMode,
    pub peer_socket_vector: std::vec::Vec<PeerSocket>,
    pub manager_socket_path: ManagerSocketPath,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
}
#[rustfmt::skip]
pub type UnixUserIdentifier = i64;
#[rustfmt::skip]
pub type NetworkPeer = String;
#[rustfmt::skip]
pub type IngressContext = MessageOrigin;
#[rustfmt::skip]
pub type RequestUnimplemented = UnimplementedReason;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum LifecycleReply {
    Ready(ComponentReady),
    Unimplemented(RequestUnimplemented),
    HealthReport(ComponentHealthReport),
    Identified(ComponentIdentity),
    StopAcknowledged(StopAcknowledgement),
    NotReady(ComponentNotReady),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ComponentPrincipal {
    Router,
    System,
    Spirit,
    Orchestrate,
    Mind,
    Introspect,
    Harness,
    Terminal,
    Message,
}
#[rustfmt::skip]
pub type ChannelIdentifier = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ComponentHealth {
    Degraded,
    Stopped,
    Running,
    Starting,
    Failed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct ComponentIdentity {
    pub component_name: ComponentName,
    pub component_kind: ComponentKind,
    pub engine_management_protocol_version: EngineManagementProtocolVersion,
    pub component_startup_error_option: Option<ComponentStartupError>,
}
#[rustfmt::skip]
pub type TimestampNanoseconds = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct PeerSocket {
    pub component_principal: ComponentPrincipal,
    pub domain_socket_path: DomainSocketPath,
}
#[rustfmt::skip]
pub type HostName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum UnimplementedReason {
    NotInPrototypeScope,
    DependencyMissing(DependencyKind),
    ResourceUnavailable(ResourceKind),
}
#[rustfmt::skip]
pub type ComponentHealthReport = ComponentHealth;
#[rustfmt::skip]
pub type DomainSocketMode = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ComponentKind {
    Mind,
    Introspect,
    Harness,
    Spirit,
    Terminal,
    System,
    Message,
    Router,
    Orchestrate,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum LifecycleQuery {
    HealthStatus(ComponentName),
    ReadinessStatus(ComponentName),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum MessageOrigin {
    InternalComponentInstance(InternalComponentInstanceOrigin),
    LocalOwner(OwnerIdentity),
    Internal(ComponentPrincipal),
    Channel(ChannelIdentifier),
    LocalConnection(ConnectionClass),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ResourceKind {
    ManagerSocket,
    SocketPath,
    StateDirectory,
}
#[rustfmt::skip]
pub type EngineManagementSocketPath = String;
#[rustfmt::skip]
pub type EngineManagementSocketMode = i64;
#[rustfmt::skip]
pub type ManagerSocketPath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ComponentStartupError {
    EnvelopeIncomplete,
    SocketBindFailed,
    StoreOpenFailed,
}
#[rustfmt::skip]
pub type ComponentName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum LifecycleRequest {
    Query(LifecycleQuery),
    Announce(Presence),
    Stop(ComponentName),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ConnectionClass {
    Network(NetworkPeer),
    OtherPersona(OtherPersonaEngine),
    Owner,
    NonOwnerUser(UnixUserIdentifier),
    System(SystemPrincipal),
}
#[rustfmt::skip]
pub type Host = HostName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum ComponentDesiredState {
    Stopped,
    Running,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum DependencyKind {
    PeerComponent,
}
#[rustfmt::skip]
pub type ExpectedKind = ComponentKind;
#[rustfmt::skip]
pub type ComponentNotReady = ComponentNotReadyReason;
#[rustfmt::skip]
pub type DomainSocketPath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct InternalComponentInstanceOrigin {
    pub component_principal: ComponentPrincipal,
    pub component_instance_name: ComponentInstanceName,
}
#[rustfmt::skip]
pub type StopAcknowledgement = std::option::Option<TimestampNanoseconds>;
#[rustfmt::skip]
pub type SystemPrincipal = String;
#[rustfmt::skip]
pub type EngineIdentifier = String;
#[rustfmt::skip]
pub type StateDirectoryPath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    Query(LifecycleQuery),
    Announce(Presence),
    Stop(ComponentName),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    Ready(ComponentReady),
    Unimplemented(RequestUnimplemented),
    HealthReport(ComponentHealthReport),
    Identified(ComponentIdentity),
    StopAcknowledged(StopAcknowledgement),
    NotReady(ComponentNotReady),
}
