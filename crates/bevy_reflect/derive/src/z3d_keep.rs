//! Z3D: which of Bevy's types register themselves in games built with
//! `--cfg z3d_reflect_trim` (z3d's exports). See `impls::common::reflect_auto_registration`.
//!
//! In such a build a Bevy type registers itself only when it's in [`ALWAYS`] or the build
//! keeps it with `--cfg z3d_keep="<Type>"`: z3d passes the types the game's scene, animation
//! and effect files name, and the components glTF models carry when the game has models.
//! Registering a type registers the types of its fields too.

/// Whether a type's automatic registration depends on the build keeping it: Bevy's own types,
/// except those always kept.
pub(crate) fn trimmable(crate_name: &str, ident: &str) -> bool {
    BEVY_CRATES.binary_search(&crate_name).is_ok() && ALWAYS.binary_search(&ident).is_err()
}

/// Whether glTF models need the type (kept with `--cfg z3d_models`).
pub(crate) fn for_models(ident: &str) -> bool {
    MODELS.binary_search(&ident).is_ok()
}

/// Bevy's own crates (crate names), sorted. Keep in step with `crates/` when upgrading Bevy.
const BEVY_CRATES: &[&str] = &[
    "bevy_a11y", "bevy_android", "bevy_animation", "bevy_anti_alias",
    "bevy_app", "bevy_asset", "bevy_audio", "bevy_camera",
    "bevy_camera_controller", "bevy_clipboard", "bevy_color", "bevy_core_pipeline",
    "bevy_curve", "bevy_derive", "bevy_dev_tools", "bevy_diagnostic",
    "bevy_dylib", "bevy_ecs", "bevy_encase_derive", "bevy_extract",
    "bevy_feathers", "bevy_gilrs", "bevy_gizmos", "bevy_gizmos_render",
    "bevy_gltf", "bevy_image", "bevy_input", "bevy_input_focus",
    "bevy_internal", "bevy_light", "bevy_log", "bevy_macro_utils",
    "bevy_material", "bevy_math", "bevy_mesh", "bevy_pbr",
    "bevy_picking", "bevy_platform", "bevy_post_process", "bevy_ptr",
    "bevy_reflect", "bevy_remote", "bevy_render", "bevy_scene",
    "bevy_settings", "bevy_shader", "bevy_shape", "bevy_solari",
    "bevy_sprite", "bevy_sprite_render", "bevy_state", "bevy_tasks",
    "bevy_text", "bevy_time", "bevy_transform", "bevy_ui",
    "bevy_ui_render", "bevy_ui_widgets", "bevy_utils", "bevy_window",
    "bevy_winit", "bevy_world_serialization",
];

/// Bevy's types every game keeps (type names), sorted: what every entity of a game has.
pub(crate) const ALWAYS: &[&str] = &[
    "ChildOf", "Children", "GlobalTransform", "InheritedVisibility",
    "Name", "Transform", "TransformTreeChanged", "ViewVisibility",
    "Visibility",
];

/// The components glTF models' entities can have, sorted: Bevy builds a model as a world of its
/// own and copies each component into the game by its registration, so one unregistered
/// component stops the whole model from appearing. Found by loading every model in Bevy's
/// `assets/models` (plus spot lights' and cameras', which none of them has); check again when
/// upgrading Bevy. (Two are resources, kept as entities in every world.)
const MODELS: &[&str] = &[
    "Aabb", "AnimatedBy", "AnimationPlayer", "AnimationTargetId",
    "Camera", "Camera3d", "CameraMainTextureUsages", "CameraRenderGraph",
    "CascadeShadowConfig", "Cascades", "CascadesFrusta", "CascadesVisibleEntities",
    "ColorGrading", "CubemapFrusta", "CubemapVisibleEntities", "DebandDither",
    "DefaultQueryFilters", "DirectionalLight", "DynamicSkinnedMeshBounds", "Exposure",
    "Frustum", "GltfExtras", "GltfMaterialExtras", "GltfMaterialName",
    "GltfMeshExtras", "GltfMeshName", "GltfSceneExtras", "GltfSceneName",
    "IsResource", "Mesh3d", "MeshMorphWeights", "MorphWeights",
    "Msaa", "PointLight", "Projection", "RenderTarget",
    "SkinnedMesh", "SpotLight", "Tonemapping", "VisibilityClass",
    "VisibleEntities", "VisibleMeshEntities",
];
