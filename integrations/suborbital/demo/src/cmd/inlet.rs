use crate::storage::*;
use ockam::{route, AsyncTryClone, Context, Identity, TcpTransport, TrustMultiIdentifiersPolicy, TCP};

pub async fn run(args: crate::args::InletOpts, ctx: Context) -> anyhow::Result<()> {
    crate::storage::ensure_identity_exists()?;
    let ockam_dir = crate::storage::get_ockam_dir()?;

    let (exported_id, vault) = crate::identity::load_identity(&ockam_dir)?;
    let idents = crate::storage::read_trusted_idents(&ockam_dir.join("trusted"))?;

    let tcp = TcpTransport::create(&ctx).await?;
    let mut identity = Identity::import(&ctx, &vault, exported_id).await?;

    let channel = identity
        .create_secure_channel(
            route![
                (TCP, &args.cloud_addr),
                format!("forward_to_{}", args.alias),
                "secure_channel_listener"
            ],
            TrustMultiIdentifiersPolicy::new(idents),
        )
        .await?;

    tcp.create_inlet(&args.inlet_address, route![channel, "outlet"]).await?;
    Ok(())
}