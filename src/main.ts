import { invoke } from "@tauri-apps/api/core";

let lastTrack: string = "";

setInterval(async () => {
  const result = await invoke<string | null>("get_current_track");
  if (!result || result === "NO_TRACK") return;

  if (result !== lastTrack) {
    lastTrack = result;
    console.log(`Now playing: ${result}`);
  }
}, 5000);