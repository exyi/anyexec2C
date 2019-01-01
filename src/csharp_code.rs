pub const PART1: &str = "
using System;
using System.IO;
using System.Runtime.InteropServices;


namespace mujprogram {
    class Proram {
        const string binaryName = \"myBinaryPayload\";

        static void extract(string payload, string filename) {
            int len = 0;
            byte[] binary = Convert.FromBase64String(payload);
            File.WriteAllBytes(filename, binary);
            // does not hurt if everything has all permissions allowed
            chmod(filename, 511);
        }

        const string executable = \"%%EXECUTABLE%%\";
";
pub const MAIN: &str = "
        public static int Main(string[] args) {
            extract(executable, binaryName);

            %%ASSETS%%

            var args2 = new string[args.Length + 1];
            args2[0] = binaryName;
            Array.Copy(args, 0, args2, 1, args.Length);
            execv(binaryName, args2);
            return 2;
        }

        [DllImport(\"libc.so.6\")]

        public static extern int chmod(string message, int a);

        [DllImport(\"libc.so.6\")]
        public static extern int execv(string binary, string[] args);
    }
}
";
