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
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    TerminalScreen()
                }
            }
        }
    }
}

@Composable
fun TerminalScreen() {
    var output by remember { mutableStateOf("") }
    var input by remember { mutableStateOf("") }
    var fd by remember { mutableStateOf(-1) }
    val scrollState = rememberScrollState()

    LaunchedEffect(Unit) {
        fd = PtyBridge.nativeStartShell()
        while (true) {
            val chunk = PtyBridge.nativeRead(fd)
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
                    PtyBridge.nativeWrite(fd, input + "\n")
                    input = ""
                })
            )
            Button(onClick = {
                PtyBridge.nativeWrite(fd, input + "\n")
                input = ""
            }) {
                Text("Send")
            }
        }
    }
}
