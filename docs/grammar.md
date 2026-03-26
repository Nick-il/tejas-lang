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
  > All end with `=`

    * Arithmetic Assignment : `+=` `-=` `*=` `/=`
    * Equality : `==` `!=`
    * Comparison : `<=` `>=`
    * Walrus : `:=` - To infer types during variable declarations
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

```ebnf
args        := expression ( "," expression )* ;
lvalue      := IDENTIFIER ( "." IDENTIFIER )* ;

expression  := assignment ;
assignment  := lvalue ( "=" | "+=" | "-=" | "*=" | "/=" ) assignment
               | conditional ;
conditional := logic_or ( "if" logic_or "else" conditional )? ;
logic_or    := logic_and  ( "or"  logic_and  )* ;
logic_and   := equality   ( "and" equality   )* ;
equality    := comparison (( "==" | "!=" ) comparison )* ;
comparison  := term ( (">=" | ">" | "<" | "<=") term )+
               | term ;
term        := exponent (( "+" | "-" ) exponent )* ;
exponent    := factor ( "^" exponent )? ;    # right-associative
factor      := unary (( "*" | "/" | "%" ) unary )* ;
unary       := ( "+" | "-" | "not" ) unary | call ;
call        := primary (( "(" args? ")" ) | ( "." IDENTIFIER ))* ;
primary     := IDENTIFIER | STRING | INT | FLOAT
            | "true" | "false"
            | ( "(" expression ")" ) ;
```

### Notes on Expression Grammar:

* **Conditional Expression (Python-style)** — `true_val "if" cond_expr "else" false_val`
    * Right-associative: `a if b else c if d else e` = `a if b else (c if d else e)`
    * Example: `10 if x > 5 else 20` evaluates to `10` if `x > 5`, otherwise `20`
    * Example: `"pos" if x > 0 else "zero" if x == 0 else "neg"` for chained conditions
* **Exponentiation** (`^`) is Right-associative: `2 ^ 3 ^ 2` = `2 ^ (3 ^ 2)` = `512`
* **Chained Comparisons:** Multiple comparison operators can be chained and are desugared to logical AND.
    * `a < b < c` desugars to `(a < b) and (b < c)`
    * `0 < x <= 10` desugars to `(0 < x) and (x <= 10)`
    * The middle value is evaluated only once and reused

## Statements Grammar

```ebnf
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

expr_stmt       :=  expression ";" ;
print_stmt      :=  "print" expression ";" ;
Block           :=  "{" declaration* "}" ;

if_stmt         :=  "if" expression Block ( "else" Block )? ;
while_stmt      :=  "while" expression Block ;
for_stmt        :=  "for" ( var_decl | expr_stmt | ";" )
                        expression? ";"
                        expression?
                     Block ;

try_stmt        :=  "try" Block ("catch" IDENTIFIER? Block)+ ;

break_stmt      :=  "break" ";" ;
continue_stmt   :=  "continue" ";" ;
return_stmt     :=  "return" expression? ";" ;
```

### Statement Notes:

* All control flow statements (`if`, `while`, `for`, `try-catch`) require explicit `{ }` blocks.
* The `for` loop follows Go-style syntax: `for (init; condition; increment) { ... }`
* `try-catch` can have multiple catch blocks.
* If `IDENTIFIER` is provided in catch, it binds the exception object; otherwise it is discarded.

## Declaration Grammar

```ebnf
# Utility
TYPE                     :=  "int" | "float" | "string" | "bool" ;
DECL                     :=  "var" | "fix" | "const" ;

params                   :=  IDENTIFIER ":" TYPE ( "," IDENTIFIER ":" TYPE )* ;

# Declaration Grammar
declaration     :=  var_decl  |
                    func_decl |
                    statement ;

var_decl        :=  DECL IDENTIFIER ":" TYPE "=" expression ";" |
                    DECL IDENTIFIER ":=" expression ";" ;

func_decl       :=  "func" IDENTIFIER "(" params? ")" ( "->" TYPE )? Block ;
```

* Variable Declaration:
    * `DECL` specifies the type of variable declaration:
        * `var` : mutable variable (runtime)
            * `var x: int = 5;` — explicit type
            * `var x := 5;` — type inference
        * `fix` : immutable variable (runtime)
            * `fix x: int = 5;` — explicit type
            * `fix x := 5;` — type inference
        * `const` : compile-time constant
            * `const x: int = 5;` — explicit type
            * `const x := 5;` — type inference
            * Value is substituted at compile-time; no runtime storage
            * Cannot be reassigned (immutable)
    * **All variables must be initialized immediately.** No uninitialized variables.
    * Explicit type and inference are mutually exclusive:
        * Use `: TYPE =` for explicit typing with initialization
        * Use `:=` for type inference with initialization

* Function Declaration:
    * Type annotations on parameters are **mandatory**.
    * Return type is optional; if omitted, the function is void.
    * Function body must be a `Block` (braced statements).
    * Declaration syntax: `func name(param: Type, ...) -> ReturnType { ... }`

## Import Grammar

* `bring` : bring in a module from a file into the AST.
* `use` : use (import) a symbol from a brought module.
* `give` : explicitly export a symbol from the current module for external use (privacy control).

```ebnf
# Utility
alias               :=  "as" IDENTIFIER ;

terminal_symbol     :=  ( IDENTIFIER alias? ) | "*" ;
symbol              :=  ( IDENTIFIER "::" )* terminal_symbol ;
symbols             :=  symbol ( "," symbol )* ;
symbol_path         :=  ( IDENTIFIER "::" )+ ( ( "{" symbols? "}" ) | symbol ) ;

# Import/Export Grammar
bring_decl          :=  "bring" STRING alias? ";" ;
use_stmt            :=  "use" symbol_path ";" ;
give_stmt           :=  "give" IDENTIFIER ";" ;
give_decl           :=  "give" var_decl |
                        "give" func_decl ;

declaration         :=  give_decl  |
                        var_decl   |
                        func_decl  |
                        give_stmt  |
                        statement ;
```

### Import/Export Notes:

* `bring` loads a file into the AST. Multiple `bring` declarations can be used.
* `use` imports symbols from a brought module into the current namespace.
* `give` **explicitly marks symbols as public/exported** for external modules to use.
    * Can be used in two ways:
        1. **Attached to declaration:** `give func name() { ... }` or `give const X := value;` — marks the symbol as public at declaration time.
        2. **Standalone statement:** `give IDENTIFIER;` — marks a previously declared symbol as public.
    * Controls module privacy: symbols are **private by default**; `give` makes them public.
    * **Semantic constraint:** A symbol can be exported via `give` **at most once** per module. Attempting to export a symbol that is already exported is a compile error.
        * Example (error): `give func add() { } give add;` — illegal, add is already given
* Shorthand for importing multiple symbols: `use module::{symbol1, symbol2};` desugars to `use module::symbol1; use module::symbol2;`
* Circular imports are prohibited (detected at parse/compile time).

### Privacy Semantics:

* **Default (Private):** All declarations (`var`, `fix`, `const`, `func`) are private to their module.
* **Public (Exported):** Only symbols marked with `give` are accessible from other modules.
* **Example 1: Attached to declaration**

    **File: `math.tej`**
    ```
    give func add(a: int, b: int) -> int { return a + b; }
    give func multiply(a: int, b: int) -> int { return a * b; }
    give const PI := 3.14159;

    const PRIVATE_MAGIC := 42;  # Not exported
    func privateHelper() { ... }
    ```

* **Example 2: Standalone statement**

    **File: `math.tej`**
    ```
    func add(a: int, b: int) -> int { return a + b; }
    func multiply(a: int, b: int) -> int { return a * b; }
    const PI := 3.14159;
    const PRIVATE_MAGIC := 42;

    give add;
    give multiply;
    give PI;
    # PRIVATE_MAGIC is not given, so it stays private
    ```

* **Example 3: Usage in another module**

    **File: `main.tej`**
    ```
    bring "math.tej" as math;
    use math::{add, multiply, PI};

    print add(2, 3);           # OK
    print multiply(4, 5);      # OK
    print PI;                  # OK
    print math::PRIVATE_MAGIC; # ERROR - not exported
    ```

## Program Grammar

```ebnf
program        :=  (bring_decl | use_stmt)* (give_stmt | give_decl | declaration)* EOF ;
```

The program consists of:
1. Module imports (`bring`) and symbol imports (`use`) — can appear in any order, at the top
2. Declarations, statements, and exports — can appear in any order
   * `var_decl` — variable declaration
   * `func_decl` — function declaration
   * `give_stmt` — export an existing symbol
   * `give_decl` — declare and export a symbol in one statement
   * `statement` — executable statements
3. End of file