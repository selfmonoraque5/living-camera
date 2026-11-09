// The FOV hook knows whether the player flies in creative: keep it. Speed FOV
// widens the view past walking speed.
fn living_camera_fov(
    game: &crate::game::Game<'_>,
    player: &horizon_client::camera::FovPlayer,
    modifier: &mut f32,
) {
    use crate::game::living_camera as m;
    if !game.native_on(m::ID) {
        return;
    }
    m::set_flying(player.flying);
    *modifier *= m::pose().fov;
}
