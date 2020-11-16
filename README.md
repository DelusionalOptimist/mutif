# mutif
A simple WIP 'notifcation daemon/script' which will work with media players that implement the [Media Player 2](https://specifications.freedesktop.org/mpris-spec/latest/Player_Interface.html) interface.

### Dependencies
- pkg-config
- libdbus-1-dev
- libdbus-1-3

### How to use
- Open spotify (web-app or desktop-app) and play a title.
- Make sure you have a notification daemon installed and it is running. See [this](https://wiki.archlinux.org/index.php/Desktop_notifications) for some help.
- In the project directory, run:
```cargo run```
* This should show a notification showing the current playing track's title. 

### TODO:
- [ ] Add artist name, album name, cover art in notification message.
- [ ] Make this a daemon.
- [ ] Make this good enough for daily use :P
