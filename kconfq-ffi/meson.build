# SPDX-FileCopyrightText: © 2026 Nikita Krasnov <nikita.nikita.krasnov@gmail.com>
#
# SPDX-License-Identifier: MIT

subdir('src')

lib = library(
    meson.project_name(),
    sources,
    rust_abi: 'c',
    install: true,
    version: meson.project_version(),
)

custom_target(
    'lib' + meson.project_name() + '_header',
    build_by_default: true,
    input: [sources, 'cbindgen.toml'],
    output: [meson.project_name() + '.h'],
    install: true,
    depends: [lib],
    install_dir: get_option('prefix') / get_option('includedir'),
    command: [
        find_program('cbindgen'),
        '@CURRENT_SOURCE_DIR@',
        '-o',
        '@OUTPUT@',
    ],
)

pkg_config = import('pkgconfig')
pkg_config.generate(
    lib,
    description: 'A portable way to query kernel configuration on a live system',
    license: meson.project_license().get(0),
)
