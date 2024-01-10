package sh.illumi.oxide.plugins

import io.ktor.network.selector.*
import io.ktor.network.sockets.*
import io.ktor.network.tls.*
import io.ktor.server.application.*
import io.ktor.utils.io.*
import io.ktor.utils.io.core.*
import java.io.InputStream
import java.util.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking

fun Application.configureSockets() {
}
