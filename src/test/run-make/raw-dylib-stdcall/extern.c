#include <stdio.h>
#include <stdint.h>

struct S {
    uint8_t x;
    int32_t y;
};

__declspec(dllexport) void __stdcall stdcall_fn_1(int i) {
    printf("stdcall_fn_1(%d)\n", i);
    fflush(stdout);
}

__declspec(dllexport) void __stdcall stdcall_fn_2(uint8_t i, float f) {
    printf("stdcall_fn_2(%d, %f)\n", i, f);
    fflush(stdout);
}

__declspec(dllexport) void __stdcall stdcall_fn_3(double d) {
    printf("stdcall_fn_3(%f)\n", d);
    fflush(stdout);
}

__declspec(dllexport) void __stdcall stdcall_fn_4(uint8_t i, uint8_t j, float f) {
    printf("stdcall_fn_4(%d, %d, %f)\n", i, j, f);
    fflush(stdout);
}

__declspec(dllexport) void __stdcall stdcall_fn_5(struct S s, int i) {
    printf("stdcall_fn_5(S { x: %d, y: %d }, %d)\n", s.x, s.y, i);
    fflush(stdout);
}

__declspec(dllexport) void __stdcall stdcall_fn_6(struct S* s) {
    if (s) {
        printf("stdcall_fn_6(S { x: %d, y: %d })\n", s->x, s->y);
    } else {
        printf("stdcall_fn_6(null)\n");
    }
    fflush(stdout);
}
