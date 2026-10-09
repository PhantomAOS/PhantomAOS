package com.phantom.aos.core.bridge

object PtyBridge {
    init {
        System.loadLibrary("phantom_core")
    }

    external fun nativePing(): String
}
