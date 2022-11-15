use argparse::{ArgumentParser, Store, StoreTrue};
use image::{ImageBuffer, Rgb};
use ray_tracer::benchmark::*;
use ray_tracer::render::*;
use ray_tracer::render_params::RenderParams;
use ray_tracer::render_params::DEFAULT_PARAMS;
use std::fs;

fn main() {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>>;
    let mut benchmark = false;
    let mut outfile: String = String::from("output.png");
    let mut use_default_params = true;
    let mut config_file: String = String::new();
    let mut aspect_ratio: f64 = DEFAULT_PARAMS.aspect_ratio;
    let mut width: u32 = DEFAULT_PARAMS.width;
    let mut max_depth: u32 = DEFAULT_PARAMS.max_depth;
    let mut params: RenderParams = DEFAULT_PARAMS;

    let desc = "Ray tracer based on \"Ray Tracing in One Weekend\" with multithreading support.\
        Allows for setting of common rendering params via command line, but\
        otherwise specify a configuration JSON file";

    {
        let mut ap = ArgumentParser::new();
        ap.set_description(desc);
        ap.refer(&mut benchmark).add_option(
            &["-b", "--benchmark"],
            StoreTrue,
            "Run a parallelized vs unparallelized rendering benchmark",
        );
        ap.refer(&mut outfile).add_option(
            &["-o", "--outfile"],
            Store,
            "Render a random scene and save as image in output",
        );
        ap.refer(&mut use_default_params).add_option(
            &["-d", "--default"],
            Store,
            "Render a random scene with default params and save as image in output",
        );
        ap.refer(&mut aspect_ratio).add_option(
            &["-a", "--aspect"],
            Store,
            "Sets the aspect ratio for rendering",
        );
        ap.refer(&mut width).add_option(
            &["-w", "--width"],
            Store,
            "Sets the width of the image and fixes the height as width / aspect ratio",
        );
        ap.refer(&mut max_depth).add_option(
            &["-m", "--maxdepth"],
            Store,
            "Sets the max rendering depth",
        );
        ap.refer(&mut config_file).add_option(
            &["-m", "--maxdepth"],
            Store,
            "Sets the max rendering depth",
        );
        ap.parse_args_or_exit()
    }

    if benchmark {
        println!("===========================Benchmark===========================\n");
        let (d1, d2) = compare_render_times();
        println!(
            "Benchmark statistics:\nUnparallelized: {:?}\nParallelized: {:?}",
            d1, d2
        );
    }

    params.width = width;
    params.aspect_ratio = aspect_ratio;
    params.max_depth = max_depth;

    if !use_default_params && config_file.len() > 0 {
        let config = fs::read_to_string(config_file).expect("Unable to read file on disk");

        params = serde_json::from_str(&config.as_str()).unwrap();
    }
    println!(
        "Starting render with the following render parameters:\n\n{}\n",
        &params
    );

    match parallelized_render_image(&params) {
        None => panic!("Something went wrong while rendering scene."),
        Some(res) => img = res,
    }
    match img.save(outfile) {
        Ok(_) => return,
        Err(e) => panic!("{}", e),
    }
}
