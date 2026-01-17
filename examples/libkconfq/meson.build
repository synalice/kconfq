project(
    'example',
    'c',
    default_options: [
        'buildtype=debugoptimized',
    ],
)

kconfq = dependency('kconfq')

executable(
    meson.project_name(),
    sources: files('main.c'),
    dependencies: kconfq,
    install: true,
)
