import Vapor
import HTTP
import Foundation

final class LogController /*: ResourceRepresentable */ {
    func index(request: Request) throws -> ResponseRepresentable {
        return try JSON(node: Log.all().map {
            it in try it.makeNode()
        })
    }

    func create(request: Request) throws -> ResponseRepresentable {
        let log = try request.log()
        log.id = String(NSDate().timeIntervalSince1970)

        try log.save()

        return try JSON(node: log.makeNode())
    }
}


extension Request {
    func log() throws -> Log {
        guard let json = json else {
            throw Abort.badRequest
        }
        return Log(from: try json.extract("from"), to: try json.extract("to"))
    }
}
