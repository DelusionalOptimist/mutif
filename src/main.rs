use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("HELLO WORLD")
        .body("WUT!!!!")
        .show()?;
    Ok(())
}
