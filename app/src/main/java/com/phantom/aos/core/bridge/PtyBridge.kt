package com.phantom.aos.core.bridge

object PtyBridge {
    init {
        System.loadLibrary("phantom_core")
    }

    external fun nativePing(): String
    external fun nativeStartShell(): Int
    external fun nativeWrite(fd: Int, input: String)
    external fun nativeRead(fd: Int): String
}
