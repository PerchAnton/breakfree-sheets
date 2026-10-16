use breakfree_sheets::{interaction::Session, office, persistence};
use std::path::PathBuf;
fn launch() -> Result<(), (i32, String)> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let mut session = Session::default();
    if let Some(action) = args.first() {
        match action.to_str() {
            Some("--capabilities") if args.len() == 1 => {
                println!("{}", office::capabilities());
                return Ok(());
            }
            Some("--version") if args.len() == 1 => {
                println!("BreakFree Sheets {}", office::VERSION);
                return Ok(());
            }
            Some("--new") if args.len() == 1 => {}
            Some("--open") if args.len() == 2 => {
                let path = PathBuf::from(&args[1]);
                if !path.is_absolute() {
                    return Err((2, "--open requires an absolute path".into()));
                }
                let book = persistence::load(&path).map_err(|e| (1, e))?;
                session = Session::new(book, Some(path));
            }
            Some("--help") if args.len() == 1 => {
                println!(
                    "breakfree-sheets [--new | --open <absolute_path> | --capabilities | --version]"
                );
                return Ok(());
            }
            _ => {
                return Err((
                    2,
                    "Use one action: --new, --open <absolute_path>, --capabilities or --version"
                        .into(),
                ));
            }
        }
    }
    #[cfg(feature = "gui")]
    {
        breakfree_sheets::gui::run(session).map_err(|e| (1, e.to_string()))?;
    }
    #[cfg(not(feature = "gui"))]
    {
        let _ = session;
        return Err((
            1,
            "This headless build has no GUI; use the default Cargo features for desktop launch"
                .into(),
        ));
    }
    #[allow(unreachable_code)]
    Ok(())
}
fn main() {
    if let Err((code, message)) = launch() {
        eprintln!("{message}");
        std::process::exit(code);
    }
}
