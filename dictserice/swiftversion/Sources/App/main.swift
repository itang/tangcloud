import Vapor
import HTTP

let drop = Droplet()

drop.get {
    req in
    //let lang = req.headers["Accept-Language"]?.string ?? "en"
    //return try drop.view.make("welcome", [
    //        "message": Node.string(drop.localization[lang, "welcome", "title"])
    //])
    Response(redirect: "/index.html")
}

drop.resource("posts", PostController())

drop.get("/ping") {
    _ in
    //return try JSON(node: ["message": "pong"])
    return try JSON(node: [
            "message": Node.string("pong"),
            "version": 0.1
    ])
}

drop.group("api") { api in
    let log = LogController()
    api.get("dict/logs") {
        req in
        return try log.index(request: req)
    }

    api.post("dict/logs") {
        req in
        return try log.create(request: req)
    }
}

drop.run()
