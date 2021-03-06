use ncube_actors_common::Registry;
use ncube_actors_host::{
    AllSettings, EndpointSetting, HostActor, InsertSetting, IsBootstrapped, SecretKeySetting,
    Settings,
};
use ncube_crypto::gen_secret_key;
use ncube_data::ConfigSetting;
use rand::{self, rngs::StdRng, SeedableRng};
use std::time::SystemTime;

use crate::HandlerError;

pub async fn is_bootstrapped() -> Result<bool, HandlerError> {
    let actor = HostActor::from_registry().await.unwrap();

    let is_bootstrapped = actor.call(IsBootstrapped).await?;

    Ok(is_bootstrapped?)
}

pub async fn show_config() -> Result<Vec<ConfigSetting>, HandlerError> {
    let actor = HostActor::from_registry().await.unwrap();

    if !is_bootstrapped().await? {
        return Err(HandlerError::Invalid(
            "Ncube requires initial bootstrapping.".into(),
        ));
    }

    let result = actor.call(Settings).await?;
    let config = result?;

    Ok(config)
}

pub async fn show_config_all() -> Result<Vec<ConfigSetting>, HandlerError> {
    let actor = HostActor::from_registry().await.unwrap();

    if !is_bootstrapped().await? {
        return Err(HandlerError::Invalid(
            "Ncube requires initial bootstrapping.".into(),
        ));
    }

    let result = actor.call(AllSettings).await?;
    let config = result?;

    Ok(config)
}

pub async fn bootstrap(settings: Vec<(String, String)>) -> Result<(), HandlerError> {
    let actor = HostActor::from_registry().await.unwrap();

    if is_bootstrapped().await? {
        return Err(HandlerError::NotAllowed(
            "Ncube already bootstrapped".into(),
        ));
    }

    // The documentation has a warning about using StdRng::seed_from_u64, that
    // it should not be used for cryptographic applications. I think it's okay
    // here though, since bootstrapping happens only once, and the secret key is
    // only relevant for remote Ncube installations. This code path almost
    // certainly is only triggered for local installations where the actual
    // secret key doesn't matter.
    let d = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Duration since UNIX_EPOCH failed");
    let rng = StdRng::seed_from_u64(d.as_secs());
    let secret_key_setting = ("secret_key".to_string(), gen_secret_key(rng));

    let restricted_settings = vec![secret_key_setting];

    for (name, value) in settings {
        let _ = actor
            .call(InsertSetting::new(name.to_string(), value.to_string()))
            .await?;
    }

    for (name, value) in restricted_settings {
        let _ = actor
            .call(InsertSetting::new(name.to_string(), value.to_string()))
            .await?;
    }

    Ok(())
}

pub async fn insert_config_setting(name: &str, value: &str) -> Result<(), HandlerError> {
    let actor = HostActor::from_registry().await.unwrap();

    if !is_bootstrapped().await? {
        return Err(HandlerError::Invalid(
            "Ncube requires initial bootstrapping.".into(),
        ));
    }

    let _ = actor
        .call(InsertSetting::new(name.to_string(), value.to_string()))
        .await?;

    Ok(())
}

pub async fn show_secret_key() -> Result<String, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();
    let key = host_actor.call(SecretKeySetting).await??;
    Ok(key
        .value
        .ok_or_else(|| HandlerError::NotFound("no secret key".into()))?)
}

pub async fn endpoint() -> Result<String, HandlerError> {
    let host_actor = HostActor::from_registry().await.unwrap();
    let endpoint = host_actor.call(EndpointSetting).await??;
    Ok(endpoint
        .value
        .ok_or_else(|| HandlerError::NotFound("no endpoint".into()))?)
}
