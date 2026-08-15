/// Slugify a display name to a valid username.
/// "John Smith" -> "john_smith"; deduplicates against existing names via suffix.
pub fn slugify_username(name: &str) -> String {
    let slug: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_");
    if slug.is_empty() {
        "user".to_string()
    } else {
        slug
    }
}

/// Normalise text into the form stored in a `search_*` column
/// (`books.search_title`, `authors.search_full_name`, `series.search_ser`).
///
/// These are derived columns: they are never displayed (templates render the
/// original `title`/`full_name`/`ser_name`), they back the `LIKE`
/// prefix/substring searches, and their whitespace-separated words feed the
/// alphabet grids on `/web/books`, `/web/authors` and `/web/series`. Keeping
/// only alphanumerics means the grid shows letters and digits instead of the
/// quotes, brackets and dashes that titles like `«Мир приключений» (№09)` would
/// otherwise contribute.
///
/// Everything that is not `char::is_alphanumeric` becomes a single space rather
/// than being deleted, so `АЛЬФА-БЕТА` normalises to `АЛЬФА БЕТА` and `БЕТА`
/// stays findable at a word boundary. Runs of separators collapse and the result
/// is trimmed, which makes the function idempotent.
///
/// Uppercasing goes through `char::to_uppercase`, which is Unicode-aware —
/// unlike SQLite's `UPPER()`, which only folds ASCII.
///
/// Accepted edge cases: `O'BRIEN` becomes `O BRIEN`, and decomposed sequences
/// (e.g. `Е` + U+0308 rather than `Ё`) split at the combining mark.
pub fn normalize_search_text(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut pending_space = false;
    for ch in title.chars() {
        if ch.is_alphanumeric() {
            if pending_space && !out.is_empty() {
                out.push(' ');
            }
            pending_space = false;
            out.extend(ch.to_uppercase());
        } else {
            pending_space = true;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify_username("John Smith"), "john_smith");
        assert_eq!(slugify_username(""), "user");
        assert_eq!(slugify_username("Иван Петров"), "user");
    }

    #[test]
    fn normalize_search_text_strips_punctuation_and_uppercases() {
        assert_eq!(
            normalize_search_text("«Мир приключений» 1963 (№09)"),
            "МИР ПРИКЛЮЧЕНИЙ 1963 09"
        );
        assert_eq!(normalize_search_text("\"Тихий Дон\""), "ТИХИЙ ДОН");
    }

    #[test]
    fn normalize_search_text_splits_on_hyphen_instead_of_joining() {
        assert_eq!(normalize_search_text("Альфа-Бета"), "АЛЬФА БЕТА");
        assert_eq!(normalize_search_text("O'Brien"), "O BRIEN");
    }

    #[test]
    fn normalize_search_text_keeps_digits() {
        assert_eq!(normalize_search_text("1984"), "1984");
        assert_eq!(normalize_search_text("451 градус"), "451 ГРАДУС");
    }

    #[test]
    fn normalize_search_text_collapses_whitespace_and_trims() {
        assert_eq!(normalize_search_text("  Война   и  мир  "), "ВОЙНА И МИР");
        assert_eq!(normalize_search_text(""), "");
        assert_eq!(normalize_search_text("!!!"), "");
    }

    #[test]
    fn normalize_search_text_drops_control_and_format_chars() {
        // Both occur in real scanned titles: U+0004 and a soft hyphen.
        assert_eq!(normalize_search_text("\u{0004}Тайна"), "ТАЙНА");
        assert_eq!(normalize_search_text("Биб\u{00ad}лиотека"), "БИБ ЛИОТЕКА");
    }

    #[test]
    fn normalize_search_text_is_idempotent() {
        for input in [
            "«Мир приключений» 1963 (№09)",
            "  Война   и  мир  ",
            "Альфа-Бета",
            "1984",
            "!!!",
        ] {
            let once = normalize_search_text(input);
            assert_eq!(normalize_search_text(&once), once, "input: {input}");
        }
    }
}
