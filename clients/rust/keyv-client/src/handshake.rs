use tokio::net::TcpStream;
use tracing::info;
use keyv_core::instructions::init::Init;
use keyv_core::NwInvoke;

pub async fn handshake(s:&mut TcpStream, p:&str) ->eyre::Result<()> {
    let init = Init {
        master_pwd:p.to_string()
    };

    let result = s.invoke_instruction(init).await?;
    
    info!("handshake success: {}",result.success);
    
    Ok(())
}