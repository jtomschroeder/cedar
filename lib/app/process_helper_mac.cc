
#include "include/cef_app.h"

// Entry point function for sub-processes.
int main(int argc, char *argv[]) {
    // Provide CEF with command-line arguments.
    CefMainArgs main_args(argc, argv);

    // Execute the sub-process.
    return CefExecuteProcess(main_args, NULL, NULL);
}
