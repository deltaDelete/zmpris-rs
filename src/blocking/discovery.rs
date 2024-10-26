#![allow(dead_code)]

use anyhow::{anyhow, bail};
use zbus::blocking::{Connection, fdo::DBusProxy};
use zbus::Proxy;
use zbus::blocking::proxy::ProxyImpl;
use crate::shared::{PlaybackStatus, BASE_PATH};

pub fn all<
    'a,
    T: ProxyImpl<'a> + From<Proxy<'a>>
>(conn: &Connection) -> anyhow::Result<Vec<T>> {
    let dbus = DBusProxy::new(conn)?;
    let names = dbus.list_names()?
        .iter()
        .filter(|it| it.starts_with(BASE_PATH))
        .cloned()
        .collect::<Vec<_>>();
    let mut proxies: Vec<T> = Vec::new();
    for name in names {
        let proxy = T::builder(conn)
            .destination(name)?
            .build()?;
        proxies.push(proxy);
    }
    Ok(proxies)
}

pub fn by_name<
    'a,
    T: ProxyImpl<'a> + From<Proxy<'a>>
>(conn: &'a Connection, name: &'a str) -> zbus::Result<T> {
    let proxy: T = T::builder(conn)
        .destination(name)?
        .build()?;

    Ok(proxy)
}

pub fn first<
    'a,
    T: ProxyImpl<'a> + From<Proxy<'a>>
>(conn: &Connection) -> anyhow::Result<T> {
    let dbus = DBusProxy::new(conn)?;
    let Some(name) = dbus.list_names()?
        .iter().find(|it| it.starts_with(BASE_PATH))
        .cloned() else {
        bail!("No MPRIS2 instances found.");
    };
    let proxy: T = T::builder(conn)
        .destination(name)?
        .build()?;

    Ok(proxy)
}

pub fn currently_playing<
    'a,
    T: ProxyImpl<'a> + From<Proxy<'a>> + Clone
>(conn: &Connection) -> anyhow::Result<T> {
    let proxies: Vec<T> = all(conn)?;

    let proxy = proxies.iter()
        .find(|&it| is_playback_status(it.inner(), PlaybackStatus::Playing))
        .cloned()
        .ok_or(anyhow!("No currently active player found!"))?;

    Ok(proxy)
}

fn is_playback_status(it: &zbus::blocking::Proxy, playback_status: PlaybackStatus) -> bool {
    let result = || {
        let current_status = PlaybackStatus::from(
            it.get_property::<String>("PlaybackStatus")?.as_str()
        );
        anyhow::Ok(current_status == playback_status)
    };
    result().unwrap_or(false)
}


#[cfg(test)]
mod test {
    use log::info;
    use test_log::test;
    use zbus::blocking::Connection;
    use crate::blocking::MediaPlayer2Proxy;
    use crate::blocking::discovery::*;

    #[test]
    fn get_all_players() -> anyhow::Result<()> {
        let conn = Connection::session()?;

        let _: Vec<MediaPlayer2Proxy> = all(&conn)?;

        Ok(())
    }

    #[test]
    fn get_first_player() -> anyhow::Result<()> {
        let conn = Connection::session()?;

        let result = first::<MediaPlayer2Proxy>(&conn);

        match result {
            Err(e) => {
                assert!(e.to_string().eq("No MPRIS2 instances found."));
            }
            Ok(proxy) => {
                info!("Player identity: {:?}", proxy.identity()?);
            }
        };


        Ok(())
    }

    #[test]
    fn get_player_by_name() -> anyhow::Result<()> {
        let conn = Connection::session()?;

        let result = by_name::<MediaPlayer2Proxy>(&conn, "ru.deltadelete.something");

        match result {
            Err(e) => {
                return if let zbus::Error::Names(..) = e {
                    Ok(())
                } else {
                    Err(anyhow!(e))
                }
            }
            Ok(proxy) => {
                info!("Player identity: {:?}", proxy.identity()?);
            }
        };

        Ok(())
    }

    #[test]
    fn get_playing() -> anyhow::Result<()> {
        let conn = Connection::session()?;

        let result: anyhow::Result<MediaPlayer2Proxy> = currently_playing(&conn);

        match result {
            Err(e) => {
                assert!(e.to_string().starts_with("No currently active player found"))
            }
            Ok(proxy) => {
                info!("identity={:?}, destination={:?}", proxy.identity()?, proxy.inner().destination());
            }
        }

        Ok(())
    }
}