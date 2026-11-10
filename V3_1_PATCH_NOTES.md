# DAO TU DO V3.1 Hotfix

## Applied
- Force correct tab visibility for MiniMap workspace and other pages (CSS `[hidden]` specificity).
- Add AI-generated sidebar character artwork (cropped from generated V3.1 design image).
- Player marker: one cyan directional arrow; removes dot and stem; retains live yaw updates.
- Overlay iframe adds `v3overlay=1`; overlay-only N/S/E/W compass, hides XY button and coordinate/status display. Full big map remains unaffected.
- Preserve existing public/private POI data, Rust commands, API and GitHub Actions.

## Not verified
- Windows runtime / GitHub Actions installer build not executed here.
- SCUM-map POI coordinate calibration remains provisional from V3.
- Compass letters are screen-fixed, not a rotating compass rose.

## Update
Copy all files into the V3 repository preserving `.git`, commit, push, and build via GitHub Actions.
