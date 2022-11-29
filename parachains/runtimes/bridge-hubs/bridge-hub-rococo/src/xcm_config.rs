// Copyright 2022 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use super::{
	AccountId, Balance, Balances, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime,
	RuntimeCall, RuntimeEvent, RuntimeOrigin, XcmpQueue,
};
use crate::{
	bridge_hub_rococo_config::ToBridgeHubWococoHaulBlobExporter,
	bridge_hub_wococo_config::ToBridgeHubRococoHaulBlobExporter,
};
use frame_support::{
	match_types, parameter_types,
	traits::{Everything, Nothing},
	weights::IdentityFee,
};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use sp_core::Get;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, CurrencyAdapter,
	EnsureXcmOrigin, IsConcrete, NativeAsset, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
	WeightInfoBounds,
};
use xcm_executor::{traits::ExportXcm, XcmExecutor};

parameter_types! {
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub UniversalLocation: InteriorMultiLocation = X2(GlobalConsensus(RelayNetwork::get()), Parachain(ParachainInfo::parachain_id().into()));
}

pub struct RelayNetwork;
impl Get<Option<NetworkId>> for RelayNetwork {
	fn get() -> Option<NetworkId> {
		Some(Self::get())
	}
}
impl Get<NetworkId> for RelayNetwork {
	fn get() -> NetworkId {
		match u32::from(ParachainInfo::parachain_id()) {
			bp_bridge_hub_rococo::BRIDGE_HUB_ROCOCO_PARACHAIN_ID => NetworkId::Rococo,
			bp_bridge_hub_wococo::BRIDGE_HUB_WOCOCO_PARACHAIN_ID => NetworkId::Wococo,
			para_id => unreachable!("Not supported for para_id: {}", para_id),
		}
	}
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// Means for transacting assets on this chain.
pub type LocalAssetTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<RelayLocation>,
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
}

match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
}

match_types! {
	pub type ParentOrParentsUnitPlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Unit, .. }) }
	};
}

// TOOD: hacked
// pub type Barrier = DenyThenTry<
// 	DenyReserveTransferToRelayChain,
// 	(
// 		TakeWeightCredit,
// 		AllowTopLevelPaidExecutionFrom<Everything>,
// 		AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
// 		// ^^^ Parent and its exec plurality get free execution
// 	),
// >;
pub type Barrier = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	AllowUnpaidExecutionFrom<ParentOrParentsUnitPlurality>,
	// TODO:check-parameter - supporting unpaid execution at first then SovereignPaid
	AllowUnpaidExecutionFrom<Everything>,
	// ^^^ Parent & its unit plurality gets free execution
);

type XcmWeigher = WeightInfoBounds<
	crate::weights::xcm::BridgeHubXcmWeight<RuntimeCall>,
	RuntimeCall,
	MaxInstructions,
>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = NativeAsset; // <- should be enough to allow teleportation of UNIT
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = XcmWeigher;
	type Trader = UsingComponents<IdentityFee<Balance>, RelayLocation, AccountId, Balances, ()>;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = ();
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = BridgeHubRococoOrBridgeHubWococoSwitchExporter;
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm, ()>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type Weigher = XcmWeigher;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = ();
	type MaxLockers = frame_support::traits::ConstU32<8>;
	type UniversalLocation = UniversalLocation;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

/// Hacky switch implementation, because we have just one runtime for Rococo and Wococo BridgeHub, so it means we have just one XcmConfig
pub struct BridgeHubRococoOrBridgeHubWococoSwitchExporter;
impl ExportXcm for BridgeHubRococoOrBridgeHubWococoSwitchExporter {
	type Ticket = (NetworkId, (sp_std::prelude::Vec<u8>, XcmHash));

	fn validate(
		network: NetworkId,
		channel: u32,
		universal_source: &mut Option<InteriorMultiLocation>,
		destination: &mut Option<InteriorMultiLocation>,
		message: &mut Option<Xcm<()>>,
	) -> SendResult<Self::Ticket> {
		match network {
			Rococo => ToBridgeHubRococoHaulBlobExporter::validate(
				network,
				channel,
				universal_source,
				destination,
				message,
			)
			.map(|result| ((Rococo, result.0), result.1)),
			Wococo => ToBridgeHubWococoHaulBlobExporter::validate(
				network,
				channel,
				universal_source,
				destination,
				message,
			)
			.map(|result| ((Wococo, result.0), result.1)),
			_ => unimplemented!("Unsupported network: {:?}", network),
		}
	}

	fn deliver(ticket: Self::Ticket) -> Result<XcmHash, SendError> {
		let (network, ticket) = ticket;
		match network {
			Rococo => ToBridgeHubRococoHaulBlobExporter::deliver(ticket),
			Wococo => ToBridgeHubWococoHaulBlobExporter::deliver(ticket),
			_ => unimplemented!("Unsupported network: {:?}", network),
		}
	}
}