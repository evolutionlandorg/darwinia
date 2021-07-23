// --- substrate ---
use frame_support::{traits::Filter, weights::constants::RocksDbWeight};
use frame_system::Config;
use sp_runtime::traits::BlakeTwo256;
use sp_version::RuntimeVersion;
// --- darwinia ---
use crate::{weights::frame_system::WeightInfo, *};

pub struct BaseFilter;
impl Filter<Call> for BaseFilter {
	fn filter(call: &Call) -> bool {
		match call {
			// Pause Ethereum -> Darwinia bridge
			// Around UTC 07-30-2021, 05:00 AM
			//
			// Due to EIP-1559, which introduces a new field in header
			// And this break the header's scale-codec data
			// The on-chain storage need to be migrated
			//
			// Ensure there is no new affirmation
			// Then apply the migration (runtime upgrade) at UTC 07-30-2021, 10:00 AM
			// For the started games, we expected the rounds could be finished during 05:00 AM ~ 10:00 AM
			// TODO: remove this filter in the next runtime upgrade to restart the bridge
			Call::EthereumRelay(darwinia_ethereum_relay::Call::affirm(..))
				if System::block_number() > 4367266 =>
			{
				false
			}
			_ => true,
		}
	}
}

frame_support::parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const SS58Prefix: u8 = 18;
}

impl Config for Runtime {
	type BaseCallFilter = BaseFilter;
	type BlockWeights = RuntimeBlockWeights;
	type BlockLength = RuntimeBlockLength;
	type Origin = Origin;
	type Call = Call;
	type Index = Nonce;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = RocksDbWeight;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = WeightInfo<Runtime>;
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}
