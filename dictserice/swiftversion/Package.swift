import PackageDescription

let package = Package(
    name: "swiftversion",
    dependencies: [
        .Package(url: "https://github.com/vapor/vapor.git", majorVersion: 1, minor: 0),
        //.Package(url: "https://github.com/vapor/redis-provider.git", majorVersion: 0, minor: 1),
        .Package(url: "https://github.com/vapor/socks.git", majorVersion: 1, minor: 0),
        .Package(url: "https://github.com/itang/redbird.git", majorVersion: 0, minor: 11),
    ],
    exclude: [
        "Config",
        "Database",
        "Localization",
        "Public",
        "Resources",
        "Tests",
    ]
)
