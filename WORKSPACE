# Rust

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
http_archive(
    name = "rules_rust",
    sha256 = "db89135f4d1eaa047b9f5518ba4037284b43fc87386d08c1d1fe91708e3730ae",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.27.0/rules_rust-v0.27.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
)

# Genrules

http_archive(
    name="genrules",
    urls=[
        "https://github.com/genrules/genrules/archive/789b95e2557116d8977fc5850b320b8ad542f3e6.zip",
    ],
    strip_prefix="genrules-789b95e2557116d8977fc5850b320b8ad542f3e6",
)

load("@genrules//gcloud:index.bzl", "gcloud_download")

gcloud_download()

load("@genrules//crane:index.bzl", "crane_download")

crane_download()
