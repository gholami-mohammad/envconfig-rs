/// A trait for creating an instance of a type from environment variables.
///
/// Types that implement this trait should provide a way to initialize themselves
/// using values from the environment. This is useful for configuration purposes,
/// where settings can be provided through environment variables.
///
/// # Examples
///
/// ```
/// use env2config::FromEnv;
///
/// #[derive(FromEnv)]
/// struct Config {
///     #[env("DATABASE_URL")]
///     database_url: String,
///     #[env("HOST", "127.0.01")] // HOST is env variable name and 127.0.01 is default value if HOST is not provided
///     host: String,
/// }
///
/// fn main() {
///     let cfg = Config::from_env();
///     println!("{}", cfg.database_url);
/// }
/// ```
mod traits;
pub use env2config_derive::FromEnv;
pub use traits::FromEnv;
