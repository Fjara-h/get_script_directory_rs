# get_script_directory_rs
Prints a script's parent directory to STDOUT

This is my first real Rust project, I have absolutely no doubt that I've done unsafe or ill-advised code. Basically, I hope it works, but if it doesn't, then let me know in a PR or issue. Same goes for improvements. 

This is intended to replace a ba(sh) script using `SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )` although caveats exist for all shell variants detailed [on stackoverflow.](https://stackoverflow.com/questions/29832037/how-to-get-script-directory-in-posix-sh/29835459#29835459)
