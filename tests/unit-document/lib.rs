#[cfg(test)]
mod tests {
    use rtc_declaration::{BUILD, UNIT};
    use unit::{document as d, record};

    const TOML: &str = include_str!("../../unit.toml");

    fn document() -> d::Unit<String> {
        toml_edit::de::from_str(TOML).unwrap()
    }

    fn assert_legacy_policy<S: AsRef<str>>(value: &d::Unit<S>) {
        assert_eq!(value.schema, 1);
        assert_eq!(value.id.as_ref(), UNIT.name);
        assert_eq!(value.loader.image.as_ref(), UNIT.loader.image);
        assert!(matches!(UNIT.loader.space, env::ProgramKind::User));
        assert_eq!(value.loader.space, d::Space::User);
        assert_eq!(
            value.loader.args,
            UNIT.loader
                .args
                .iter()
                .map(|arg| *arg as u64)
                .collect::<Vec<_>>()
        );
        assert_eq!(value.loader.stack, UNIT.loader.stack as u64);

        assert_eq!(
            value.operator.namespace.len(),
            UNIT.operator.namespace.len()
        );
        for (current, previous) in value.operator.namespace.iter().zip(UNIT.operator.namespace) {
            assert_eq!(current.key.as_ref(), previous.key.name());
            assert_eq!(current.path.as_ref(), previous.path);
            assert!(matches!(
                previous.placement,
                legacy_unit::operator::Placement::Shared
            ));
            assert_eq!(current.placement, d::Placement::Shared);
            assert!(matches!(
                previous.scope,
                legacy_unit::operator::Scope::Subtree
            ));
            assert!(matches!(current.scope, d::Scope::Subtree));
            assert_eq!(current.ops, previous.ops.bits());
            assert_eq!(previous.permit, Some(system_api::operator::Permit::Public));
            assert_eq!(current.permit, Some(d::Permit::Public));
            assert_eq!(current.source_mark, previous.source_mark);
            assert_eq!(current.bindings.len(), previous.bindings.len());
            for (binding, old_binding) in current.bindings.iter().zip(previous.bindings) {
                assert_eq!(binding.name.as_ref(), old_binding.name);
                assert_eq!(binding.mark, old_binding.mark.get());
                assert_eq!(binding.view, old_binding.view.bits().bits());
                assert_eq!(old_binding.permit, system_api::operator::Permit::Public);
                assert_eq!(binding.permit, d::Permit::Public);
                assert_eq!(binding.name.as_ref(), rtc_api::ENTRY_NAME);
                // RTC's original declaration and server both use Router's mark.
                assert_eq!(binding.mark, router_api::ENTRY_MARK.get());
                assert_eq!(binding.view, env::Access::STORE.bits().bits());
            }
        }
        assert_eq!(
            value
                .identity
                .aliases
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            UNIT.identity.aliases
        );
        assert_eq!(
            value
                .identity
                .grants
                .iter()
                .map(|code| system_api::identity::Grant::from_action(*code).unwrap())
                .collect::<Vec<_>>(),
            UNIT.identity.grants
        );

        assert_eq!(value.control.creation, d::Creation::Startup);
        assert_eq!(
            UNIT.control.after.unwrap(),
            [
                legacy_unit::Dependency::Bootstrap(legacy_unit::Bootstrap::Operator),
                legacy_unit::Dependency::Unit("hub"),
                legacy_unit::Dependency::Unit("router"),
            ]
        );
        // System bootstraps Operator; these are the actual packaged providers.
        for providers in [&value.control.requires, &value.control.after] {
            assert_eq!(
                providers.iter().map(AsRef::as_ref).collect::<Vec<_>>(),
                ["hub", "router"]
            );
        }
        match (value.control.ready, UNIT.control.ready) {
            (d::Ready::Signal(current), legacy_unit::Ready::Signal(previous)) => {
                assert_eq!(current, previous.get());
                assert_eq!(current, unit::READY_MARK.get());
            }
            _ => panic!("ready policy changed"),
        }
        assert!(matches!(
            UNIT.control.ending,
            Some(legacy_unit::Ending::Resident)
        ));
        assert_eq!(value.control.ending, d::Ending::Resident);
        assert_eq!(
            value.control.supervision.as_ref().map(AsRef::as_ref),
            UNIT.control.supervision
        );
        assert!(UNIT.control.construction.is_empty());
        assert!(value.control.construction.is_empty());
        assert!(UNIT.control.retirement.is_empty());
        assert!(value.control.retirement.is_empty());
        // Previously Vanllia PRODUCT.targets["supervised"] contained RTC.
        assert_eq!(
            value
                .control
                .groups
                .iter()
                .map(AsRef::as_ref)
                .collect::<Vec<_>>(),
            ["supervised"]
        );
        assert!(!value.control.guarded);
        assert!(UNIT.supply.bindings.is_empty());
        assert!(value.supply.bindings.is_empty());
        assert!(UNIT.valid());
        value.validate().unwrap();
    }

    fn with_marks(entry: &str, ready: &str) -> String {
        let original_entry = format!("\"{}\"", router_api::ENTRY_MARK.get());
        let original_ready = format!("\"{}\"", unit::READY_MARK.get());
        assert!(TOML.contains(&original_entry));
        assert!(TOML.contains(&original_ready));
        TOML.replace(&original_entry, entry)
            .replace(&original_ready, ready)
    }

    #[test]
    fn toml_preserves_legacy_policy_and_supervised_membership() {
        assert_legacy_policy(&document());
    }

    #[test]
    fn binary_roundtrip_preserves_every_field_and_borrows_strings() {
        let declaration = document();
        let bytes = record::encode(&declaration).unwrap();
        let decoded = record::decode(&bytes).unwrap();
        assert_legacy_policy(&decoded);
        let text = toml_edit::ser::to_string(&decoded).unwrap();
        let owned: d::Unit<String> = toml_edit::de::from_str(&text).unwrap();
        assert_eq!(owned, declaration);
        let begin = bytes.as_ptr() as usize;
        let end = begin + bytes.len();
        for text in [
            decoded.id,
            decoded.loader.image,
            decoded.operator.namespace[0].bindings[0].name,
            decoded.control.requires[0],
            decoded.control.after[1],
            decoded.control.groups[0],
        ] {
            let pointer = text.as_ptr() as usize;
            assert!(pointer >= begin && pointer + text.len() <= end);
        }
    }

    #[test]
    fn runtime_conversion_preserves_the_legacy_native_contract() {
        // Runtime strings borrow immutable boot bytes with a static lifetime.
        static BYTES: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
        let bytes = BYTES.get_or_init(|| record::encode(&document()).unwrap());
        let decoded = record::decode(bytes).unwrap();
        assert_legacy_policy(&decoded);
        let native = unit::runtime::UnitFile::try_from(decoded).unwrap();
        assert_eq!(native.name, UNIT.name);
        assert_eq!(native.loader.image, UNIT.loader.image);
        assert!(matches!(native.loader.space, env::ProgramKind::User));
        assert_eq!(native.loader.args, UNIT.loader.args);
        assert_eq!(native.loader.stack, UNIT.loader.stack);
        assert_eq!(
            native.operator.namespace.len(),
            UNIT.operator.namespace.len()
        );
        for (current, previous) in native
            .operator
            .namespace
            .iter()
            .zip(UNIT.operator.namespace)
        {
            assert_eq!(current.key, previous.key);
            assert!(matches!(
                current.placement,
                unit::operator::Placement::Shared
            ));
            assert!(matches!(
                previous.placement,
                legacy_unit::operator::Placement::Shared
            ));
            assert_eq!(current.path, previous.path);
            assert!(matches!(current.scope, unit::runtime::Scope::Subtree));
            assert_eq!(current.ops, previous.ops);
            assert_eq!(current.permit, previous.permit);
            assert_eq!(current.source_mark, previous.source_mark);
            assert_eq!(current.bindings.len(), previous.bindings.len());
            for (binding, old_binding) in current.bindings.iter().zip(previous.bindings) {
                assert_eq!(binding.name, old_binding.name);
                assert_eq!(binding.mark, old_binding.mark);
                assert_eq!(binding.view, old_binding.view);
                assert_eq!(binding.permit, old_binding.permit);
            }
        }
        assert_eq!(native.identity.aliases, UNIT.identity.aliases);
        assert_eq!(native.identity.grants, UNIT.identity.grants);
        assert_eq!(native.control.creation, d::Creation::Startup);
        assert_eq!(
            native.control.after.as_deref(),
            Some(
                &[
                    unit::Dependency::Unit("hub"),
                    unit::Dependency::Unit("router"),
                ][..]
            )
        );
        match (native.control.ready, UNIT.control.ready) {
            (unit::Ready::Signal(current), legacy_unit::Ready::Signal(previous)) => {
                assert_eq!(current, previous);
            }
            _ => panic!("native ready policy changed"),
        }
        assert_eq!(native.control.ending, unit::Ending::Resident);
        assert!(matches!(
            UNIT.control.ending,
            Some(legacy_unit::Ending::Resident)
        ));
        assert_eq!(native.control.supervision, UNIT.control.supervision);
        assert!(native.control.construction.is_empty());
        assert!(native.control.retirement.is_empty());
        assert_eq!(native.control.groups, ["supervised"]);
        assert!(!native.control.guarded);
        assert!(native.supply.bindings.is_empty());
        assert!(native.valid());
    }

    #[test]
    fn quoted_decimal_and_hex_preserve_the_original_native_marks() {
        assert!(router_api::ENTRY_MARK.get() > i64::MAX as u64);
        assert!(unit::READY_MARK.get() > i64::MAX as u64);
        let hexadecimal = with_marks(
            &format!("\"0x{:016x}\"", router_api::ENTRY_MARK.get()),
            &format!("\"0x{:016x}\"", unit::READY_MARK.get()),
        );
        let decoded: d::Unit<String> = toml_edit::de::from_str(&hexadecimal).unwrap();
        assert_eq!(decoded, document());
        assert_legacy_policy(&record::decode(&record::encode(&decoded).unwrap()).unwrap());
    }

    #[test]
    fn integer_and_quoted_u64_forms_survive_binary_record_roundtrip() {
        for (literal, expected) in [
            ("17", 17),
            ("\"17\"", 17),
            ("\"0x11\"", 17),
            ("\"18446744073709551615\"", u64::MAX),
            ("\"0xffffffffffffffff\"", u64::MAX),
        ] {
            let value: d::Unit<String> =
                toml_edit::de::from_str(&with_marks(literal, literal)).unwrap();
            value.validate().unwrap();
            assert_eq!(value.operator.namespace[0].bindings[0].mark, expected);
            assert_eq!(value.control.ready, d::Ready::Signal(expected));
            let bytes = record::encode(&value).unwrap();
            let decoded = record::decode(&bytes).unwrap();
            assert_eq!(decoded.operator.namespace[0].bindings[0].mark, expected);
            assert_eq!(decoded.control.ready, d::Ready::Signal(expected));
            let text = toml_edit::ser::to_string(&decoded).unwrap();
            let owned: d::Unit<String> = toml_edit::de::from_str(&text).unwrap();
            assert_eq!(owned, value);
        }
    }

    #[test]
    fn negative_and_overflowing_marks_are_rejected_before_record_encoding() {
        for literal in [
            "-1",
            "\"-1\"",
            "\"18446744073709551616\"",
            "\"0x10000000000000000\"",
        ] {
            let valid_entry = format!("\"{}\"", router_api::ENTRY_MARK.get());
            let valid_ready = format!("\"{}\"", unit::READY_MARK.get());
            for source in [
                with_marks(literal, &valid_ready),
                with_marks(&valid_entry, literal),
            ] {
                assert!(toml_edit::de::from_str::<d::Unit<String>>(&source).is_err());
            }
        }
    }

    #[test]
    fn image_names_the_existing_cargo_binary_and_preserves_the_build_contract() {
        assert_eq!(document().loader.image, BUILD.image);
        assert_eq!(BUILD.manifest, "server/Cargo.toml");
        let manifest: toml_edit::DocumentMut =
            include_str!("../../server/Cargo.toml").parse().unwrap();
        assert_eq!(manifest["package"]["name"].as_str(), Some(BUILD.package));
        let bins = manifest["bin"].as_array_of_tables().unwrap();
        assert_eq!(bins.len(), 1);
        let binary = bins.get(0).unwrap();
        assert_eq!(binary["name"].as_str(), Some(BUILD.bin));
        assert_eq!(BUILD.bin, BUILD.image);
        let entry = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../server")
            .join(binary["path"].as_str().unwrap());
        assert!(entry.is_file());
        assert!(BUILD.features.is_empty());
        assert!(BUILD.default_features);
        assert_eq!(BUILD.target, "riscv64gc-unknown-none-elf");
        assert_eq!(BUILD.profile, "release");
    }
}
