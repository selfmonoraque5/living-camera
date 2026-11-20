<p align="center"><img src="icon.png" width="128" alt="Living Camera icon"></p>

# Living Camera (Horizon mod)

A camera that moves with you. It leans into strafes, tips as you jump and fall, sways on fast turns, dips when you
land, bobs with your steps and breathes while you stand still. Everything runs on springs, so the camera eases into
each motion and settles back with a little weight instead of snapping. Every effect has its own strength.

Client side only: it works in single player and on any server, and the server never knows.

## Effects

| Effect | What it does | Default |
|---|---|---|
| Strafe lean | Rolls the camera into sideways movement | 100% |
| Forward lean | A slight tip forward while running | 100% |
| Turn sway | Fast mouse turns tilt the view | 100% |
| Jump and fall tilt | The view lifts going up and dips coming down | 100% |
| Landing dip | A hard landing bumps the camera down; it springs back | 100% |
| Damage shake | Getting hit jolts the camera, harder for bigger hits, and it rings out | 100% |
| Step bob | A soft bounce and sway in step with your walking, on top of vanilla's | 50% |
| Idle breathing | A slow rise and fall while standing still | Off |
| Speed FOV | The view widens past walking speed, up to 15% | Off |
| Hand sway | The held item lags behind fast turns and leans with the camera | 100% |

## Settings

Options > Mods > Living Camera > **Config**. Each button steps through its values.

- **Living Camera**: ON / OFF, everything at once.
- **Preset**: one click sets the effects, smoothness and bounce, then fine-tune anything (the preset then reads
  *Custom*):

  | Preset | Feel |
  |---|---|
  | Subtle | A light touch: half-strength lean and sway, a little bob |
  | Default | The defaults below |
  | Cinematic | Stronger lean and sway, floaty springs with more bounce, breathing and speed FOV on: for videos |
  | PvP | Few distractions: small lean, no bob or tilt, snappy springs, a steady view while aiming |

- **Overall intensity**: 25% to 300%.
- **Each effect above**: OFF, 25, 50, 75, 100, 150 or 200%.
- **Smoothness**: Snappy, Normal, Smooth or Floaty (how quickly the camera follows).
- **Bounce**: None, Low, Normal, High or Very high (how much it overshoots before settling).
- **While swimming / While flying (creative)**: how much of the effects stay, OFF to 100%.
- **While riding mounts**: horses, camels and other living mounts, OFF to 100% (default 75%).
- **While aiming (bow, spyglass)**: how much stays while you draw a bow or look through a spyglass, so your aim is
  steady, OFF to 100% (default 25%).
- **Fade out when fast**: Never, or above 10, 12, 16 or 24 blocks a second. Elytra flight, boats and minecarts fade
  the effects out, so the mod stays out of the way of flight mods such as Gyro.
- **Lean direction**: Normal or Inverted.

A **Toggle Living Camera** key turns the mod on and off in game; it starts unbound, set it in Options > Controls. Horizon
also scales camera motion by Accessibility > **Distortion Effects**: at 0% the camera effects disappear.

Living Camera 2.0 is made for Horizon's mod API, so its settings start from the defaults again once: the 1.x
settings files aren't read.

## Install

Made for Horizon **Nightly 2026-11-19 · f00125f**.
It uses only the mod API, so the launcher also builds it on later Horizon builds where nothing it uses
changed.

From GitHub:

1. Download `living_camera-2.0.0.hmod` from the [latest release](https://github.com/selfmonoraque5/living-camera/releases/latest).
2. Put it in the `horizon-native-mods` folder of your game folder, not in `mods`. On Windows that is
   `%APPDATA%\horizon\horizon-native-mods`.
3. In the launcher, open the Mods page and click **Build** (the first build downloads the mod SDK and takes a few
   minutes). After a build the launcher can look stuck for a while as it cleans its cache: let it finish.
4. When it says **Up to date**, play.

Once it is listed in the Horizon registry: Mods, find **Living Camera**, Install.

To build a local copy: `horizon-launcher native-build <this folder> --instance <instance> --trust-native-code`.

## Compatibility

A native mod on Horizon's mod API (`horizon-mod.toml`, `[native]`). It uses only the API's hooks and reads, no game
internals, so the launcher also builds it on later Horizon builds where nothing it uses changed. Camera shakes from
other mods add up with this one.

## How it works

| File | What it is |
|---|---|
| `horizon-mod.toml` | The manifest: id, version and the Horizon build it is made for |
| `src/lib.rs` | The hooks: settings and toggle key (`register`, `setting_changed`), movement each tick (`client_tick_post`), turning each frame (`render_frame_pre`), the camera (`camera_shake`), the hand (`render_hand`), Speed FOV and aiming (`compute_fov_modifier`), health, mounts and water (`health_hud`; it draws nothing) |
| `src/camera.rs` | The settings, the motion, the springs and the effects, with their tests |
| `assets/living_camera/lang/en_us.json` | The settings page's text |

Damage shake, mount riding and the swimming calm come from the health bar, so they don't act while the HUD is hidden
(F1) or in creative mode, where there is no health bar.

The mod reads only your own player's position, view, health and the health bar's water and mount state. It has no
network, file or process access beyond Horizon's own settings for the mod.

```sh
cargo test    # the motion, springs and settings
```

## Credits

Inspired by [Camera Overhaul](https://github.com/Mirsario/Minecraft-CameraOverhaul) by Mirsario. Living Camera is written from
scratch for Horizon and contains none of its code or assets; it is not made or endorsed by Mirsario.

## License

MIT, see [LICENSE](LICENSE).
