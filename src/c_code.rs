pub const C_HEADER: &[u8] = b"#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>
#include <sys/stat.h>

static char *myCode = \"";

pub const C_LIB_FUNCTIONS: &[u8] = b"\";

static char encoding_table[] = {'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
'w', 'x', 'y', 'z', '0', '1', '2', '3',
'4', '5', '6', '7', '8', '9', '+', '/'};
static char *decoding_table = NULL;
static int mod_table[] = {0, 2, 1};

void build_decoding_table() {

decoding_table = malloc(256);

for (int i = 0; i < 64; i++)
decoding_table[(unsigned char) encoding_table[i]] = i;
}


unsigned char *base64_decode(const char *data,
size_t input_length,
size_t *output_length) {

if (decoding_table == NULL) build_decoding_table();

if (input_length % 4 != 0) return NULL;

*output_length = input_length / 4 * 3;
if (data[input_length - 1] == '=') (*output_length)--;
if (data[input_length - 2] == '=') (*output_length)--;

unsigned char *decoded_data = malloc(*output_length);
if (decoded_data == NULL) return NULL;

for (int i = 0, j = 0; i < input_length;) {

uint32_t sextet_a = data[i] == '=' ? 0 & i++ : decoding_table[data[i++]];
uint32_t sextet_b = data[i] == '=' ? 0 & i++ : decoding_table[data[i++]];
uint32_t sextet_c = data[i] == '=' ? 0 & i++ : decoding_table[data[i++]];
uint32_t sextet_d = data[i] == '=' ? 0 & i++ : decoding_table[data[i++]];

uint32_t triple = (sextet_a << 3 * 6)
+ (sextet_b << 2 * 6)
+ (sextet_c << 1 * 6)
+ (sextet_d << 0 * 6);

if (j < *output_length) decoded_data[j++] = (triple >> 2 * 8) & 0xFF;
if (j < *output_length) decoded_data[j++] = (triple >> 1 * 8) & 0xFF;
if (j < *output_length) decoded_data[j++] = (triple >> 0 * 8) & 0xFF;
}

return decoded_data;
}
";

pub const C_MAIN_SIMPLE: &[u8] = b"
int main (char **args) {
	size_t len = 0;
	char* binary = base64_decode(myCode, strlen(myCode), &len);
	FILE *fp = fopen(\"myBinaryCode\" ,\"w\");
	fwrite(binary, len, 1, fp);
	fclose(fp);
	chmod(\"myBinaryCode\", 511);
	execl(\"myBinaryCode\", \"\", NULL);
	return 2;
}";

pub const C_MAIN_WITH_CHECKS: &[u8] = b"

int cfileexists(const char * filename){
    /* try to open file to read */
    FILE *file;
    if (file = fopen(filename, \"r\")){
        fclose(file);
        return 1;
    }
    return 0;
}


int main (char **args) {
	size_t len = 0;
	char* binary = base64_decode(myCode, strlen(myCode), &len);
	FILE *fp = fopen(\"myBinaryCode\" ,\"w\");
	if (fp == NULL) {
		return 33;
	}
	if (fwrite(binary, len, 1, fp) <= 0) {
		return 25;
	}
	if (fclose(fp) != 0) {
		if (errno == EACCES) {
			return 24;
		}
		if (errno == EFAULT) {
			return 28;
		}
		if (errno == EIO) {
			return 29;
		}
		if (errno == EROFS) {
			return 30;
		}
		if (errno == ENOENT) {
			return 31;
		}
		if (errno == EINVAL) {
			return 32;
		}
		else return 26;
	}
	if (chmod(\"myBinaryCode\", 511) == -1) {
		if (errno == EACCES) {
			return 34;
		}
		if (errno == EFAULT) {
			return 19;
		}
		if (errno == EIO) {
			return 20;
		}
		if (errno == EROFS) {
			return 21;
		}
		if (errno == ENOENT) {
			return 22;
		}
		if (errno == EINVAL) {
			return 23;
		}
		return 27;
	}
	if (execl(\"myBinaryCode\", \"\", NULL) != -1) {
		return 11;
	}
	if (errno == EACCES) {
		return 4;
	}
	if (errno == EIO) {
		return 5;
	}
	if (errno == ELIBBAD) {
		return 6;
	}
	if (errno == EISDIR) {
		return 7;
	}
	if (errno == EMFILE) {
		return 8;
	}
	if (errno == EINVAL) {
		return 9;
	}
	if (errno == E2BIG) {
		return 10;
	}
	if (errno == EFAULT) {
		return 12;
	}
	if (errno == ELOOP) {
		return 13;
	}
	if (errno == ENOENT) {
		return 14;
	}
	if (errno == ENOEXEC) {
		return 15;
	}
	if (errno == ENOMEM) {
		return 16;
	}
	if (errno == EPERM) {
		return 17;
	}
	if (errno == ETXTBSY) {
		return 18;
	}
	if (cfileexists(\"myBinaryCode\") == 0) {
		return 3;
	}
	return 2;
}
";
