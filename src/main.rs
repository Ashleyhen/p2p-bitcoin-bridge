use libp2p::{
    core::upgrade,
    identity::{self},
    mplex::{self},
    noise::{NoiseConfig, X25519Spec},
    tcp::Config,
    PeerId, Transport,
};
use once_cell::sync::Lazy;

static KEYS: Lazy<identity::Keypair> = Lazy::new(|| identity::Keypair::generate_ed25519());
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    pretty_env_logger::init();
    log::info!("Peer Id: {}", PEER_ID.clone());

    let auth_keys = libp2p::noise::Keypair::<X25519Spec>::new()
        .into_authentic(&KEYS)
        .expect("can create auth keys");


    let transp = libp2p::tcp::tokio::Transport::new(Config::new())
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();
}
