extern crate self as gimoji;

mod colors;
mod emoji;
mod search_entry;
mod selection_view;
mod terminal;

use anyhow::{bail, Context};
use arboard::Clipboard;
use clap::{command, Parser, Subcommand, ValueEnum};
use colors::Colors;
use std::{
    fmt::Debug,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, BufWriter, ErrorKind, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    process,
};
use terminal::{EventResponse, Terminal};

/// Select emoji for git commit message.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initialize gimoji as a commit message (`prepare-commit-msg`) hook.
    Init {
        /// Force initialize hook, use with caution
        #[arg(short, long)]
        force: bool,
    },
    /// Select and copy an emoji to clipboard.
    Copy {
        #[arg(long)]
        color_scheme: Option<ColorScheme>,
    },
    /// Run as git hook
    Hook {
        #[arg()]
        msg_file: PathBuf,
        #[arg()]
        msg_source: Option<MessageSource>,
        #[arg(long)]
        color_scheme: Option<ColorScheme>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum MessageSource {
    Message,
    Template,
    Merge,
    Squash,
    Commit,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum ColorScheme {
    Light,
    Dark,
}

impl From<ColorScheme> for Colors {
    fn from(c: ColorScheme) -> Self {
        match c {
            ColorScheme::Dark => Colors::DARK,
            ColorScheme::Light => Colors::LIGHT,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let get_emoji_factory = |color_scheme| {
        move || {
            let colors = Colors::from(get_color_scheme(color_scheme));
            select_emoji(colors)
        }
    };

    match args.cmd {
        Command::Init { force } => install_hook(force),
        Command::Copy { color_scheme } => {
            let Some(emoji) = get_emoji_factory(color_scheme)()? else {
                return Ok(());
            };
            println!("Copied {emoji} to the clipboard");
            copy_to_clipboard(emoji)
        }
        Command::Hook {
            msg_file,
            msg_source,
            color_scheme,
        } => {
            match msg_source {
                None | Some(MessageSource::Message | MessageSource::Merge) => {
                    prepend_emoji(&msg_file, get_emoji_factory(color_scheme))
                }
                Some(MessageSource::Template | MessageSource::Squash | MessageSource::Commit) => {
                    // We do not support any operations for these message types
                    Ok(())
                }
            }
        }
    }
}

fn select_emoji(colors: Colors) -> anyhow::Result<Option<&'static str>> {
    let mut terminal = Terminal::new(colors)?;
    loop {
        let response = terminal.render_ui()?;
        match response {
            EventResponse::Noop => {}
            EventResponse::EmojiSelected(emoji) => return terminal.reset().map(|()| Some(emoji)),
            EventResponse::Exit => return terminal.reset().map(|()| None),
        }
    }
}

fn install_hook(force: bool) -> anyhow::Result<()> {
    fs::create_dir_all(HOOK_FOLDER).context("Failed to create hooks dir")?;
    let file_path = Path::new(HOOK_FOLDER).join(PRE_COMMIT_MSG_HOOK);

    let mut options = OpenOptions::new();
    options.write(true).create(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o744);
    }

    let file = match options.open(&file_path) {
        Ok(f) => f,
        Err(e) if e.kind() == ErrorKind::AlreadyExists && !force => {
            bail!(
                "Failed to create `{}` as it already exists. Use -f to force overwrite it.",
                file_path.display()
            )
        }
        Err(e) => return Err(anyhow::anyhow!(e)).context("Failed to create hook file"),
    };

    let mut writer = BufWriter::new(file);
    writer
        .write_all(HOOK_HEADER.as_bytes())
        .context("Failed to write hook header")?;
    writer
        .write_all(HOOK_CMD.as_bytes())
        .context("Failed to write hook command")?;
    writer.flush().context("Failed to flush hook buffer")?;

    Ok(println!("Hooked gimoji with git successfully!"))
}

/// Copy the text to the clipboard.
///
/// This function exits the process and never returns because on some platforms (X11, Wayland)
/// clipboard data is only available for as long as the process that "owns" it is alive, in which
/// case this function will spawn a background task to host the clipboard data.
///
/// Note that it is possible to make it work without exiting the process, but it would require an
/// `unsafe { fork() }`. However, in this program this is simply not needed.
fn copy_to_clipboard(emoji: &str) -> anyhow::Result<()> {
    macro_rules! clipboard {
        () => {
            Clipboard::new()
                .context("Failed to create clipboard instance")?
                .set()
        };
    }

    macro_rules! paste_text {
        ($set:expr) => {
            $set.text(emoji)
                .context("Failed to copy emoji to clipboard")?
        };
    }

    cfg_if::cfg_if! {
        if #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "illumos",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "solaris"
        ))]
        {
            use arboard::SetExtLinux;
            nix::unistd::daemon(false, false).context("Failed to daemonize process")?;
            paste_text!(clipboard!().wait())
        } else {
            paste_text!(clipboard!())
        }
    }

    process::exit(0)
}

// Color scheme selection. Precedence: env, arg, detection, default.
fn get_color_scheme(color_scheme_arg: Option<ColorScheme>) -> ColorScheme {
    std::env::var("GIMOJI_COLOR_SCHEME")
        .ok()
        .and_then(|s| match s.as_str() {
            "light" => Some(ColorScheme::Light),
            "dark" => Some(ColorScheme::Dark),
            _ => None,
        })
        .or(color_scheme_arg)
        .unwrap_or_else(|| {
            terminal_light::luma()
                .map(|l| {
                    if l > 0.6 {
                        ColorScheme::Light
                    } else {
                        ColorScheme::Dark
                    }
                })
                .unwrap_or_else(|e| {
                    eprintln!("WARNING: Failed to detect terminal luma: {e}. Assuming dark.");

                    ColorScheme::Dark
                })
        })
}

fn prepend_emoji(
    path: &Path,
    get_emoji: impl FnOnce() -> anyhow::Result<Option<&'static str>>,
) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .context("Failed to open commit msg file in r/w mode")?;

    let file_size = file
        .metadata()
        .context("Failed to get commit msg file metadata")?
        .len() as usize;

    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader
        .read_line(&mut content)
        .context("Failed to read first line in commit msg file")?;

    if !content.is_empty() {
        // FIXME: There has to be a faster way to detect an emoji.
        for emoji in emoji::EMOJIS {
            if content.contains(emoji.emoji) || content.contains(emoji.code) {
                // The commit shortlog already contains an emoji.
                return Ok(());
            }
        }
    }

    let Some(emoji) = get_emoji()? else {
        return Ok(());
    };

    let mut content = content.into_bytes();
    content.reserve(file_size - content.len());
    reader
        .read_to_end(&mut content)
        .context("Failed to read rest of the commit msg file")?;
    reader
        .seek(SeekFrom::Start(0))
        .context("Failed to seek to start of commit msg file")?;

    let mut writer = BufWriter::new(reader.into_inner());
    write!(&mut writer, "{emoji} ").context("Failed to write emoji to buffer")?;
    writer
        .write_all(&content)
        .context("Failed to write commit message to buffer")?;
    writer.flush().context("Failed to flush commit msg buffer")
}

const HOOK_FOLDER: &str = ".git/hooks";
const PRE_COMMIT_MSG_HOOK: &str = "prepare-commit-msg";
const HOOK_HEADER: &str = "#!/usr/bin/env bash\n# gimoji as a commit hook\n";
const HOOK_CMD: &str = r#"gimoji hook $1 $2"#;
