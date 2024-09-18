use clap::{builder::{styling::{AnsiColor, Effects}, Styles}, Parser};
use color_print::cprintln;
use uuid::{Timestamp, Uuid};

#[derive(Parser)]
#[command(name = "ulidtools")]
#[command(bin_name = "ulidtools")]
#[command(author, version = env!("CARGO_PKG_VERSION"), about, long_about = None, styles = cli_styles())]
#[command(propagate_version = true, disable_help_subcommand = true)]
enum UlidToolsCli {
    /// Generate UUIDv7 and ULID
    Generate,
    /// Parse UUID or ULID
    Parse {
        /// UUID or ULID to parse
        input: String,
    },
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
struct UlidToolsParseArgs {
    #[arg(long)]
    manifest_path: Option<std::path::PathBuf>,
}

fn main() {
    match UlidToolsCli::parse() {
        UlidToolsCli::Generate => {
            let uuid = generate_uuid();
            let ulid = uuid_to_ulid(uuid).unwrap();

            cprintln!("<strong,m>ULID</>: <u>{}</u>", ulid);
            cprintln!("<strong,m>UUIDv7</>: <u>{}</u>", uuid);
            cprintln!("<strong,m>Timestamp</>: {}", ulid_to_timestamp(ulid));
        }
        UlidToolsCli::Parse { input } => {
            // Parse as UUID
            let uuid_parse = Uuid::parse_str(&input);
            match uuid_parse {
                Err(_) => {
                    // Parse as ULID
                    let ulid_parse = ulid::Ulid::from_string(&input);
                    match ulid_parse {
                        Err(_) => {
                            cprintln!("<red>Invalid input format</>");
                        }
                        Ok(ulid) => {
                            let uuid = ulid_to_uuid(ulid);
                            cprintln!("<strong,m>ULID</>: <u>{}</u>", ulid);
                            cprintln!("<strong,m>UUIDv7</>: <u>{}</u>", uuid);
                            cprintln!("<strong,m>Timestamp</>: {}", ulid_to_timestamp(ulid));
                        }
                    }
                }
                Ok(uuid) => {
                    match uuid_to_ulid(uuid) {
                        Ok(ulid) => {
                            cprintln!("<strong,m>ULID</>: {}", ulid);
                            cprintln!("<strong,m>UUIDv7</>: <u>{}</u>", uuid);
                            cprintln!("<strong,m>Timestamp</>: {}", ulid_to_timestamp(ulid));
                        }
                        Err(e) => {
                            cprintln!("<red>{}</>", e);
                        }
                    }
                }
            }
        }
    }
}

/// Generate v7 UUID for the current timestamp
///
/// # Examples
/// ```rust
/// use showtimes_shared::generate_uuid;
///
/// let uuid = generate_uuid();
/// println!("{}", uuid);
/// ```
pub fn generate_uuid() -> Uuid {
    let ts = Timestamp::now(uuid::timestamp::context::NoContext);
    Uuid::new_v7(ts)
}

/// Convert UUIDv7 to ULID
///
/// # Examples
/// ```rust
/// use showtimes_shared::{generate_uuid, uuid_to_ulid};
///
/// let uuid = generate_uuid();
/// let ulid = uuid_to_ulid(uuid);
///
/// println!("{}", ulid);
/// ```
pub fn uuid_to_ulid(uuid: Uuid) -> Result<ulid::Ulid, String> {
    if uuid.get_version_num() != 7 {
        Err(format!("Expected UUIDv7, got UUIDv{}", uuid.get_version_num()))
    } else {
        Ok(ulid::Ulid::from_bytes(*uuid.as_bytes()))
    }
}

/// Convert ULID to UUIDv7
///
/// # Examples
/// ```rust
/// use showtimes_shared::{generate_uuid, ulid_to_uuid, uuid_to_ulid};
///
/// let uuid_act = generate_uuid();
/// let ulid = uuid_to_ulid(uuid_act);
/// let uuid = ulid_to_uuid(ulid);
///
/// assert_eq!(uuid_act, uuid);
/// ```
pub fn ulid_to_uuid(ulid: ulid::Ulid) -> Uuid {
    let bita = ulid.to_bytes();
    Uuid::from_bytes(bita)
}

fn ulid_to_timestamp(ulid: ulid::Ulid) -> String {
    let ts = ulid.timestamp_ms();
    let parsed = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ts as i64);
    // MM dd, yyyy HH:mm:ss
    parsed.unwrap().format("%b %d, %Y %H:%M:%S").to_string()
}

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Magenta.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::BrightCyan.on_default())
}
