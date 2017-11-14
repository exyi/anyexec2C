#include<stdlib.h>

#define BLOCK_SIZE 1024*1024

int main() {
    for (int aloc = 0; aloc < 255; aloc++) {
        size_t size = BLOCK_SIZE;

        // prevents optimalization by compiler
        unsigned char volatile* data = malloc(size);
        if (data == NULL) {
            return aloc;
        }

        // force it to really work with the data - prevents COW
        for (int i = 0; i < size; i+=4000) {
            data[i] += 17 + i;
        }
    }
    return 255;
}