#include <stdio.h>

__declspec(dllexport) void f() {
    printf("extern_1 f\n");
    fflush(stdout);
}
