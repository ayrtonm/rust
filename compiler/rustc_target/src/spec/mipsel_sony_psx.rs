use crate::spec::{
    LinkArgs, LinkerFlavor, LldFlavor, PanicStrategy, RelocModel, Target, TargetOptions,
};

// The PSX has custom linker requirements.
const LINKER_SCRIPT: &str = include_str!("./mipsel_sony_psx_linker_script.ld");

// These options are mostly taken from the mipsel-sony-psp and msp430-none-elf
// configurations with some changes specific to MIPS I.
pub fn target() -> Target {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Lld(LldFlavor::Ld), vec!["--emit-relocs".to_string()]);

    Target {
        llvm_target: "mipsel-sony-psx".to_string(),
        target_endian: "little".to_string(),
        pointer_width: 32,
        target_c_int_width: "32".to_string(),
        data_layout: "e-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32-S64".to_string(),
        arch: "mips".to_string(),
        target_os: "none".to_string(),
        target_env: "psx".to_string(),
        target_vendor: "sony".to_string(),
        linker_flavor: LinkerFlavor::Ld,

        options: TargetOptions {
            cpu: "mips1".to_string(),
            executables: true,
            linker: Some("ld".to_owned()),
            linker_is_gnu: true,
            relocation_model: RelocModel::Static,
            eliminate_frame_pointer: false,
            dynamic_linking: false,
            function_sections: true,

            // PSX doesn't natively support floats.
            features: "+soft-float".to_string(),

            // The MIPS I in the PSX doesn't have a SYNC instruction so we have
            // to disable the Atomic* API.
            // See https://github.com/rust-lang/rust/issues/54511 for more info.
            max_atomic_width: Some(0),

            // Taken from msp430-none-elf target configuration.
            panic_strategy: PanicStrategy::Abort,
            trap_unreachable: false,

            // PSX does not support trap-on-condition instructions.
            llvm_args: vec!["-mno-check-zero-division".to_string()],
            pre_link_args,
            link_script: Some(LINKER_SCRIPT.to_string()),
            ..Default::default()
        },
    }
}
