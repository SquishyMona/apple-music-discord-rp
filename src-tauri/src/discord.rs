use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::sync::Mutex;

const DISCORD_APP_ID: &str = "1459214232049549414";

pub struct DiscordClient {
    client: Mutex<DiscordIpcClient>,
}

impl DiscordClient {
    pub fn new() -> Self {
        println!("\nDISCORD CLIENT:::Initializing Discord IPC client");
        let mut client = DiscordIpcClient::new(DISCORD_APP_ID);
        match client.connect() {
            Ok(_) => println!("DISCORD CLIENT:::Connected to Discord IPC."),
            Err(e) => println!("DISCORD CLIENT:::Failed to connect to Discord IPC: {}", e),
        }

        Self {
            client: Mutex::new(client),
        }
    }

    pub fn set_activity(
        &self,
        state: &str,
        title: &str,
        artist: &str,
        album: &str,
        public_url: &str,
        start_ts: i64,
        end_ts: i64,
    ) {
        let mut activity = activity::Activity::new();
        if state == "paused" {
            activity = activity
                .activity_type(activity::ActivityType::Listening)
                .details(title)
                .state(artist)
                .assets(
                    activity::Assets::new()
                        .large_image(public_url)
                        .large_text(album)
                )
                .status_display_type(activity::StatusDisplayType::Details);

        }
        else {
            activity = activity
            .activity_type(activity::ActivityType::Listening)
            .details(title)
            .state(artist)
            .assets(
                activity::Assets::new()
                    .large_image(public_url)
                    .large_text(album)
            )
            .timestamps(
                activity::Timestamps::new()
                    .start(start_ts)
                    .end(end_ts),
            )
            .status_display_type(activity::StatusDisplayType::Details);
        }

        let _ = self
            .client
            .lock()
            .unwrap()
            .set_activity(activity);
    }

    pub fn clear(&self) {
        let _ = self.client.lock().unwrap().clear_activity();
    }
}
