load("@ecsact_runtime//:codegen_plugin.bzl", "EcsactCodegenPluginInfo")

def _rust_ecsact_codegen_plugin_impl(ctx):
    plugin = None
    files = ctx.attr.rust_shared_library[DefaultInfo].files

    for file in files.to_list():
        if file.extension == "so":
            plugin = file
        if file.extension == "dll":
            plugin = file

    return [
        EcsactCodegenPluginInfo(
            output_extension = ctx.attr.output_extension,
            plugin = plugin,
            data = [plugin],
        ),
    ]

rust_ecsact_codegen_plugin = rule(
    implementation = _rust_ecsact_codegen_plugin_impl,
    attrs = {
        "rust_shared_library": attr.label(mandatory = True),
        "output_extension": attr.string(mandatory = True),
    },
)
