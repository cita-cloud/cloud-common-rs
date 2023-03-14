#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatusCodeEnum {
    /// / Success: 0
    Success = 0,
    /// / Convert int to status Error
    ConvertIntError = 1,
    /// / status code is none
    NoneStatusCode = 2,
    /// / fate error
    FatalError = 3,
    /// / controller error; start from 100
    /// / node in misbehave list
    MisbehaveNode = 100,
    /// / node in ban list
    BannedNode = 101,
    /// / address not consistent with record origin
    AddressOriginCheckError = 102,
    /// / provide address len is not 20
    ProvideAddressError = 103,
    /// / message not provide address
    NoProvideAddress = 104,
    /// / not get the block
    NoBlock = 105,
    /// / not get the proof
    NoProof = 106,
    /// / not get height of block which wrap tx
    NoTxHeight = 107,
    /// / not get tx index
    NoTxIndex = 108,
    /// / not get transaction
    NoTransaction = 109,
    /// / not get the block height base on hash
    NoBlockHeight = 110,
    /// / not get the block hash base on height
    NoBlockHash = 111,
    /// / proposal is none
    NoneProposal = 112,
    /// / block body is none
    NoneBlockBody = 113,
    /// / block header is none
    NoneBlockHeader = 114,
    /// / chain status is none
    NoneChainStatus = 115,
    /// / transaction's witness is none
    NoneWitness = 116,
    /// / transaction is none
    NoneTransaction = 117,
    /// / utxo is none
    NoneUtxo = 118,
    /// / raw tx is none
    NoneRawTx = 119,
    /// / early status received
    EarlyStatus = 120,
    /// / execute error
    ExecuteError = 121,
    /// / proto struct encode error
    EncodeError = 122,
    /// / proto struct encode error
    DecodeError = 123,
    /// / no candidate block
    NoCandidate = 124,
    /// / not get early status
    NoEarlyStatus = 125,
    /// / fork tree no block
    NoForkTree = 126,
    /// / find dup transaction
    DupTransaction = 127,
    /// / proposal too high
    ProposalTooHigh = 128,
    /// / proposal too low
    ProposalTooLow = 129,
    /// / proposal check error
    ProposalCheckError = 130,
    /// / consensus check proposal error
    ConsensusProposalCheckError = 131,
    /// / block hash check error
    BlockCheckError = 132,
    /// / the sig of chain status init check error
    CsiSigCheckError = 133,
    /// / chain version or chain id check error
    VersionOrIdCheckError = 134,
    /// / hash check error
    HashCheckError = 135,
    /// / hash len is not correct
    HashLenError = 136,
    /// / signature len is not correct
    SigLenError = 137,
    /// / signature check error
    SigCheckError = 138,
    /// / the node in sync mode
    NodeInSyncMode = 139,
    /// / Dup tx in history
    HistoryDupTx = 140,
    /// / emergency brake
    EmergencyBrake = 141,
    /// / auth check tx's version error
    InvalidVersion = 142,
    /// / auth check tx's to error
    InvalidTo = 143,
    /// / auth check tx's nonce error
    InvalidNonce = 144,
    /// / auth check tx's valid until block error
    InvalidValidUntilBlock = 145,
    /// / auth check tx's value error
    InvalidValue = 146,
    /// / auth check tx's chain id error
    InvalidChainId = 147,
    /// / auth limit utxo's witness only one
    InvalidWitness = 148,
    /// / auth check utxo's lock id error
    InvalidLockId = 149,
    /// / auth check utxo's pre tx hash error
    InvalidPreHash = 150,
    /// / auth check send is not admin
    AdminCheckError = 151,
    /// / network msg's module not controller
    ModuleNotController = 152,
    /// / the quota use of tx has exceeded quota-limit
    QuotaUsedExceed = 153,
    /// / not get the state_root
    NoStateRoot = 154,
    /// / block state_root check error
    StateRootCheckError = 155,
    /// / update system-config error; wrong prehash or unallowed lockid
    UpdateSystemConfigError = 156,
    /// / Consensus from 200
    /// / check proposal proof error
    ConsensusServerNotReady = 200,
    /// / proof of proposal error
    ProposalProofError = 201,
    /// / Crypto from 300
    /// / Crypto server not ready
    CryptoServerNotReady = 300,
    /// / hash result is none
    NoneHashResult = 301,
    /// / construct signature error
    ConstructSigError = 302,
    /// / construct key pair error
    ConstructKeyPairError = 303,
    /// / sign error
    SignError = 304,
    /// / Network from 400
    /// / Network server not ready
    NetworkServerNotReady = 400,
    /// / send message error
    SendMsgError = 401,
    /// / broadcast message error
    BroadcastMsgError = 402,
    /// / multi-addr error
    MultiAddrParseError = 403,
    /// / dial node failed
    DialNodeFail = 404,
    /// add an existed peer
    AddExistedPeer = 405,
    /// / executor from 500
    /// / Executor server not ready
    ExecuteServerNotReady = 500,
    /// / internal channel disconnected
    InternalChannelDisconnected = 501,
    /// / early same block reenter
    ReenterBlock = 502,
    /// / invalid block reenter
    ReenterInvalidBlock = 503,
    /// / storage from 600
    /// / storage server not ready
    StorageServerNotReady = 600,
    /// / kv not found
    NotFound = 601,
    /// / invalid region
    InvalidRegion = 602,
    /// / invalid key
    InvalidKey = 603,
    /// / bad region
    BadRegion = 604,
    /// / store data error
    StoreError = 605,
    /// / load data error
    LoadError = 606,
    /// / delete data error
    DeleteError = 607,
}
impl StatusCodeEnum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StatusCodeEnum::Success => "Success",
            StatusCodeEnum::ConvertIntError => "ConvertIntError",
            StatusCodeEnum::NoneStatusCode => "NoneStatusCode",
            StatusCodeEnum::FatalError => "FatalError",
            StatusCodeEnum::MisbehaveNode => "MisbehaveNode",
            StatusCodeEnum::BannedNode => "BannedNode",
            StatusCodeEnum::AddressOriginCheckError => "AddressOriginCheckError",
            StatusCodeEnum::ProvideAddressError => "ProvideAddressError",
            StatusCodeEnum::NoProvideAddress => "NoProvideAddress",
            StatusCodeEnum::NoBlock => "NoBlock",
            StatusCodeEnum::NoProof => "NoProof",
            StatusCodeEnum::NoTxHeight => "NoTxHeight",
            StatusCodeEnum::NoTxIndex => "NoTxIndex",
            StatusCodeEnum::NoTransaction => "NoTransaction",
            StatusCodeEnum::NoBlockHeight => "NoBlockHeight",
            StatusCodeEnum::NoBlockHash => "NoBlockHash",
            StatusCodeEnum::NoneProposal => "NoneProposal",
            StatusCodeEnum::NoneBlockBody => "NoneBlockBody",
            StatusCodeEnum::NoneBlockHeader => "NoneBlockHeader",
            StatusCodeEnum::NoneChainStatus => "NoneChainStatus",
            StatusCodeEnum::NoneWitness => "NoneWitness",
            StatusCodeEnum::NoneTransaction => "NoneTransaction",
            StatusCodeEnum::NoneUtxo => "NoneUtxo",
            StatusCodeEnum::NoneRawTx => "NoneRawTx",
            StatusCodeEnum::EarlyStatus => "EarlyStatus",
            StatusCodeEnum::ExecuteError => "ExecuteError",
            StatusCodeEnum::EncodeError => "EncodeError",
            StatusCodeEnum::DecodeError => "DecodeError",
            StatusCodeEnum::NoCandidate => "NoCandidate",
            StatusCodeEnum::NoEarlyStatus => "NoEarlyStatus",
            StatusCodeEnum::NoForkTree => "NoForkTree",
            StatusCodeEnum::DupTransaction => "DupTransaction",
            StatusCodeEnum::ProposalTooHigh => "ProposalTooHigh",
            StatusCodeEnum::ProposalTooLow => "ProposalTooLow",
            StatusCodeEnum::ProposalCheckError => "ProposalCheckError",
            StatusCodeEnum::ConsensusProposalCheckError => "ConsensusProposalCheckError",
            StatusCodeEnum::BlockCheckError => "BlockCheckError",
            StatusCodeEnum::CsiSigCheckError => "CSISigCheckError",
            StatusCodeEnum::VersionOrIdCheckError => "VersionOrIdCheckError",
            StatusCodeEnum::HashCheckError => "HashCheckError",
            StatusCodeEnum::HashLenError => "HashLenError",
            StatusCodeEnum::SigLenError => "SigLenError",
            StatusCodeEnum::SigCheckError => "SigCheckError",
            StatusCodeEnum::NodeInSyncMode => "NodeInSyncMode",
            StatusCodeEnum::HistoryDupTx => "HistoryDupTx",
            StatusCodeEnum::EmergencyBrake => "EmergencyBrake",
            StatusCodeEnum::InvalidVersion => "InvalidVersion",
            StatusCodeEnum::InvalidTo => "InvalidTo",
            StatusCodeEnum::InvalidNonce => "InvalidNonce",
            StatusCodeEnum::InvalidValidUntilBlock => "InvalidValidUntilBlock",
            StatusCodeEnum::InvalidValue => "InvalidValue",
            StatusCodeEnum::InvalidChainId => "InvalidChainId",
            StatusCodeEnum::InvalidWitness => "InvalidWitness",
            StatusCodeEnum::InvalidLockId => "InvalidLockId",
            StatusCodeEnum::InvalidPreHash => "InvalidPreHash",
            StatusCodeEnum::AdminCheckError => "AdminCheckError",
            StatusCodeEnum::ModuleNotController => "ModuleNotController",
            StatusCodeEnum::QuotaUsedExceed => "QuotaUsedExceed",
            StatusCodeEnum::NoStateRoot => "NoStateRoot",
            StatusCodeEnum::StateRootCheckError => "StateRootCheckError",
            StatusCodeEnum::UpdateSystemConfigError => "UpdateSystemConfigError",
            StatusCodeEnum::ConsensusServerNotReady => "ConsensusServerNotReady",
            StatusCodeEnum::ProposalProofError => "ProposalProofError",
            StatusCodeEnum::CryptoServerNotReady => "CryptoServerNotReady",
            StatusCodeEnum::NoneHashResult => "NoneHashResult",
            StatusCodeEnum::ConstructSigError => "ConstructSigError",
            StatusCodeEnum::ConstructKeyPairError => "ConstructKeyPairError",
            StatusCodeEnum::SignError => "SignError",
            StatusCodeEnum::NetworkServerNotReady => "NetworkServerNotReady",
            StatusCodeEnum::SendMsgError => "SendMsgError",
            StatusCodeEnum::BroadcastMsgError => "BroadcastMsgError",
            StatusCodeEnum::MultiAddrParseError => "MultiAddrParseError",
            StatusCodeEnum::DialNodeFail => "DialNodeFail",
            StatusCodeEnum::AddExistedPeer => "AddExistedPeer",
            StatusCodeEnum::ExecuteServerNotReady => "ExecuteServerNotReady",
            StatusCodeEnum::InternalChannelDisconnected => "InternalChannelDisconnected",
            StatusCodeEnum::ReenterBlock => "ReenterBlock",
            StatusCodeEnum::ReenterInvalidBlock => "ReenterInvalidBlock",
            StatusCodeEnum::StorageServerNotReady => "StorageServerNotReady",
            StatusCodeEnum::NotFound => "NotFound",
            StatusCodeEnum::InvalidRegion => "InvalidRegion",
            StatusCodeEnum::InvalidKey => "InvalidKey",
            StatusCodeEnum::BadRegion => "BadRegion",
            StatusCodeEnum::StoreError => "StoreError",
            StatusCodeEnum::LoadError => "LoadError",
            StatusCodeEnum::DeleteError => "DeleteError",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Success" => Some(Self::Success),
            "ConvertIntError" => Some(Self::ConvertIntError),
            "NoneStatusCode" => Some(Self::NoneStatusCode),
            "FatalError" => Some(Self::FatalError),
            "MisbehaveNode" => Some(Self::MisbehaveNode),
            "BannedNode" => Some(Self::BannedNode),
            "AddressOriginCheckError" => Some(Self::AddressOriginCheckError),
            "ProvideAddressError" => Some(Self::ProvideAddressError),
            "NoProvideAddress" => Some(Self::NoProvideAddress),
            "NoBlock" => Some(Self::NoBlock),
            "NoProof" => Some(Self::NoProof),
            "NoTxHeight" => Some(Self::NoTxHeight),
            "NoTxIndex" => Some(Self::NoTxIndex),
            "NoTransaction" => Some(Self::NoTransaction),
            "NoBlockHeight" => Some(Self::NoBlockHeight),
            "NoBlockHash" => Some(Self::NoBlockHash),
            "NoneProposal" => Some(Self::NoneProposal),
            "NoneBlockBody" => Some(Self::NoneBlockBody),
            "NoneBlockHeader" => Some(Self::NoneBlockHeader),
            "NoneChainStatus" => Some(Self::NoneChainStatus),
            "NoneWitness" => Some(Self::NoneWitness),
            "NoneTransaction" => Some(Self::NoneTransaction),
            "NoneUtxo" => Some(Self::NoneUtxo),
            "NoneRawTx" => Some(Self::NoneRawTx),
            "EarlyStatus" => Some(Self::EarlyStatus),
            "ExecuteError" => Some(Self::ExecuteError),
            "EncodeError" => Some(Self::EncodeError),
            "DecodeError" => Some(Self::DecodeError),
            "NoCandidate" => Some(Self::NoCandidate),
            "NoEarlyStatus" => Some(Self::NoEarlyStatus),
            "NoForkTree" => Some(Self::NoForkTree),
            "DupTransaction" => Some(Self::DupTransaction),
            "ProposalTooHigh" => Some(Self::ProposalTooHigh),
            "ProposalTooLow" => Some(Self::ProposalTooLow),
            "ProposalCheckError" => Some(Self::ProposalCheckError),
            "ConsensusProposalCheckError" => Some(Self::ConsensusProposalCheckError),
            "BlockCheckError" => Some(Self::BlockCheckError),
            "CSISigCheckError" => Some(Self::CsiSigCheckError),
            "VersionOrIdCheckError" => Some(Self::VersionOrIdCheckError),
            "HashCheckError" => Some(Self::HashCheckError),
            "HashLenError" => Some(Self::HashLenError),
            "SigLenError" => Some(Self::SigLenError),
            "SigCheckError" => Some(Self::SigCheckError),
            "NodeInSyncMode" => Some(Self::NodeInSyncMode),
            "HistoryDupTx" => Some(Self::HistoryDupTx),
            "EmergencyBrake" => Some(Self::EmergencyBrake),
            "InvalidVersion" => Some(Self::InvalidVersion),
            "InvalidTo" => Some(Self::InvalidTo),
            "InvalidNonce" => Some(Self::InvalidNonce),
            "InvalidValidUntilBlock" => Some(Self::InvalidValidUntilBlock),
            "InvalidValue" => Some(Self::InvalidValue),
            "InvalidChainId" => Some(Self::InvalidChainId),
            "InvalidWitness" => Some(Self::InvalidWitness),
            "InvalidLockId" => Some(Self::InvalidLockId),
            "InvalidPreHash" => Some(Self::InvalidPreHash),
            "AdminCheckError" => Some(Self::AdminCheckError),
            "ModuleNotController" => Some(Self::ModuleNotController),
            "QuotaUsedExceed" => Some(Self::QuotaUsedExceed),
            "NoStateRoot" => Some(Self::NoStateRoot),
            "StateRootCheckError" => Some(Self::StateRootCheckError),
            "UpdateSystemConfigError" => Some(Self::UpdateSystemConfigError),
            "ConsensusServerNotReady" => Some(Self::ConsensusServerNotReady),
            "ProposalProofError" => Some(Self::ProposalProofError),
            "CryptoServerNotReady" => Some(Self::CryptoServerNotReady),
            "NoneHashResult" => Some(Self::NoneHashResult),
            "ConstructSigError" => Some(Self::ConstructSigError),
            "ConstructKeyPairError" => Some(Self::ConstructKeyPairError),
            "SignError" => Some(Self::SignError),
            "NetworkServerNotReady" => Some(Self::NetworkServerNotReady),
            "SendMsgError" => Some(Self::SendMsgError),
            "BroadcastMsgError" => Some(Self::BroadcastMsgError),
            "MultiAddrParseError" => Some(Self::MultiAddrParseError),
            "DialNodeFail" => Some(Self::DialNodeFail),
            "AddExistedPeer" => Some(Self::AddExistedPeer),
            "ExecuteServerNotReady" => Some(Self::ExecuteServerNotReady),
            "InternalChannelDisconnected" => Some(Self::InternalChannelDisconnected),
            "ReenterBlock" => Some(Self::ReenterBlock),
            "ReenterInvalidBlock" => Some(Self::ReenterInvalidBlock),
            "StorageServerNotReady" => Some(Self::StorageServerNotReady),
            "NotFound" => Some(Self::NotFound),
            "InvalidRegion" => Some(Self::InvalidRegion),
            "InvalidKey" => Some(Self::InvalidKey),
            "BadRegion" => Some(Self::BadRegion),
            "StoreError" => Some(Self::StoreError),
            "LoadError" => Some(Self::LoadError),
            "DeleteError" => Some(Self::DeleteError),
            _ => None,
        }
    }
}
