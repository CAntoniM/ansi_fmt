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

Below is a series of examples that show off the intended usecases of ansi_fmt

### inline file processing

this is indended to be able for you to be able to remove to put in line as part 
of a build command to strip and escape seqeunces from them for use in build logs

```bash
    make VERBOSE=1 | ansi_fmt --format html --ouput build_log.html
```

or

```bash
    make VERBOSE=1 | ansi_fmt --ouput build.log
```


### batch file processing 

This is where you have a serires of logs that you wish to remove the escape
sequences from to make it easier to read outside of the terminal.

```bash
    ansi_fmt --format html --ouput build_log.html test1.log test2.log ...
```

or

```bash
    ansi_fmt --ouput build.log test1.log test2.log ...
```



## The Theory

So what are ANSI Control codes? these are a set of formatting instructions that
have more been evolved into rather than actually designed. but they generally
follow the following sturcture:

```ebnf
digit = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0"
int = { digit }
args = [int],{[";"],[int]}
command_identifier = 'm'| 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'J' | 'K' | 'S' | 'T' | 'f' | 'i' | 'n' | 's' | 'u'
control_sequence = args, command_identifier
fe_escape_sequence = fe_identifier "N" | "O" | "P" | "[",control_sequence | "X" | "^" | "_" | "\\"
esc = "\e"
letter = "A" | "B" | "C" | "D" | "E" | "F" | "G"
        | "H" | "I" | "J" | "K" | "L" | "M" | "N"
        | "O" | "P" | "Q" | "R" | "S" | "T" | "U"
        | "V" | "W" | "X" | "Y" | "Z" | "a" | "b"
        | "c" | "d" | "e" | "f" | "g" | "h" | "i"
        | "j" | "k" | "l" | "m" | "n" | "o" | "p"
        | "q" | "r" | "s" | "t" | "u" | "v" | "w"
        | "x" | "y" | "z" ;

digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;

symbol = "[" | "]" | "{" | "}" | "(" | ")" | "<" | ">"
        | "'" | '"' | "=" | "|" | "." | "," | ";" | "-"
        | "+" | "*" | "?" | "\n" | "\t" | "\r" | "\f" | "\b" ;

character = letter | digit | symbol | "_" | " " ;
string = {character}
esc_sequence = esc,fe_escape_sequence
text = {string | esc_sequence}
```

where the Escape can be any of the following possible valid values

- ```^[```
- ```\033```
- ```\u001b```
- ```\x1B```
- ```27``` - This reffers to the actual byte value.

and the Command values are any of the ones described in the following docs found
[here](https://en.wikipedia.org/wiki/ANSI_escape_code). For this project we only
really give a damn about preserving the format codes as while the other control
codes make alot of sence as part of a terminal setting outside that they do not
proide much so can be removed. as such the supported stlyings will be:

- Reset
- Bold
- Faint
- Italic
- Underline
- Slow Blink
- Rapid Blink
- Invert
- Hide
- Strike
- PrimaryFont
- AlternatFont
- Fraktur
- DoubleUnderline
- ResetIntensity
- RestItlaics
- ResetUnderline
- ResetBlinking
- RestReversed
- RestConcealed
- ResetStrike
- ForgroundColor
- 8 bit ForgroundColor
- 24bit ForgroundColor
- ResetForground
- 8 bit BackgroundColor
- 24 bit BackgroundColor
- BackgrounColor
- RestetBackground
- Framed
- Encircled
- Overlined
- ResetFramedAndEncircled
- NotOverlined
- SetUnderlineColor
- DefaultUnderline
- IdeogramUnderline
- IdeogramDoubleUnderline
- IdeogramOverline
- IdeogramDoubleOverline
- IdeogramStressMarking
- ResetIdeogram
- Subscript
- Superscript
- NetherSuborSuperscript
- SetBrightForeground
- SetBrightBackground
