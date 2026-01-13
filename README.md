<img width="100" height="100" alt="AppleMusicRP" src="https://github.com/user-attachments/assets/28d2086e-ba4e-49a1-8e58-5e738e6ea2a7" />

# Apple Music Rich Presence for Discord

This project enables rich presence for Apple Music on Discord, allowing users to share their current listening activity with friends on the platform. Currently for macOS only, it uses AppleScript to fetch the currently playing track from Apple Music and updates the Discord Rich Presence accordingly.

### [Download in releases](https://github.com/SquishyMona/apple-music-discord-rp/releases)

## What It Does

- Fetches the currently playing track from Apple Music using AppleScript and sends the following to Discord:
  - Track title
  - Artist name
  - Album name
  - Album artwork (if available)
- Updates Discord Rich Presence in real-time as the track changes, is paused, fast-forwarded or rewinded.
- Rich presence will clear upon quitting the Music app
- Adds a menu bar icon to quit the app

## What It Doesn't

- Does not support Spotify or other music services.
- Does not support Windows or Linux (at the moment :stuck_out_tongue:).
- Does not show music playing from other devices (iPhone, iPad, etc.)

## Planned Features

- If paused for more then a set amount of time, clear the rich presence.
- Add link to track on Apple Music (if that's even possible).
- Maybe potentially possibly support for Windows and Linux (maybe).
- GUI for...something idk (maybe debug/error stuff)

enjoy my little app!! :blue_heart:
