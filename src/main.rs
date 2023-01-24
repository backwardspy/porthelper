use std::{fs, path::PathBuf};

use catppuccin::{
    css_colors::{self, percent, Color},
    Flavour,
};
use clap::{Parser, ValueEnum};
use color_eyre::{eyre::eyre, Result};
use handlebars::{handlebars_helper, Handlebars};
use serde::Serialize;
use titlecase::titlecase;

#[derive(Clone, Copy, ValueEnum)]
enum CliFlavour {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl From<CliFlavour> for Flavour {
    fn from(flavour: CliFlavour) -> Self {
        match flavour {
            CliFlavour::Latte => Flavour::Latte,
            CliFlavour::Frappe => Flavour::Frappe,
            CliFlavour::Macchiato => Flavour::Macchiato,
            CliFlavour::Mocha => Flavour::Mocha,
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    template_file: PathBuf,

    #[arg(value_enum)]
    flavour: CliFlavour,
}

#[derive(Serialize)]
struct Context {
    flavour: &'static str,
    text: String,
    base: String,
    mantle: String,
    crust: String,
    rosewater: String,
    mauve: String,
    blue: String,
    sky: String,
    teal: String,
    peach: String,
    green: String,
    green1: String,
    green2: String,
    yellow: String,
    pink: String,
    red: String,
    red1: String,
    red2: String,
    maroon: String,
    surface0: String,
    surface1: String,
    surface2: String,
    overlay0: String,
}

fn hex(rgb: css_colors::RGB) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        rgb.r.as_u8(),
        rgb.g.as_u8(),
        rgb.b.as_u8()
    )
}

impl From<Flavour> for Context {
    fn from(flavour: Flavour) -> Self {
        let colours = flavour.colours();

        let base: css_colors::RGB = colours.base.into();
        let red: css_colors::RGB = colours.red.into();
        let green: css_colors::RGB = colours.green.into();

        let red1 = red.mix(base, percent(20)).to_rgb();
        let red2 = red.mix(base, percent(40)).to_rgb();

        let green1 = green.mix(base, percent(20)).to_rgb();
        let green2 = green.mix(base, percent(40)).to_rgb();

        Self {
            flavour: flavour.name(),
            text: colours.text.hex(),
            base: colours.base.hex(),
            mantle: colours.mantle.hex(),
            crust: colours.crust.hex(),
            rosewater: colours.rosewater.hex(),
            mauve: colours.mauve.hex(),
            blue: colours.blue.hex(),
            sky: colours.sky.hex(),
            teal: colours.teal.hex(),
            peach: colours.peach.hex(),
            green: colours.green.hex(),
            green1: hex(green1),
            green2: hex(green2),
            yellow: colours.yellow.hex(),
            pink: colours.pink.hex(),
            red: colours.red.hex(),
            red1: hex(red1),
            red2: hex(red2),
            maroon: colours.maroon.hex(),
            surface0: colours.surface0.hex(),
            surface1: colours.surface1.hex(),
            surface2: colours.surface2.hex(),
            overlay0: colours.overlay0.hex(),
        }
    }
}

handlebars_helper!(title: |s: String| titlecase(&s));

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let template = fs::read_to_string(&cli.template_file).map_err(|e| {
        eyre!(
            "Failed to read template file from \"{}\": {e}",
            cli.template_file.to_string_lossy()
        )
    })?;
    let flavour: Flavour = cli.flavour.into();

    let mut hb = Handlebars::new();
    hb.set_strict_mode(true);
    hb.register_helper("titlecase", Box::new(title));

    let output = hb.render_template::<Context>(&template, &flavour.into())?;

    println!("{output}");

    Ok(())
}
