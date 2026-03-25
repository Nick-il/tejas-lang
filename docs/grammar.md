# Grammar

* [What do the symbols mean?](#what-do-the-symbols-mean)
* [Lexical Grammar](#lexical-grammar)
* [Expression Grammar](#expression-grammar)
* [Statement Grammar](#statements-grammar)
* [Declaration Grammar](#declaration-grammar)
* [Program Grammar](#program-grammar)

## What do the symbols mean?

|   Symbol | Meaning                                                                         |
|---------:|:--------------------------------------------------------------------------------|
| `A := B` | `A` is defined as `B`                                                           |
| `"word"` | A literal lexeme                                                                |
|    `(_)` | for grouping                                                                    |
|    `<_>` | for conveying easily understood rules in English. Eg: `<all stmt except Block>` |
| `A \| B` | Either A or B                                                                   |
|     `A?` | `A` is Optional, (i.e. 0 or 1 times)                                            |
|     `A+` | Repeat `A` 1 or more times                                                      |
|     `A*` | Repeat `A` 0 or more times                                                      |
|      `;` | Used to terminate grammar rules. (not language semicolons)                      |
|      `#` | Single-line comment for the grammar. (not specifically for the language)        |

## Lexical Grammar

Lexical grammar is used by lexer while lexing the tokens. Its Main types are :

* Single Char Tokens
    * Brackets : `(` `)` `{` `}` `[` `]`
    * Arithmetic Operators : `+` `-` `*` `/` `%` `^`
    * Comparison : `<` `>`
    * Other Punctuators : `=` `,` `.` `:` `;` `?`

* Double Char Tokens
  > All except `=>` end with `=`

    * Arithmetic Assignment : `+=` `-=` `*=` `/=`
    * Equality : `==` `!=`
    * Comparison : `<=` `>=`
    * Walrus : `:=` - To infer types during variable declarations
    * Then Alias : `=>` - used as shorthand for single statement blocks.
    * RArrow : `->` - for func return type.
* Numbers
    * Int
        * stored as `isize`
    * Float
        * stored as `f64`
    ```ebnf
    INT      := ('0'..='9')+ ;
    FLOAT    := ('0'..='9')+ "." ('0'..='9')+ ;
    ```
* Identifiers
    * Aim is to make it Unicode aware so that even `பெயர்` or `नाम` can be an identifier.
    ```ebnf
    IDENTIFIER  := (UnicodeLetter | '_') (UnicodeLetter | '_' | 0..=9)* ;
    ```
* Keywords
    * Values : `true` `false`
    * Declarations
        * `var` `fix` `const`
        * `func`
    * Control Flow
        * `if` `else`
        * `for` `while` `break` `continue`
        * `return`
    * Logical Ops :     `and` `or` `not`
    * `print`
    * Error Handling : `try` `catch`
    * Module Handling : `bring` `use` `give`
* Strings
    * Delimiters : `"` or `'`
    * Prefixes
        * `f` - `f"Hello {name}"` for interpolation. Currently, does nothing, as no interpolation yet.
        * `r` - `r"path\to\file.ext"` for raw-strings.
        * `fr` or `rf` - everything will be raw, except the `{` `}` used for interpolation. It means `fr"x is \'{x}\'"`
          with `fix x := 1` will result in the output of `x is \'1\'` not `x is '1'`
    * Escaping
        * Supports `\n` `\t` `\r` `\\` `\"` `\'`
        * escape for Unicode and others will be added later.
    * Multi-line
        * Multiline strings are unified with single-line ones as there is no grammar requirement for separate syntax.
* Comments
    * Single-line: from `#` to the end of line
    * Multi-line: from `#[` to `]#`
    * Docs : (T.B.D.)

## Expression Grammar

Lowest priority to Highest Priority while moving down.
Dot Access and Indexing are pending.

```ebnf
args         := expression ( "," expression )* ;

expression   := assignment ;
assignment   := logic_or ( "=" logic_or )? ;
logic_or     := logic_and ( "or" logic_and )* ;
logic_and    := equality ( "and" equality )* ;
equality     := comparison ( ("==" | "!=") comparison )* ;
comparison   := term ( (">=" | ">" | "<" | "<=") term )* ;
term         := factor ( ("+" | "-") factor )* ;
factor       := unary ( ("*" | "/" | "%") unary )* ;
unary        := ( "+" | "-" | "not" ) unary | call ;
call         := primary (
                    ( "(" args? ")"      ) |
                    ( "." IDENTIFIER     ) |
                    ( "[" expression "]" )
                )* ;
primary      := IDENTIFIER | STRING     |
                INT        | FLOAT      |
                "true"     | "false"    |
                ( "(" expression ")" )  ;

```

## Statements Grammar

```ebnf'
# Utility
block_alias     :=  Block | ("=>" single_stmt) ;
single_stmt     :=  <statement that isnt Block> ;

# Statement Grammar
statement       :=  expr_stmt     |
                    print_stmt    |
                    Block         |
                    if_stmt       |
                    while_stmt    |
                    for_stmt      |
                    try_stmt      |
                    break_stmt    |
                    continue_stmt |
                    return_stmt   ;

expr_stmt       :=  expression ";"
print_stmt      :=  "print" expression ";"
Block           :=  "{" declarations* "}"


if_stmt         :=  "if" expression block_alias ( "else" block_alias )?

while_stmt      :=  "while" expression block_alias;
for_stmt        :=  "for"
                        ( varDecl | exprStmt | ";" )
                        expression? ";"
                        expression?
                     block_alias ; # go-style

try_stmt        :=  "try" block_alias ("catch" IDENTIFIER? block_alias)+ ;

break_stmt      :=  "break" ";" ;
continue_stmt   :=  "continue" ";" ;
return_stmt     :=  "return" expression? ";" ;
```

## Declaration Grammar

```ebnf
# Utility
TYPE                     :=  "int" | "float" | "string" | "bool" ;
DECL                     :=  "var" | "fix" | "const";

params                   :=  ( IDENTIFIER ":" TYPE ) ( "," ( IDENTIFIER ":" TYPE ) )*;


# Declaration Grammer
declaration     :=  var_decl  |
                    func_decl |
                    statement;  # maybe will add class/struct decl later

var_decl        :=  DECL IDENTIFIER ( (":" TYPE "=") | ":=" ) expression ";" ;

func_decl       :=  "func" IDENTIFIER "(" params? ")" ( "->" TYPE )? block_alias;


```

* Variable Declaration :
    * `DECL` is the type of variable declaration it can be :-
        * `var` : declaration of mutable variable, (runtime variable)
            * `var xyz = 1`
        * `fix` : declaration of immutable variable, (runtime variable)
            * `fix xyz = 1`
        * `const` (compile-time variable)
            * `const xyz = 1`
            * Compile time constant, i.e. resolvable at compile time.
            * Its value is substituted at compile-time.
            * No runtime footprint.
            * Const can also be made of `const` which will be folded also at compile-time. just no function calls for
              now. (cuz it'll pollute the state)

* Function Declaration
    * declaration for both functions and methods.
    * Specifying the type of each param is mandatory.
    * If the return type is empty, it will be considered a void function.

## Import Grammar

* `bring` : bring in a module from a file.
* `use` : use a symbol from a module.
* `give` : give a symbol to external module. (used for exporting)

```bnf
# Utility
alias               :=  "as" IDENTIFIER

terminal_symbol     :=  ( IDENTIFIER alias? ) | "*"
symbol              :=  ( IDENTIFIER "::" )* terminal_symbol ;
symbols             :=  symbol ( "," symbol )*
symbol_path         :=  ( IDENTIFIER "::" )+ ( ( "{" symbols? "}" ) | symbol )

bring_decl          :=  "bring" STRING alias? ";" ;
use_stmt            :=  "use" symbol_path ";" ;
give_stmt           :=  "give" ( var_decl | func_decl | (  symbol_path ";" ) ) ;
```

## Program Grammar

I think it's self-explanatory.

```ebnf
bring_block    :=  bring_decl* ( use_stmt | give stmt )* ;
program        :=  bring_block declarations* EOF;
```