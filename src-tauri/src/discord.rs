use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::sync::Mutex;

const DISCORD_APP_ID: &str = "1459214232049549414";

pub struct DiscordClient {
    client: Mutex<DiscordIpcClient>,
}

impl DiscordClient {
    pub fn new() -> Self {
        let mut client = DiscordIpcClient::new(DISCORD_APP_ID);
        client.connect().ok();

        Self {
            client: Mutex::new(client),
        }
    }

    pub fn set_activity(
        &self,
        title: &str,
        artist: &str,
        album: &str,
        public_url: &str,
        start_ts: i64,
        end_ts: i64,
    ) {
        let activity = activity::Activity::new()
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
            );

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
