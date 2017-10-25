
#include "include/cef_app.h"

// Entry point function for sub-processes.
extern "C" int cef_helper_run(int argc, char **argv) {
    // Provide CEF with command-line arguments.
    CefMainArgs args(argc, argv);

    // Execute the sub-process.
    return CefExecuteProcess(args, NULL, NULL);
}
