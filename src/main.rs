use raytracer::scene::Scene;
use std::{env, path::Path};

const USAGE_STRING: &'static str =
    "Usage: raytracer scene_file output_file [xres] [yres] [samples]";

fn main() -> Result<(), &'static str> {
    // TODO: Implement main method.
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(USAGE_STRING)
    } 

    let scene_path = &args[1];
    let output_path = &args[2];

    let x_res = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(500);
    let y_res = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(500);
    let samples = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(500);

    let scene = Scene::from_json(scene_path).map_err(|_| USAGE_STRING)?;

    let image = scene.render(x_res, y_res, samples);

    image.save(Path::new(output_path)).map_err(|_| USAGE_STRING)?;

    Ok(())
}
