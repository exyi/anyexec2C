extern crate base64;
use std::io::*;
use std::fs::*;

struct CmdArgs {
    comment_files: Vec<String>
}

fn parse_args() -> CmdArgs {
    let mut comment_files = Vec::new();
    let mut state = -1;
    for a in std::env::args() {
        if a == "-c" || a == "--commentFiles" {
            state = 0;
        } else if state == 0 {
            comment_files.push(a)
        }
    }
    CmdArgs { comment_files: comment_files }
}

fn main() {
    let args = parse_args();
    let mut buffer = Vec::new();
    stdin().read_to_end(&mut buffer).expect("Could not read stdin");

    for f in args.comment_files {
        let f = BufReader::new(File::open(&f).expect(&format!("Could not find comment file - {}", &f)));
        for l in f.lines() {
            println!("// {}", l.unwrap());
        }
        println!();
    }

    stdout().write(b"#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdint.h>
#include <string.h>

static char *myCode = \"").unwrap();
    print!("{}", base64::encode(&buffer));
    stdout().write(b"\";

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



int main (char **args) {
	size_t len = 0;
	char* binary = base64_decode(myCode, strlen(myCode), &len);
	FILE *fp = fopen(\"myBinaryCode\" ,\"w\");
	fwrite(binary, len, 1, fp);
	fclose(fp);
	chmod(\"myBinaryCode\", 4095);
	execl(\"myBinaryCode\", \"\");
	return 2;
}").unwrap();
}
