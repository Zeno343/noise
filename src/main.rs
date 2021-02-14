use amethyst::{
    prelude::*,
    assets::{
        AssetLoaderSystemData,
        Progress,
        ProgressCounter,
    },
    winit::{
        Event, 
        KeyboardInput, 
        VirtualKeyCode, 
        WindowEvent
    },
    renderer::{
        Format,
        palette::{
            Srgb,
            Pixel,
        },
        rendy::texture::TextureBuilder,
        Texture,
        types::TextureData,
    },
};

use noise::{
    NoiseFn, 
    Perlin
};

struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let noise = Perlin::new();
        let mut height = Vec::new(); 

        for i in 0..(10 * 20) {
            let x = (i % 20) as f64;
            let y = (i / 20) as f64;
            let noise_val = noise.get([x, y]);
            let normed = ((1.0 / 255.0) * noise_val) as u8;
            for _ in 0..3 {
                height.push(normed);
            }
        }

        let world = data.world;
        let height_map_builder = TextureBuilder::new()
            .with_raw_data(
                height,
                Format::Rgb8Uint,
            );

        let height_map = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
                loader.load_from_data(TextureData::from(height_map_builder), &mut ProgressCounter::new(),
            )
        });
    }

    fn handle_event(
        &mut self, 
        _: StateData<'_, GameData<'_, '_>>, 
        event: StateEvent
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            match event {
                 Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { 
                            virtual_keycode: 
                                Some(VirtualKeyCode::Escape),
                                .. 
                        }, ..
                    } |
                    WindowEvent::CloseRequested => Trans::Quit,
                    _ => Trans::None,
                },
                _ => Trans::None,
            }
        } else {
            Trans::None
        }
    }
}

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config/display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow
                    ::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(
        assets_dir, 
        GameState, 
        game_data
    )?;

    game.run();
    Ok(())
}
