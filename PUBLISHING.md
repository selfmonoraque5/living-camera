# Publishing Living Camera to the Horizon registry

## 1. Put the source on GitHub

1. Create a public repository named `living-camera` at https://github.com/new (no README, the folder has one).
2. Click "uploading an existing file" and drag in everything in this folder except `living_camera-1.0.0.hnmod`:
   `Cargo.toml`, `compatibility.json`, `icon.png`, `LICENSE`, `README.md`, `.gitignore`, `PUBLISHING.md` and the
   whole `mixins` folder. Commit.
3. Check `mixins/living_camera/` shows all 12 files on GitHub.

## 2. Submit

Go to https://playhorizon.dev/mods/submit, sign in with GitHub, pick `living_camera-1.0.0.hnmod` and fill in the
page with the text below. The form uploads it and opens a pull request; the registry's checks build it on the newest
Nightly, then a reviewer reads the source.

## Page text

**Name:** Living Camera

**Summary:**
A camera that moves with you: it leans into strafes, sways on turns, dips on landing and bobs with your steps.

**Description:**

```
A camera that moves with you. It leans into strafes, tips as you jump and fall, sways on fast turns, dips when you land, bobs with your steps and breathes while you stand still. Everything runs on springs, so the camera eases into each motion and settles back with a little weight instead of snapping.

- **Strafe and forward lean**: the camera rolls into sideways movement and tips slightly while running.
- **Turn sway**: fast mouse turns tilt the view.
- **Jump and fall tilt**: the view lifts going up and dips coming down.
- **Landing dip**: a hard landing bumps the camera down, then it springs back.
- **Step bob**: a soft bounce in step with your walking, on top of vanilla's.
- **Idle breathing** and **Speed FOV** (off by default).
- **Hand sway**: the held item lags behind fast turns and leans with the camera.

Every effect has its own strength (OFF to 200%), plus overall intensity, smoothness and bounce presets, how much stays while swimming or flying, when fast travel fades it out (elytra, boats, minecarts, so it stays out of flight mods' way) and the lean direction. All in Options > Mods > Living Camera, with a key to toggle it in game.

Client side only: it works in single player and on any server. It reads only your own position, view and the block at your eyes, with no network or file access beyond its own settings.

Inspired by Camera Overhaul by Mirsario (https://github.com/Mirsario/Minecraft-CameraOverhaul). Written from scratch for Horizon: it contains none of Camera Overhaul's code or assets and is not made or endorsed by Mirsario.
```

**License:** MIT

**Source:** `https://github.com/selfmonoraque5/living-camera`

**Icon:** `icon.png`

**Categories:** utility, atmosphere

**Tags:** camera, immersion, client, animation, fov, view bobbing

## Later versions

Raise `version` in `mixins/living_camera/horizon-mixin.toml`, `Cargo.toml` and `VERSION` in `runtime.rs`, package a new
`.hnmod`, push the source, and add the version through the same form. A published file never changes.
