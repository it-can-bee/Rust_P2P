//引入异步流
use libp2p::futures::StreamExt;

//Swarm蜂巢
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::{identity, PeerId};
use std::error::Error;

//引入mDNS解析其他节点ip
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 生成新的密钥对和 PeerId
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());

    println!("New PeerId is {:?}", new_peer_id);

    /*创建mDNS行为，一般改behavior*/
    let behaviour = Mdns::new(MdnsConfig::default()).await?;
    /*创建传输*/
    let transport = libp2p::development_transport(new_key).await?;
    /*创建swarm*/
    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);

    //监听所有端口 包括本地回环地址127.0.0.1和本地网络地址192.168.xx
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            //监听event
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                /*发现peer远程节点*/
                for (peer, addr) in peers {
                    println!("peers is discovered {:?} {:?}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(peers)) => {
                /*发现peer远程节点过期*/
                for (peer, addr) in peers {
                    println!("peers is expired {:?} {:?}", peer, addr);
                }
            }
            _ => {}
        }
    }
}




