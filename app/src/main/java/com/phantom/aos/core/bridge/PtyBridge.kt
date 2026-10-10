package com.phantom.aos.core.bridge

import java.io.BufferedReader
import java.io.InputStreamReader
import java.io.OutputStreamWriter

class PtyBridge(private val nativeLibDir: String) {
    private var process: Process? = null
    private var writer: OutputStreamWriter? = null
    private var reader: BufferedReader? = null

    fun start() {
        val binaryPath = "$nativeLibDir/libphantom_pty.so"
        val pb = ProcessBuilder(binaryPath)
        pb.redirectErrorStream(true)
        process = pb.start()
        writer = OutputStreamWriter(process!!.outputStream)
        reader = BufferedReader(InputStreamReader(process!!.inputStream))
    }

    fun write(input: String) {
        writer?.write(input)
        writer?.flush()
    }

    fun readChunk(): String {
        val buf = CharArray(4096)
        val stream = process?.inputStream ?: return ""
        return if (stream.available() > 0) {
            val n = reader?.read(buf) ?: 0
            if (n > 0) String(buf, 0, n) else ""
        } else {
            ""
        }
    }
}
