package com.phantom.aos

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.phantom.aos.core.bridge.PtyBridge
import kotlinx.coroutines.delay

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val bridge = PtyBridge(applicationInfo.nativeLibraryDir)
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    TerminalScreen(bridge)
                }
            }
        }
    }
}

@Composable
fun TerminalScreen(bridge: PtyBridge) {
    var output by remember { mutableStateOf("") }
    var input by remember { mutableStateOf("") }
    val scrollState = rememberScrollState()

    LaunchedEffect(Unit) {
        try {
            bridge.start()
        } catch (e: Exception) {
            output = "Failed to start shell: ${e.message}"
            return@LaunchedEffect
        }
        while (true) {
            val chunk = try { bridge.readChunk() } catch (e: Exception) { "" }
            if (chunk.isNotEmpty()) output += chunk
            delay(100)
        }
    }

    LaunchedEffect(output) {
        scrollState.animateScrollTo(scrollState.maxValue)
    }

    Column(modifier = Modifier.fillMaxSize().padding(8.dp)) {
        Text(
            text = output,
            modifier = Modifier.weight(1f).verticalScroll(scrollState),
            fontFamily = FontFamily.Monospace,
            fontSize = 12.sp
        )
        Row(verticalAlignment = Alignment.CenterVertically) {
            TextField(
                value = input,
                onValueChange = { input = it },
                modifier = Modifier.weight(1f),
                keyboardOptions = androidx.compose.foundation.text.KeyboardOptions(imeAction = ImeAction.Send),
                keyboardActions = KeyboardActions(onSend = {
                    bridge.write(input + "\n")
                    input = ""
                })
            )
            Button(onClick = {
                bridge.write(input + "\n")
                input = ""
            }) {
                Text("Send")
            }
        }
    }
}
