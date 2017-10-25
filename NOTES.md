
# NOTES

See https://bitbucket.org/chromiumembedded/cef-project/src/master/examples/message_router for a stand-alone
example application that demonstrates routing asynchronous messages between JavaScript running in the
renderer process and C++ running in the browser process

`CefBrowserProcessHandler::OnRenderProcessThreadCreated()`
- Use this callback to send initialization info to renderer process

## IPC

https://bitbucket.org/chromiumembedded/cef/wiki/GeneralUsage#markdown-header-inter-process-communication-ipc

A message sent from the browser process to the render process will arrive in `CefRenderProcessHandler::OnProcessMessageReceived()`
A message sent from the render process to the browser process will arrive in `CefClient::OnProcessMessageReceived()`
