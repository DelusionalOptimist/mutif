use dbus::{blocking::Connection, arg};
use std::time::Duration;
use std::collections::HashMap;
use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = Connection::new_session()?;
    let p = c.with_proxy("org.mpris.MediaPlayer2.firefox.instance28360", "/org/mpris/MediaPlayer2", Duration::from_millis(5000));
    use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

    let metadata: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>> = p.get("org.mpris.MediaPlayer2.Player", "Metadata")?;

    let title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
    if let Some(title) = title {
        let track_title = title;
        println!("The title is: {}", title);
        Notification::new()
            .summary("SPOTIFY")
            .body(track_title)
            .show()?;
    }

    Ok(())
}
