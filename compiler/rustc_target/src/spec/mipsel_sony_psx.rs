use crate::spec::{LinkArgs, LinkerFlavor, LldFlavor, RelocModel};
use crate::spec::{Target, TargetOptions};

// The PSX has custom linker requirements.
const LINKER_SCRIPT: &str = include_str!("./mipsel_sony_psx_linker_script.ld");

pub fn target() -> Target {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Lld(LldFlavor::Ld), vec!["--emit-relocs".to_string()]);

    Target {
        llvm_target: "mipsel-sony-psx".to_string(),
        target_endian: "little".to_string(),
        pointer_width: 32,
        target_c_int_width: "32".to_string(),
        data_layout: "e-p:32:32:32-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v64:64:64-v128:64:128-a:0:64-n32".to_string(),
        arch: "mips".to_string(),
        target_os: "psx".to_string(),
        target_env: "".to_string(),
        target_vendor: "sony".to_string(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),

        options: TargetOptions {
            cpu: "mips1".to_string(),
            executables: true,
            linker: Some("rust-lld".to_owned()),
            linker_is_gnu: true,
            relocation_model: RelocModel::Static,
            eliminate_frame_pointer: false,
            dynamic_linking: false,
            function_sections: true,

            // PSX doesn't natively support floats.
            features: "+soft-float".to_string(),

            // PSP does not support trap-on-condition instructions.
            //llvm_args: vec!["-mno-check-zero-division".to_string()],
            pre_link_args,
            link_script: Some(LINKER_SCRIPT.to_string()),
            ..Default::default()
        },
    }
}
    //"no-compiler-rt": true,
    //"morestack": false,
