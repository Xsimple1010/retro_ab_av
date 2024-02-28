use std::path::PathBuf;

fn main() {
    let video_fns = "get_video_extra_data|core_video_refresh|set_rust_video_refresh";
    let audio_sample_fns = "get_audio_extra_data|core_audio_sample|set_rust_audio_sample";
    let audio_sample_batch_fns = "core_audio_sample_batch|set_rust_audio_sample_batch";

    cc::Build::new()
        .file("src/lib/handle_retro_cb.c")
        .compile("handle_retro_cb");

    let bindings = bindgen::Builder::default()
        .header("src/lib/handle_retro_cb.h")
        .allowlist_function(
            video_fns.to_owned()
                + "|"
                + audio_sample_fns
                + "|"
                + audio_sample_batch_fns
                + "|"
                + "de_init_all_callbacks"
                + "|"
                + "set_video_extra_data"
                + "|"
                + "set_audio_extra_data",
        )
        .clang_arg("-fparse-all-comments")
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src/binding");
    bindings
        .write_to_file(out_path.join("binding_handle_retro_cb.rs"))
        .expect("Couldn't write bindings!");
}
