//! Temporary probe (not committed): stage a project's certificates.
use repo_conformance::preservation::{certificates, staged};
use repo_conformance::support::P;

#[test]
#[ignore]
fn probe_v() {
    let out = std::path::PathBuf::from(std::env::var("PROBE_OUT").expect("PROBE_OUT"));
    let name = std::env::var("PROBE_EXAMPLE").expect("PROBE_EXAMPLE");
    let project = P::copy_example(&name);
    let certified = certificates(&project);
    let ws = staged(&project, &certified);
    let mut order = String::new();
    for file in ws.files.iter() {
        let path = out.join("src").join(&file.path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &file.text).unwrap();
        order.push_str(&format!("{}\n", file.module));
    }
    std::fs::write(out.join("order.txt"), order).unwrap();
}
