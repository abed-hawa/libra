//! `bal` subcommand

use crate::{config::AppCfg, entrypoint, node::node::Node, prelude::app_config};
use anyhow::Error;
use anyhow::Result;
use cli::diem_client::DiemClient;
use diem_types::waypoint::Waypoint;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use reqwest::Url;
use std::path::PathBuf;

/// returns a DiemClient instance.
// TODO: Use app config file for params
pub fn make_client(url: Option<Url>, waypoint: Waypoint) -> Result<DiemClient, Error> {
    match url {
        Some(u) => DiemClient::new(u, waypoint),
        None => DiemClient::new(
            Url::parse("http://localhost:8080").expect("Couldn't create diem client"),
            waypoint,
        ),
    }
}

/// Experimental
pub fn get_client() -> Option<DiemClient> {
    let entry_args = entrypoint::get_args();
    // if url and waypoint provided as fn params
    //     return DiemClient::new(url, waypoint);

    // if local node in-sync
    //    return default_local_client()
    // else find and return connect-able upstream node
    let config = app_config();
    let waypoint = config
        .get_waypoint(entry_args.swarm_path)
        .expect("could not get waypoint");
    if let Some(vec_urs) = config.profile.upstream_nodes.as_ref() {
      for url in vec_urs {
          match DiemClient::new(url.clone(), waypoint){
            Ok(client) => { 
              // TODO: What's the better way to check we can connect to client?
                let metadata = client.get_metadata();
                if metadata.is_ok() {
                    // found a connect-able upstream node
                    return Some(client);
                }
            },
            Err(_) => {},
        };
      }
    }


    None
}

/// get client type with defaults from toml for remote node
pub fn find_a_remote_jsonrpc(config: &AppCfg, waypoint: Waypoint) -> Result<DiemClient, Error> {
    let mut rng = thread_rng();
    if let Some(list) = &config.profile.upstream_nodes {
        let len = list.len();
        let url = list.choose_multiple(&mut rng, len)
            .into_iter()
            .find(|&remote_url| {
                println!("trying upstream url: {}", &remote_url);
                match make_client(Some(remote_url.to_owned()), waypoint) {
                    Ok(c) => {
                      match c.get_metadata(){
                        Ok(m) => {
                          if m.version > 0 { true }
                          else { 
                            println!("can make client but could not get blockchain height > 0");
                            false
                          }
                        },
                        Err(e) => {
                          println!("can make client but could not get metadata {:?}", e);
                          false
                        },
                    }
                    },
                    Err(e) => {
                      println!("could not make client {:?}", e);
                      false
                    },
                }
            });
            
            if let Some(url_clean) = url {
              return make_client(Some(url_clean.to_owned()), waypoint);
            }; 
            
    }
    Err(Error::msg(
        "Cannot connect to any JSON RPC peers in the list of upstream_nodes in 0L.toml",
    ))
}

/// get client type with defaults from toml for local node
pub fn default_local_client(config: &AppCfg, waypoint: Waypoint) -> Result<DiemClient, Error> {
    let local_url = config
        .profile
        .default_node
        .clone()
        .expect("could not get url from configs");

    make_client(Some(local_url.clone()), waypoint)
}

/// connect a swarm client
pub fn swarm_test_client(config: &mut AppCfg, swarm_path: PathBuf) -> Result<DiemClient, Error> {
    let (url, waypoint) = ol_types::config::get_swarm_rpc_url(swarm_path.clone());
    config.profile.default_node = Some(url.clone());
    config.profile.upstream_nodes = Some(vec![url.clone()]);

    make_client(Some(url.clone()), waypoint)
}

/// picks what URL to connect to based on sync state. Or returns the client for swarm.
pub fn pick_client(swarm_path: Option<PathBuf>, config: &mut AppCfg) -> Result<DiemClient, Error> {
    let is_swarm = *&swarm_path.is_some();
    if let Some(path) = swarm_path {
        return swarm_test_client(config, path);
    };
    let waypoint = config.get_waypoint(swarm_path)?;

    // check if is in sync
    let local_client = default_local_client(config, waypoint.clone())?;

    let remote_client = find_a_remote_jsonrpc(config, waypoint.clone())?;
    // compares to an upstream random remote client. If it is synced, use the local client as the default
    let mut node = Node::new(local_client, config, is_swarm);
    match node.check_sync()?
    .is_synced {
      true => Ok(node.client),
      false => Ok(remote_client),

    }
}
