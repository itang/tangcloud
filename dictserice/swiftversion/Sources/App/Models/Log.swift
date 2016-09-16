import Vapor
import Fluent
import Foundation
import Redbird

final class Log /*: Model */ {
    static let DICT_LOG_KEY = "tc:dict:log"
    static let DICT_LOG_DATA_KEY = "tc:dict:log:data"

    var id: String?
    var from: String
    var fromLang: String?
    var to: String
    var toLang: String?

    init(id: String, from: String, to: String) {
        self.id = id
        self.from = from
        self.to = to
    }

    convenience init(from: String, to: String) {
        self.init(id: UUID().uuidString, from: from, to: to)
    }

    func makeNode() throws -> Node {
        return try Node(node: [
                "id": id,
                "from": from,
                "to": to
        ])
    }

    func save() throws {
        let client = Log.redis!

        let json = try makeNode().converted(to: JSON.self).makeBytes().string
        let key = id!
        let score = String(NSDate().timeIntervalSince1970)

        let _ = try client.command("ZADD", params: [Log.DICT_LOG_KEY, key, score])
        let _ = try client.command("HSET", params: [Log.DICT_LOG_DATA_KEY, key, json])
    }

    class func all() throws -> [Log] {
        let client = Log.redis!

        let resp = try client.command("HVALS", params: [DICT_LOG_DATA_KEY])
        return try resp.toArray().map {
            it in

            let str = it as! RespBulkString
            let json = try JSON(bytes: str.content.bytes)

            return try Log(id: json.extract("id"), from: json.extract("from"), to: json.extract("to"))
        }
    }

    static let redis: Redbird? = {
        let config = RedbirdConfig(address: "127.0.0.1", port: 6379 /*, password: ""*/)
        do {
            return try Redbird(config: config)
        } catch {
            print(error)
            return nil
        }
    }()
}
