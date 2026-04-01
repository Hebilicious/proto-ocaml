use ocaml_plugin::{build_activate_environment_output, parse_version_file};
use proto_pdk::{AnyResult, HostArch, HostEnvironment, HostLibc, HostOS, UnresolvedVersionSpec, VirtualPath};
use std::path::PathBuf;

fn host_env() -> HostEnvironment {
    HostEnvironment {
        arch: HostArch::Arm64,
        ci: false,
        libc: HostLibc::Gnu,
        os: HostOS::Linux,
        home_dir: VirtualPath::Real(PathBuf::from("/home/tester")),
    }
}

fn tool_dir() -> VirtualPath {
    VirtualPath::Virtual {
        path: PathBuf::from("/proto/tools/ocaml/5.4.1"),
        virtual_prefix: PathBuf::from("/proto"),
        real_prefix: PathBuf::from("/root/.proto"),
    }
}

#[test]
fn public_parse_version_file_normalizes_supported_ocaml_inputs() -> AnyResult<()> {
    assert_eq!(
        parse_version_file(".ocaml-version", "ocaml-base-compiler.4.08.1")?.version,
        Some(UnresolvedVersionSpec::parse("4.8.1")?),
    );

    assert_eq!(
        parse_version_file(
            "dune-project",
            r#"
            (lang dune 3.18)
            (package
             (name sample)
             (depends (ocaml (>= 5.04.0) (< 5.5))))
            "#,
        )?
        .version,
        Some(UnresolvedVersionSpec::parse(">=5.4.0,<5.5")?),
    );

    Ok(())
}

#[test]
fn public_activate_environment_output_keeps_tool_paths_from_opam_env() {
    let output = build_activate_environment_output(
        r#"(("OPAMROOT" "/root/.proto/tools/ocaml/5.4.1/.opam-root")
            ("OPAMSWITCH" "/root/.proto/tools/ocaml/5.4.1")
            ("OPAM_SWITCH_PREFIX" "/root/.proto/tools/ocaml/5.4.1/_opam")
            ("PATH" "/root/.proto/tools/ocaml/5.4.1/bin:/root/.proto/tools/ocaml/5.4.1/_opam/bin:/root/.cargo/bin:/usr/bin"))"#,
        &tool_dir(),
        &host_env(),
    );

    assert_eq!(
        output.paths,
        vec![
            PathBuf::from("/root/.proto/tools/ocaml/5.4.1/bin"),
            PathBuf::from("/root/.proto/tools/ocaml/5.4.1/_opam/bin"),
        ],
    );
    assert_eq!(
        output.env.get("OPAMROOT"),
        Some(&"/root/.proto/tools/ocaml/5.4.1/.opam-root".into()),
    );
    assert_eq!(
        output.env.get("OPAM_SWITCH_PREFIX"),
        Some(&"/root/.proto/tools/ocaml/5.4.1/_opam".into()),
    );
}
