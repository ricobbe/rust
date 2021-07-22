#include <stdio.h>

__declspec(dllexport) void f() {
    printf("extern_2 f\n");
    fflush(stdout);
}
