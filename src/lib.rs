extern crate open;
extern crate percent_encoding;

use std::panic;
use std::any::Any;
use percent_encoding::{utf8_percent_encode, QUERY_ENCODE_SET};

/// Search panic messages on stack overflow via duck duck go.
///
/// Indentical to `with_query(|msg| format!("!so {}", msg))`
pub fn enable() {
    with_query(|msg| format!("!so {}", msg));
}

/// Search panic messages via duck duck go using a custom query.
/// The passed closure receives the panic message and needs
/// to return a duck duck query.
///
/// # Example (searching on google)
///
/// ```
/// with_query(|msg| format!("!google {}", msg))
/// ```
pub fn with_query<F>(query_fn: F)
where
    F: Fn(&str) -> String + Send + Sync + 'static,
{
    let hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        hook(info);

        let payload = info.payload();
        let msg = Any::downcast_ref::<String>(payload);
        let msg = msg.as_ref().map(|s| s.as_str());
        let msg = msg.or_else(|| Any::downcast_ref::<&'static str>(payload).map(|s| *s));

        if let Some(msg) = msg {
            let query = query_fn(msg);
            let url = format!(
                "https://duckduckgo.com/?q={}",
                utf8_percent_encode(&query, QUERY_ENCODE_SET)
            );

            if let Err(e) = open::that(&url) {
                println!("Error opening {}: {}", url, e);
            }
        }
    }));
}
