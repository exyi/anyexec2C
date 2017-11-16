#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>

#define BLOCK_SIZE 1024*1024

int child(int pipe_write) {
    for (int aloc = 0; aloc < 254; aloc++) {
        size_t size = BLOCK_SIZE;

        // prevents optimalization by compiler
        unsigned char volatile* data = malloc(size);
        if (data == NULL) {
            return 1;
        }

        // force it to really work with the data - prevents COW
        for (int i = 0; i < size; i += 4000) {
            data[i] += 17 + i;
        }
        
        // child still alive - make parent know
        // try until a char is written or run out of time
        while(write(pipe_write, ".", 1) != 1);
    }
    return 0;
}

int parent(int pipe_read) {
    char c;
    int nblocks = 0;
    for(;;)
        switch(read(pipe_read, &c, 1)) {
            case 0: // eof
                return nblocks;
            case 1: // character read
                nblocks++;
            case -1: // error
                ; // try until a char is read or run out of time
        }
}

int main() {
    int pipefd[2];
    if(pipe(pipefd) == -1)
        return 255;
    pid_t pid = fork();
    if(pid == 0) {
        close(pipefd[0]);
        return child(pipefd[1]); // 1 = write end
    }
    if(pid > 0) {
        close(pipefd[1]);
        return parent(pipefd[0]); // 0 = read end
    }
    // fork error:
    return 255;
}
