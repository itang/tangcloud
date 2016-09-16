import Vapor

let drop = Droplet()

drop.get {
    req in
    let lang = req.headers["Accept-Language"]?.string ?? "en"
    return try drop.view.make("welcome", [
            "message": Node.string(drop.localization[lang, "welcome", "title"])
    ])
}

drop.resource("posts", PostController())

drop.get("/ping") {
    _ in
    return "pong"
}

let log = LogController()
drop.get("dict/logs") {
    req in
    return try log.index(request: req)
}

drop.post("dict/logs") {
    req in
    return try log.create(request: req)
}

drop.run()
