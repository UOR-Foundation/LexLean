//! scratch
use repo_conformance::support::P;
use repo_conformance::{preservation, stress};

fn row(label: &str, project: &P) {
    for entry in preservation::certificates(project) {
        let m = lexlean::production::lower::measure_program(&entry.program);
        let slots = lexlean::production::lower::record_slots(&entry.program);
        let a = entry.certificate.text.len();
        let b = entry
            .renderings
            .iter()
            .map(|(_, c)| c.text.len())
            .max()
            .unwrap_or(0);
        let e = entry
            .composed
            .iter()
            .map(|(_, c)| c.text.len())
            .max()
            .unwrap_or(0);
        println!(
            "DATA,{label},{},{},{},{},{},{},{},{},{slots},{a},{b},{e}",
            entry.root,
            m.printed_types,
            m.exprs,
            m.shapes,
            m.string_bytes,
            m.hex_digits,
            m.number_digits,
            m.arm_pairs
        );
    }
}

#[test]
fn sizes() {
    std::thread::Builder::new()
        .stack_size(1 << 30)
        .spawn(sizes_inner)
        .unwrap()
        .join()
        .unwrap();
}

fn sizes_inner() {
    for name in ["production", "production-coverage", "models"] {
        row(name, &P::copy_example(name));
    }
    for (family, project) in stress::families() {
        row(&family.replace(',', ";"), &project);
    }
    for (family, project) in stress::fitting() {
        row(&family.replace(',', ";"), &project);
    }
    for (index, shape) in stress::mixed_shapes().into_iter().enumerate() {
        for numerator in [32, 64, 128, 192, 256] {
            let scaled = shape.scaled(numerator);
            row(
                &format!("mixed{index}@{numerator}"),
                &stress::mixed_lifted(scaled),
            );
        }
    }
    for (index, shape) in stress::random_shapes(24, 0x5eed).into_iter().enumerate() {
        row(&format!("random{index}"), &stress::mixed_lifted(shape));
    }
}
