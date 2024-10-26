#![allow(dead_code)]

use anyhow::{anyhow, bail, Result};
use zbus::proxy::ProxyImpl;
use zbus::{Connection, Proxy};
use futures::stream::StreamExt;
use crate::shared::{PlaybackStatus, BASE_PATH};

pub async fn all<
    'a,
    T: ProxyImpl<'a> + From<Proxy<'a>>
>(conn: &Connection) -> Result<Vec<T>> {
    let dbus = zbus::fdo::DBusProxy::new(conn).await?;
    let names = dbus.list_names().await?
        .iter()
        .filter(|it| it.starts_with(BASE_PATH))
        .cloned()
        .collect::<Vec<_>>();
    let mut proxies: Vec<T> = Vec::new();
    for name in names {
        let proxy = T::builder(conn)
            .destination(name)?
            .build();
        proxies.push(proxy.await?);
    }
    Ok(proxies)
}

pub async fn by_name<
    'a,
    T: ProxyImpl<'a> + From<Proxy<'a>>
>(conn: &'a Connection, name: &'a str) -> zbus::Result<T> {
    let proxy: T = T::builder(conn)
        .destination(name)?
        .build().await?;

    Ok(proxy)
}

pub async fn first<
    'a,
    T: ProxyImpl<'a> + From<Proxy<'a>>
>(conn: &Connection) -> Result<T> {
    let dbus = zbus::fdo::DBusProxy::new(conn).await?;
    let Some(name) = dbus.list_names().await?
        .iter().find(|it| it.starts_with(BASE_PATH))
        .cloned() else {
        bail!("No MPRIS2 instances found.");
    };
    let proxy: T = T::builder(conn)
        .destination(name)?
        .build().await?;

    Ok(proxy)
}

pub async fn currently_playing<
    'a,
    T: ProxyImpl<'a> + From<Proxy<'a>> + Clone
>(conn: &Connection) -> Result<T> {
    let proxies: Vec<T> = all(conn).await?;

    let proxy = futures::stream::iter(&proxies)
        .filter(|it| Box::pin(is_playback_status(it.inner(), PlaybackStatus::Playing)))
        .next()
        .await
        .cloned()
        .ok_or(anyhow!("No currently active player found!"))?;

    Ok(proxy)
}

async fn is_playback_status(it: &Proxy<'_>, playback_status: PlaybackStatus) -> bool {
    let result: Result<bool> = async {
        let status = PlaybackStatus::from(it.get_property::<String>("PlaybackStatus").await?.as_str()) == playback_status;
        Ok(status)
    }.await;
    result.unwrap_or(false)
}


#[cfg(test)]
mod test {
    use anyhow::anyhow;
    use log::info;
    use crate::media_player::MediaPlayer2Proxy;
    use test_log::test;

    #[test(tokio::test)]
    async fn get_all_players() -> anyhow::Result<()> {
        let conn = zbus::Connection::session().await?;

        let _: Vec<MediaPlayer2Proxy> = crate::sync::discovery::all(&conn).await?;

        Ok(())
    }

    #[test(tokio::test)]
    async fn get_first_player() -> anyhow::Result<()> {
        let conn = zbus::Connection::session().await?;

        let result = crate::sync::discovery::first::<MediaPlayer2Proxy>(&conn).await;

        match result {
            Err(e) => {
                assert!(e.to_string().eq("No MPRIS2 instances found."));
            }
            Ok(proxy) => {
                info!("Player identity: {:?}", proxy.identity().await?);
            }
        };


        Ok(())
    }

    #[test(tokio::test)]
    async fn get_player_by_name() -> anyhow::Result<()> {
        let conn = zbus::Connection::session().await?;

        let result = crate::sync::discovery::by_name::<MediaPlayer2Proxy>(&conn, "com.example.application").await;

        match result {
            Err(e) => {
                return if let zbus::Error::Names(..) = e {
                    Ok(())
                } else {
                    Err(anyhow!(e))
                }
            }
            Ok(proxy) => {
                info!("Player identity: {:?}", proxy.identity().await?);
            }
        };

        Ok(())
    }

    #[test(tokio::test)]
    async fn get_playing() -> anyhow::Result<()> {
        let conn = zbus::Connection::session().await?;

        let result: anyhow::Result<MediaPlayer2Proxy> = crate::sync::discovery::currently_playing(&conn).await;

        match result {
            Err(e) => {
                assert!(e.to_string().starts_with("No currently active player found"))
            }
            Ok(proxy) => {
                info!("identity={:?}, destination={:?}", proxy.identity().await?, proxy.inner().destination());
            }
        }

        Ok(())
    }
}