# **Minigrep**

This is a simple program for searching text in a file system\
Famous [grep](https://man7.org/linux/man-pages/man1/grep.1.html) command was taken as inspiration

It can search files recursively, output line numbers, output whole result into a file you provided, etc.\
Example of usage:
```
minigrep [options] -s $pattern -p $filepath [-f $output_to_file_path]
```
Options can be:
- -h -- provide information about usage of the program
- -d -- search a directory starting from $filepath (by default program expect a file)
- -r -- do recursive search starting from $filepath
- -i -- ignore case in $pattern and occurences
- -n -- output line numbers

Program is written in safest language possible and virus free (~I hope~)\
So you should expect a safe experience when using it (~until it panics for some random reasons~)
