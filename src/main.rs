use dbus::{
    arg,
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
};
use notify_rust::Notification;
use std::collections::HashMap;
use color_eyre::eyre::Result;
use std::time::Duration;

fn get_media_player(c: &Connection) -> Result<String> {
    let p = c.with_proxy("org.freedesktop.DBus", "/", Duration::from_millis(5000));

    let (names,): (Vec<String>,) = p.method_call("org.freedesktop.DBus", "ListNames", ())?;

    let mut player_name = String::new();
    for name in names {
        if name.starts_with("org.mpris.MediaPlayer2") {
            player_name = name;
        }
    }
    Ok(player_name)
}

fn get_current_track_info(
    c: &Connection,
    player_name: String,
) -> Result<()> {
    let p = c.with_proxy(
        player_name,
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000),
    );
    let metadata: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>> =
        p.get("org.mpris.MediaPlayer2.Player", "Metadata")?;

    let title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
    if let Some(title) = title {
        let track_title = title;
        Notification::new()
            .summary("Currently Playing")
            .body(track_title)
            .show()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let c = Connection::new_session()?;

    let player_name = get_media_player(&c).unwrap();
    let _ = get_current_track_info(&c, player_name);

    Ok(())
}
