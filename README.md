# ANSI_FMT

ANSI_FMT (ansi format) is a project desiged to allow you to strip all of the
annoying ANSI formatting characters (color codes, bold, itallics ect) and 
replace them with a diffrent format output. 

## The Problem
Shell commands want to add pretty outputs to there programme however the only 
way they can do this that is portable is via ANSI Escape sequences. the issue 
with this is when using these in scripts or in logs they lead to text that was
there being made very hard to read. 

#### Why not just use the flags such as --no-color?

The issues with this are multiple:

1. You have to apply these flags on a programe by programe basis as they all
use their own with no stanard in place
2. You loose all formatting infomation from document which can convay useful 
infomation at a glance e.g. warnings are yello, errors are red.
3. You do not allways have complete controll over the flags used during these
processes. e.g. that old CI system that was built before this was mainstream.

## The solution

Write a programe that will read in this infomation and put it into a fomrat that
is more readable/understandable. This can be done by simply stripping them or
via a conversion to another format e.g. html. By doing this either inline via a
pipe or after fact by reading a file it will allow you to either capture more 
readable logs or process existing logs to make them easier to read.

## Examples
