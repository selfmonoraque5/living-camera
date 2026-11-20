//! Living Camera on Horizon's mod API: the hooks that feed the camera in
//! `camera.rs` (the player's movement, turning, health and aim) and apply what
//! it asks (the camera shake, the hand's pose and the field of view).
use mod_api::client::camera::UseItem;
use mod_api::client::first_person::Hand;
use mod_api::{
    self, CameraShakeEvent, ClientHost, ClientTickPostEvent, ComputeFovModifierEvent, Flow,
    HealthHudEvent, Hooks, KeyRegistration, Registrar, RegistrationContext, RenderFramePreEvent,
    RenderHandEvent, SettingChangedEvent, SettingSpec, Text,
};
use std::cell::Cell;

mod camera;

use camera::{OPTS, Settings};

thread_local! {
    /// Whether the player's eyes are under water, as the health HUD last said.
    static UNDER_WATER: Cell<bool> = const { Cell::new(false) };
}

/// Setting `key` as the camera reads it (its value, not the page's choice index).
fn value(host: &dyn ClientHost, key: &str) -> Option<String> {
    let opt = OPTS.iter().find(|opt| opt.key == key)?;
    let at = usize::try_from(host.setting_choice(key)?).ok()?;
    opt.choices.get(at).map(|c| c.0.to_string())
}

/// The settings, read from the page once, then kept.
fn settings(host: &dyn ClientHost) -> Settings {
    camera::settings(|key| value(host, key))
}

#[derive(Default)]
struct LivingCamera;

#[mod_api::horizon_mod]
impl Hooks for LivingCamera {
    /// The settings page (one choice per option) and the toggle key.
    fn register(&self, r: &mut dyn Registrar, _: &RegistrationContext) {
        let defaults = Settings::default();
        for (i, opt) in OPTS.iter().enumerate() {
            let options = opt.choices.iter().map(|(v, _)| camera::choice_key(opt.key, *v));
            let default = defaults.choice_index(i).parse().unwrap_or(0);
            let _ = r.register_setting(SettingSpec::choice(
                opt.key,
                camera::label_key(opt.key),
                options,
                default,
            ));
        }
        r.register_key_mapping(
            KeyRegistration::new(camera::TOGGLE_KEY, camera::TOGGLE_KEY_DEFAULT)
                .label(camera::TOGGLE_KEY_LABEL_KEY),
        );
    }

    /// The player changed an option: a preset sets its values, a hand change
    /// makes the preset Custom.
    fn setting_changed(&self, host: &mut dyn ClientHost, event: &mut SettingChangedEvent<'_>) {
        let mut settings = Settings::from_choices(|key| host.setting_choice(key));
        for (key, index) in settings.changed(event.key) {
            host.set_setting(camera::ID, key, &index);
        }
        camera::set_settings(settings);
    }

    /// Each tick: how the player moved, and the toggle key.
    fn client_tick_post(&self, host: &mut dyn ClientHost, _: &mut ClientTickPostEvent) {
        while host.consume_click(camera::ID, camera::TOGGLE_KEY) {
            let mut settings = settings(&*host);
            settings.toggle();
            host.set_setting(camera::ID, OPTS[camera::o::ENABLED].key, &settings.choice_index(camera::o::ENABLED));
            camera::set_settings(settings);
            let _ = host.chat_line(Text::literal(camera::toggled_line(settings.enabled())));
        }
        let reads: &dyn ClientHost = &*host;
        let player = reads.local_player().map(|p| p.entity());
        let at = player.and_then(|e| Some((e.position(reads)?, e.rotation(reads)?)));
        match at {
            Some((pos, [yaw, _])) => camera::tick(Some(pos), yaw, UNDER_WATER.get()),
            None => camera::tick(None, 0.0, false),
        }
    }

    /// Each frame: the springs move toward what the movement and turning ask.
    fn render_frame_pre(&self, host: &mut dyn ClientHost, event: &mut RenderFramePreEvent) {
        settings(&*host);
        let reads: &dyn ClientHost = &*host;
        let view = reads
            .local_player()
            .filter(|_| reads.playing())
            .and_then(|p| p.entity().rotation(reads));
        camera::frame(event.elapsed.as_secs_f32(), view);
    }

    /// The camera's lean, tilt, dip, bob and breath, added to other mods' shakes.
    fn camera_shake(&self, _: &dyn ClientHost, event: &mut CameraShakeEvent<'_>) {
        let pose = camera::pose();
        event.shake.pitch += pose.pitch;
        event.shake.roll += pose.roll;
        event.shake.offset[1] += pose.rise;
    }

    /// The first-person hand lags behind the view and leans with the camera;
    /// the off hand mirrors the main one.
    fn render_hand(&self, _: &dyn ClientHost, event: &mut RenderHandEvent<'_, '_>) -> Flow {
        let pose = camera::pose();
        let side = if event.render.hand == Hand::Main { 1.0 } else { -1.0 };
        event.render.translation[0] += pose.hand[0];
        event.render.translation[1] += pose.hand[1];
        event.render.rotation[2] += pose.hand_roll * side;
        Flow::Continue
    }

    /// Whether the player flies in creative or aims a bow or spyglass; Speed
    /// FOV widens the view past walking speed.
    fn compute_fov_modifier(&self, _: &dyn ClientHost, event: &mut ComputeFovModifierEvent<'_>) {
        let aiming = matches!(event.player.using, Some((UseItem::Bow | UseItem::Spyglass, _)));
        camera::set_player(event.player.flying, aiming);
        event.modifier *= camera::pose().fov;
    }

    /// The health HUD, each frame it draws: the player's health (a drop shakes
    /// the camera), a living mount, and eyes under water. It draws nothing of its own.
    fn health_hud(
        &self,
        _: &mut dyn ClientHost,
        event: &mut HealthHudEvent<'_>,
    ) -> Option<Vec<mod_api::GuiDraw>> {
        let inputs = event.inputs;
        camera::health(inputs.health, inputs.vehicle_hearts > 0);
        UNDER_WATER.set(inputs.under_water);
        None
    }
}
