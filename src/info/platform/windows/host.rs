use super::ffi;
use crate::utils::clean;

pub fn detect_host() -> Option<String> {
    let manufacturer = ffi::reg_sz(
        r"HARDWARE\DESCRIPTION\System\BIOS",
        "SystemManufacturer",
    );
    let product = ffi::reg_sz(
        r"HARDWARE\DESCRIPTION\System\BIOS",
        "SystemProductName",
    )
    .or_else(|| {
        ffi::reg_sz(
            r"HARDWARE\DESCRIPTION\System\BIOS",
            "BaseBoardProduct",
        )
    });

    let filter = |s: Option<String>| -> Option<String> {
        s.map(|v| clean(&v)).filter(|v| {
            let l = v.to_lowercase();
            !l.is_empty()
                && l != "to be filled by o.e.m."
                && l != "default string"
                && l != "system manufacturer"
                && l != "system product name"
        })
    };

    let manufacturer = filter(manufacturer);
    let product = filter(product);

    match (manufacturer, product) {
        (Some(m), Some(p)) if !p.to_lowercase().contains(&m.to_lowercase()) => {
            Some(format!("{m} {p}"))
        }
        (_, Some(p)) => Some(p),
        (Some(m), None) => Some(m),
        _ => None,
    }
}
