fn create_s6_mounted_draw(
    result: (),
    game: &crate::game::Game<'_>,
    camera: glam::DVec3,
    alpha: f64,
    out: &mut Vec<horizon_client::mesher::Vertex>,
) {
    crate::game::create_s5_piston::draw_mounted_item(game, camera, alpha, out);
    result
}
