# Regular expression

## External specification

### Position specifier

|Expression|Semantic|
|-|-|
|`^`|Beginning of line|
|`$`|End of line|

### Specific character specifier

|Expression|Semantic|
|-|-|
|`\$`|`$`|
|`\(`|`(`|
|`\)`|`)`|
|`\*`|`*`|
|`\+`|`+`|
|`\-`|`-`|
|`\.`|`.`|
|`\/`|`/`|
|`\?`|`?`|
|`\[`|`[`|
|`\\`|`\`|
|`\]`|`]`|
|`\^`|`^`|
|`\f`|`f`|
|`\n`|`n`|
|`\r`|`r`|
|`\t`|`t`|
|`\xXX`|An arbitrary byte of 2-digit hexadecimal|
|`\{`|`{`|
|`\\|`|`\|`|
|`\}`|`}`|

### Arbitrary character specifier

|Expression|Semantic|
|-|-|
|`[S]`|An arbitrary character of character set `S`|
|`[^S]`|An arbitrary character of complement set of `S`|
|`.`|`[^]`|
|`\s`|`[ \f\n\r\t]`|
|`\S`|`[^\s]`|
|`\d`|`[0-9]`|
|`\D`|`[^\d]`|
|`\l`|`[a-z]`|
|`\L`|`[^\l]`|
|`\u`|`[A-Z]`|
|`\U`|`[^\u]`|
|`\w`|`[\d\l\u_]`|
|`\W`|`[^w]`|

#### Character set `S`

|Expression|Semantic|
|-|-|
|`a`|Character set `{a}`|
|`a-b`|Characters from `a` to `b`|
|`AB`|Union of character set `A` and `B`|

### Repeater

|Expression|Semantic|
|-|-|
|`B{min,max}`|Limited repeat of block `B`|
|`B{min,}`|Lower bounded repeat of block `B`|
|`B{num}`|`B{num,num}`|
|`B*`|`B{0,}`|
|`B+`|`B{1,}`|
|`B?`|`B{0,1}`|

### Selection

|Expression|Semantic|
|-|-|
|`A\|B`|Selection of block `A` or `B`|

## Internal specification

```
expression = term{'|'term};
term = {power};
power = base[exponent];
base = character + '['['^']set']' + '('expression')';
character = '^' + '$' + '.' + '\'escaped + unescaped;
escaped = '$' + '(' + ')' + '*' + '+' + '.' + '/' + '?' + 'D' + 'L' + 'S' + 'U' + 'W' + '[' + '\' + ']' + '^' + 'd' + 'f' + 'l' + 'n' + 'r' + 's' + 't' + 'u' + 'w' + 'x'byte + '{' + '|' + '}';
byte = hexadecimal hexadecimal;
hexadecimal = digit + 'a' + 'b' + 'c' + 'd' + 'e' + 'f' + 'A' + 'B' + 'C' + 'D' + 'E' + 'F';
unescaped = digit + lowercase + uppercase + symbol;
digit = '0' + '1' + '2' + '3' + '4' + '5' + '6' + '7' + '8' + '9';
lowercase = 'a' + 'b' + 'c' + 'd' + 'e' + 'f' + 'g' + 'h' + 'i' + 'j' + 'k' + 'l' + 'm' + 'n' + 'o' + 'p' + 'q' + 'r' + 's' + 't' + 'u' + 'v' + 'w' + 'x' + 'y' + 'z';
uppercase = 'A' + 'B' + 'C' + 'D' + 'E' + 'F' + 'G' + 'H' + 'I' + 'J' + 'K' + 'L' + 'M' + 'N' + 'O' + 'P' + 'Q' + 'R' + 'S' + 'T' + 'U' + 'V' + 'W' + 'X' + 'Y' + 'Z';
symbol = "'" + '!' + '"' + '#' + '%' + '&' + ',' + '-' + ':' + ';' + '<' + '=' + '>' + '@' + '_' + '`' + '~';
set = {range};
range = element['-'element];
element = digit + lowercase + uppercase + '\'element_escaped + element_symbol;
element_escaped = '$' + '(' + ')' + '*' + '+' + '-' + '.' + '/' + '?' + 'D' + 'L' + 'S' + 'U' + 'W' + '[' + '\' + ']' + '^' + 'd' + 'f' + 'l' + 'n' + 'r' + 's' + 't' + 'u' + 'w' + 'x'byte + '{' + '|' + '}';
element_symbol = "'" + '!' + '"' + '#' + '%' + '&' + ',' + ':' + ';' + '<' + '=' + '>' + '@' + '_' + '`' + '~';
exponent = '*' + '+' + '?' + '{'number[,[number]]'}';
number = {digit};
```
