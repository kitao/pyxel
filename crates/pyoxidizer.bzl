def make_exe():
    dist = default_python_distribution()  # type: ignore  # noqa F821

    policy = dist.make_python_packaging_policy()
    policy.resources_location_fallback = "filesystem-relative:lib"

    python_config = dist.make_python_interpreter_config()
    python_config.module_search_paths = ["$ORIGIN/lib"]
    python_config.run_module = "pyxel"

    exe = dist.to_python_executable(
        name="pyxel",
        packaging_policy=policy,
        config=python_config,
    )

    for resource in exe.pip_install([".."]):
        resource.add_location = "filesystem-relative:lib"
        exe.add_python_resource(resource)

    return exe


def make_embedded_resources(exe):
    return exe.to_embedded_resources()


def make_install(exe):
    files = FileManifest()  # type: ignore  # noqa F821
    files.add_python_resource(".", exe)

    return files


def make_msi(exe):
    return exe.to_wix_msi_builder(
        id_prefix="pyxel",
        product_name="Pyxel",
        product_version="1.5.0",
        product_manufacturer="Takashi Kitao",
    )


register_target("exe", make_exe)  # type: ignore  # noqa F821
register_target(  # type: ignore  # noqa F821
    "resources", make_embedded_resources, depends=["exe"], default_build_script=True
)
register_target(  # type: ignore  # noqa F821
    "install", make_install, depends=["exe"], default=True
)
register_target("msi_installer", make_msi, depends=["exe"])  # type: ignore  # noqa F821

resolve_targets()  # type: ignore  # noqa F821
