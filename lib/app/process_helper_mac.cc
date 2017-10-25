
#include "include/cef_app.h"

// Entry point function for sub-processes.
extern "C" int cef_helper_run(int argc, char **argv) {
    // Provide CEF with command-line arguments.
    CefMainArgs main_args(argc, argv);

    // Execute the sub-process.
    return CefExecuteProcess(main_args, NULL, NULL);
}

// // Entry point function for sub-processes.
// extern "C" void cef_helper_run(int argc, char *argv[]) {
//     CefMainArgs main_args(0, nullptr);

//     CefExecuteProcess(main_args, NULL, NULL);
// }
