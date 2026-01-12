use std::process::Command;

pub fn get_track_info() -> Option<String> {
    let script = r#"
    if application "Music" is not running then
        return "NO_TRACK"
    end if
    tell application "Music"
        if not (exists current track) then
            return "NO_TRACK"
        end if

        set playerState to player state as string
        set trackName to name of current track
        set artistName to artist of current track
        set albumName to album of current track
        set durationSec to duration of current track
        set positionSec to player position

        set artPath to "/tmp/apple_music_artwork.png"
        try
            set rawData to data of artwork 1 of current track
            set outFile to open for access artPath with write permission
            set eof of outFile to 0
            write rawData to outFile
            close access outFile
        end try

        return playerState & "||" & trackName & "||" & artistName & "||" & albumName & "||" & durationSec & "||" & positionSec & "||" & artPath
    end tell
    "#;

    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .ok()?;

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
