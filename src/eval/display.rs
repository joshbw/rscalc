/// Result formatting: convert Rational to display strings.

use calc_manager::prelude::*;
use calc_manager::ratpack::conv::rat_to_string;

/// Format a Rational value for display.
pub fn format_result(
    val: &Rational,
    radix_type: RadixType,
    precision: i32,
    format: NumberFormat,
) -> String {
    let radix = radix_type.to_radix();
    match rat_to_string(val, format, radix, precision) {
        Ok(s) => clean_display_string(&s, radix_type),
        Err(e) => format!("Error: {e}"),
    }
}

/// Format a Rational for programmer mode, showing multiple bases.
pub fn format_programmer_result(
    val: &Rational,
    primary_radix: RadixType,
    precision: i32,
) -> String {
    let primary = format_with_prefix(val, primary_radix, precision);

    let mut parts = vec![primary];

    let bases = [RadixType::Hex, RadixType::Decimal, RadixType::Octal, RadixType::Binary];
    for base in &bases {
        if *base != primary_radix {
            let formatted = format_with_prefix(val, *base, precision);
            parts.push(formatted);
        }
    }

    format!("{} ({})", parts[0], parts[1..].join(", "))
}

fn format_with_prefix(val: &Rational, radix_type: RadixType, precision: i32) -> String {
    let s = rat_to_string(val, NumberFormat::Float, radix_type.to_radix(), precision)
        .unwrap_or_else(|_| "0".to_string());
    let cleaned = clean_display_string(&s, radix_type);
    match radix_type {
        RadixType::Hex => format!("0x{}", cleaned.to_uppercase()),
        RadixType::Octal => format!("0o{cleaned}"),
        RadixType::Binary => format!("0b{cleaned}"),
        RadixType::Decimal => cleaned,
    }
}

/// Clean up a display string from ratpack (remove trailing zeros, etc.)
fn clean_display_string(s: &str, radix_type: RadixType) -> String {
    let s = s.trim();

    if radix_type != RadixType::Decimal {
        return s.to_string();
    }

    if !s.contains('.') {
        return s.to_string();
    }

    if s.contains('e') || s.contains('E') {
        return s.to_string();
    }

    let trimmed = s.trim_end_matches('0');
    let trimmed = trimmed.trim_end_matches('.');
    trimmed.to_string()
}
