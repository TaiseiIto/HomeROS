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

## Internal specification
