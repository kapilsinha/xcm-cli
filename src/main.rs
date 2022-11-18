#[subxt::subxt(runtime_metadata_path = "rococo_metadata.scale")]
pub mod relay_chain {}

use relay_chain::runtime_types::xcm::{
    v0::{
        junction::{Junction, NetworkId},
        multi_location::MultiLocation as V0MultiLocation,
    },
    v1::{
        multiasset::{AssetId, Fungibility, MultiAsset, MultiAssets},
        multilocation::{Junctions, MultiLocation as V1MultiLocation},
    },
    VersionedMultiAssets, VersionedMultiLocation,
};

use color_eyre::eyre::{self, WrapErr};
use structopt::StructOpt;
use subxt::{ClientBuilder, Config, DefaultConfig, PairSigner, PolkadotExtrinsicParams};
use subxt::sp_core::{Pair, sr25519};
use core::fmt::Write;

type SignedExtra = PolkadotExtrinsicParams<DefaultConfig>;

/// CLI for submitting XCM messages.
#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Teleport an asset to another parachain.
    #[structopt(name = "teleport")]
    TeleportAsset {
        /// The id of the destination parachain.
        #[structopt(long, short)]
        parachain_id: u32,
        /// The account on the destination parachain to which.
        #[structopt(long, short)]
        dest_account: <DefaultConfig as Config>::AccountId,
        /// The amount of the (fungible) asset to transfer.
        #[structopt(long, short)]
        amount: u128,
        #[structopt(flatten)]
        extrinsic_opts: ExtrinsicOpts,
    },
}

/// Arguments required for creating and sending an extrinsic to a substrate node
#[derive(Clone, Debug, StructOpt)]
pub(crate) struct ExtrinsicOpts {
    /// Websockets url of a substrate node
    #[structopt(name = "url", long, default_value = "ws://localhost:9944")]
    url: String,
    /// Secret key URI for the account deploying the contract.
    #[structopt(name = "suri", long, short)]
    suri: String,
    /// Password for the secret key
    #[structopt(name = "password", long)]
    password: Option<String>,
}

impl ExtrinsicOpts {
    pub fn signer(&self) -> color_eyre::Result<sr25519::Pair> {
        sr25519::Pair::from_string(&self.suri, self.password.as_ref().map(String::as_ref))
            .map_err(|_| eyre::eyre!("Secret string error"))
    }
}

pub fn vec_to_hex_string(v: &Vec<u8>) -> String {
    let mut res = "0x".to_string();
    for a in v.iter() {
        write!(res, "{:02x}", a).expect("should create hex string");
    }
    res
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    env_logger::init();
    color_eyre::install()?;

    let opts = Opts::from_args();

    let Command::TeleportAsset {
        parachain_id,
        dest_account,
        amount,
        extrinsic_opts,
    } = opts.command;
    let signer = PairSigner::new(extrinsic_opts.signer()?);

    let api = ClientBuilder::new()
        .set_url(extrinsic_opts.url)
        .build()
        .await
        .context("Error connecting to substrate node")?
        .to_runtime_api::<relay_chain::RuntimeApi<DefaultConfig, SignedExtra>>();

    let dest = VersionedMultiLocation::V0(V0MultiLocation::X1(Junction::Parachain(parachain_id)));
    let beneficiary = VersionedMultiLocation::V0(V0MultiLocation::X1(Junction::AccountId32 {
        network: NetworkId::Any,
        id: dest_account.into(),
    }));
    let assets = VersionedMultiAssets::V1(MultiAssets(vec![MultiAsset {
        id: AssetId::Concrete(V1MultiLocation {
            parents: 0,
            interior: Junctions::Here,
        }),
        fun: Fungibility::Fungible(amount),
    }]));
    let fee_asset_item = 0;

    let signed_extrinsic = api
        .tx()
        .xcm_pallet()
        .teleport_assets(dest, beneficiary, assets, fee_asset_item)
        .context("Error creating extrinsic")?
        .create_signed(&signer, Default::default())
        .await?;
    println!("{:?}", vec_to_hex_string(&signed_extrinsic.encoded().to_vec()));

    // let events = api
    //     .tx()
    //     .xcm_pallet()
    //     .teleport_assets(dest, beneficiary, assets, fee_asset_item)
    //     .context("Error creating extrinsic")?
    //     .sign_and_submit_then_watch_default(&signer)
    //     .await?
    //     .wait_for_finalized_success()
    //     .await
    //     .context("Error submitting extrinsic")?;

    // for event in events.as_slice() {
    //     println!("{:?}", event)
    // }

    Ok(())
}
