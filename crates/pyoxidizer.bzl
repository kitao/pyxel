def make_exe():
    dist = default_python_distribution()

    policy = dist.make_python_packaging_policy()
    policy.resources_location_fallback = "filesystem-relative:pyxel_lib"

    python_config = dist.make_python_interpreter_config()
    python_config.module_search_paths = ["$ORIGIN/pyxel_lib"]
    python_config.run_module = "pyxel"

    exe = dist.to_python_executable(
        name="pyxel",
        packaging_policy=policy,
        config=python_config,
    )

    for resource in exe.pip_install([VARS.get("wheel_name")]):
        resource.add_location = "filesystem-relative:pyxel_lib"
        exe.add_python_resource(resource)

    return exe


def make_embedded_resources(exe):
    return exe.to_embedded_resources()


def make_install(exe):
    files = FileManifest()
    files.add_python_resource(".", exe)

    return files


register_target("exe", make_exe)
register_target(
    "resources", make_embedded_resources, depends=["exe"], default_build_script=True
)
register_target("install", make_install, depends=["exe"], default=True)

resolve_targets()
