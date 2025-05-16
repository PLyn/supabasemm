mod migrate_functions;
mod migrate_page;

pub use migrate_functions::check_cache_hit_ratio;
pub use migrate_page::MigratePage;
