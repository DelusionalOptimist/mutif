use color_eyre::eyre::Result;
use dbus::{
    arg,
    blocking::{stdintf::org_freedesktop_dbus::Properties, Connection},
};
use notify_rust::Notification;
use std::collections::HashMap;
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

fn get_current_track_info(c: &Connection, player_name: String) -> Result<()> {
    let p = c.with_proxy(
        player_name,
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000),
    );
    let metadata: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>> =
        p.get("org.mpris.MediaPlayer2.Player", "Metadata")?;

    let title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
    let album: Option<&String> = arg::prop_cast(&metadata, "xesam:album");
    let image: Option<&String> = arg::prop_cast(&metadata, "mpris:artUrl");

    let artist = &metadata["xesam:artist"];
    let iter = artist.0.as_iter().unwrap();
    let mut artists = String::new();
    for name in iter {
        artists.push_str(name.as_str().unwrap());
    }

    if let (Some(title), Some(album), Some(image)) = (title, album, image) {
        let track_title = title;
        let album_name = album;
        let art = image;
        Notification::new()
            .summary("Currently Playing")
            .body(&format!(
                "<b>{}</b>\nby {}\nin {}",
                track_title, artists, album_name
            ))
            .image_path(art)
            .show()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let c = Connection::new_session()?;

    let player_name = get_media_player(&c)?;
    let _ = get_current_track_info(&c, player_name);

    Ok(())
}
