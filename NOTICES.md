# Dependency and asset notices

The project owner has not selected a software or brand-asset license. This
change does not choose one or grant third-party reuse rights to BreakFree.
Application sources are original; fixtures are synthetic. No Microsoft code,
logo, Office font or reference-product artwork is copied into the application.

Rust runtime/build dependencies are declared in Cargo.toml and pinned by
Cargo.lock. The [locked dependency inventory](third_party/NOTICES.md) preserves
the declared license metadata and notice/license files shipped by those crates,
including egui's default-font notices. Missing texts retrieved from upstream
are pinned to each crate's original source Git SHA with provenance recorded.
Remaining omissions are identified for review before redistribution. The inventory
covers all resolved platforms/features, not a claim that every listed crate is
linked into this target. It is not a completed binary-distribution license audit.

Direct dependencies: egui/eframe, rfd, image, num-bigint, num-rational, num-traits,
serde/serde_json and tempfile. Their declared license terms remain their own;
they do not select a license for this project. Before any distributed installer,
review the actual target dependency set and include its complete required notices.
No release is made by this task.

The approved project-owned Sheets icon is documented in
[brand provenance](docs/BRAND_PROVENANCE.md). No external fonts are downloaded at
runtime. The earlier Python/Tk prototype and its notices are preserved only as
historical reference under legacy/python; Python/Tcl/Tk are not shipped with or
required by the Rust product.
