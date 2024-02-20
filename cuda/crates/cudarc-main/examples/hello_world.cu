#include <stdint.h>
#include <cstdio>
extern "C" __global__ void hello_world(int *i) {
    printf("Hello from the cuda kernel in thread %d\n", *i);
}
