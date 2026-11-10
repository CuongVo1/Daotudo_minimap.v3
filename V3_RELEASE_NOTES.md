# ĐẢO TỰ DO MiniMap V3 — source preview

Built on original V2 source. V2 position API, Leaflet map, overlay, navigation and old controls remain in place.

## Implemented
- Home, Server, Guide, Waypoint, Settings pages, tactical dark-red theme; keeps V2 MiniMap configuration and big map.
- Public SCUM-MAP POI: 123 markers across seven allowed categories, filter and search.
- Private POI: 7,933 markers across other categories, encrypted in `src-tauri/src/v3_private.bin` with AES-256-GCM and Argon2id (64 MiB, 3 passes). Unlock in Rust via `unlock_v3_poi`; no plaintext private POI or password in distributed source.
- Session-only private access; relocking removes private marker layers and application references. JavaScript memory cannot be guaranteed to be wiped by garbage collection.
- Game-coordinate waypoint form, persistence and fly-to.
- Windows system tray menu Open/Quit and close-to-tray behavior.

## IMPORTANT — pending verification / limitations
- **POI map alignment is not calibrated**. SCUM-MAP marker coordinates are provisionally mapped from 0..320 into the V2 Leaflet 256x256 CRS. Compare fixed landmarks before using positions as authoritative. A warning is shown in the map UI.
- The requested AI cinematic artwork has **not** been generated in this source preview. The existing map preview image is used as atmospheric background; replace or augment with commissioned artwork later.
- No Windows EXE or Rust build was run in this environment. Run GitHub Actions and inspect Rust compiler feedback before publishing.
- V2's original waypoint/route systems remain; new V3 waypoint form is an additional locally stored list, not a replacement.
- `teamA3` is a short password: offline brute-force remains possible. Never treat client-side encrypted data as a strong authorization boundary.
- Private data returned to JavaScript exists in memory while unlocked; session lock clears references and map layers, but cannot securely erase JS heap snapshots.
- CI may need Cargo.lock refresh due to newly added Rust dependencies. Run `cargo update` locally or allow CI to resolve the lockfile.
