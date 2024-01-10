package sh.illumi.oxide

import io.ktor.server.application.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*
import sh.illumi.oxide.plugins.*

fun main() {
    embeddedServer(Netty, port = 8080, host = "0.0.0.0", module = Application::module)
        .start(wait = true)
}

fun Application.module() {
    configureSerialization()
    configureSockets()
    configureDatabases()
    configureSecurity()
    configureHTTP()
    configureMonitoring()
    configureRouting()
}
