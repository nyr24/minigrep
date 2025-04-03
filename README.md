# **Minigrep**

This is a simple program for searching text in a file system\
Famous [grep](https://man7.org/linux/man-pages/man1/grep.1.html) command was taken as inspiration

It can search files recursively, output line numbers, output whole result into a file you provided, etc.\
All utf-8 codepoints are supported, you can search for Chinese or Cyrillic characters freely

Example of usage:
```
minigrep [options] -s $pattern -p $filepath [-f $output_to_file_path] [-e .git,.png,.exe]
```
Options can be:
- -h -- provide information about usage of the program
- -q -- make program quiet, error logs would not be displayed, highly recommended
- -d -- search a directory starting from $filepath (by default program expect a file)
- -r -- do recursive search starting from $filepath
- -i -- ignore case in $pattern and occurences
- -n -- output line numbers

You can also include options that accept an argument:
- -f -- write all program output to the file, instead of stdin:\n-f $output_to_file_path
- -e -- exclude searching from paths which contain patterns:\n-e .git,.png,.exe
