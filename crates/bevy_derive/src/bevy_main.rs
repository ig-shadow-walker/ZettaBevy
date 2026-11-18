use bevy_macro_utils::BevyManifest;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn bevy_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    assert_eq!(
        input.sig.ident, "main",
        "`bevy_main` can only be used on a function called 'main'."
    );

    // Zetta: resolve `bevy_android` through the crate's own dependencies (`bevy`,
    // `bevy_android` or `z3d_engine`) instead of assuming a `bevy` dependency.
    let android = BevyManifest::shared(|manifest| manifest.get_path("bevy_android"));

    TokenStream::from(quote! {
        // SAFETY: `#[bevy_main]` should only be placed on a single `main` function
        // TODO: Potentially make `bevy_main` and unsafe attribute as there is a safety
        // guarantee required from the caller.
        #[unsafe(no_mangle)]
        #[cfg(target_os = "android")]
        fn android_main(android_app: #android::android_activity::AndroidApp) {
            let _ = #android::ANDROID_APP.set(android_app);
            main();
        }

        #[allow(unused)]
        #input
    })
}
