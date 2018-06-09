#[macro_use]
extern crate structopt;
use structopt::StructOpt;

// For now, it's basically copied from the original:
// https://github.com/naelstrof/slop/blob/master/src/main.cpp

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// An application that queries for a selection from the user
/// and prints the region to stdout.
struct Opt {
    /// Sets the xdisplay to use
    #[structopt(short = "x", long = "xdisplay")]
    xdisplay: Option<String>,

    /// Sets the output format for slop.
    /// Format specifiers are  %x, %y, %w, %h, %i, %c, and %g.
    /// If actual percentage signs are desired in output,
    /// use a double percentage sign like so `%%`.
    #[structopt(short = "f", long = "format")]
    format: Option<String>,

    /// Sets the selection rectangle's thickness.
    #[structopt(short = "b", long = "bordersize")]
    bordersize: Option<f64>,

    /// Sets the padding size for the selection, this can be negative.
    #[structopt(short = "p", long = "padding")]
    padding: Option<f64>,

    /// How far in pixels the mouse can move after clicking,
    /// and still be detected as a normal click instead of a click-and-drag.
    /// Setting this to 0 will disable window selections.
    /// Alternatively setting it to 9999999 would force a window selection.
    #[structopt(short = "t", long = "tolerance")]
    tolerance: Option<f64>,

    /// Sets the selection rectangle's color. Supports RGB or RGBA input.
    /// Depending on the system's window manager/OpenGL support,
    /// the opacity may be ignored.
    #[structopt(short = "c", long = "color")]
    color: Option<String>,

    /// This sets the vertex shader, and fragment shader combo to use
    /// when drawing the final framebuffer to the screen.
    /// This obviously only works when OpenGL is enabled.
    /// The shaders are loaded from ~/.config/maim.
    /// See https://github.com/naelstrof/slop for more information
    /// on how to create your own shaders.
    #[structopt(short = "r", long = "shader")]
    shader: Option<String>,

    /// Sets the level of aggressiveness when trying to remove window decorations.
    /// `0' is off, `1' will try lightly to remove decorations,
    /// and `2' will recursively descend into the root tree
    /// until it gets the deepest available visible child under the mouse.
    /// Defaults to `0'.
    #[structopt(short = "n", long = "nodecorations", default_value = "0")]
    nodecorations: u32,

    /// Instead of outlining a selection, maim will highlight it instead.
    /// This is particularly useful if the color is set to an opacity lower than 1.
    #[structopt(short = "l", long = "highlight")]
    highlight: bool,

    /// Disable any unnecessary cerr output. Any warnings or info simply won't print.
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Disables the ability to cancel selections with the keyboard.
    #[structopt(short = "k", long = "nokeyboard")]
    nokeyboard: bool,

    /// Disables graphics hardware acceleration.
    #[structopt(short = "o", long = "noopengl")]
    noopengl: bool,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
