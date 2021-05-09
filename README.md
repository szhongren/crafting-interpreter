# context-free grammars

Rules for the lexical grammar was a regular language, but that can't handle expressions which can nest arbitrarily deeply.

Previously, our 'alphabet' consists of chars, and the 'strings' are valid lexemes/tokens.

Now, we need a context-free grammar, which is at a higher level of abstraction, where each 'letter' is a token, and a 'string' is a sequence of tokens: an expression.

# rules for grammars

If you start with the rules, you can use them to generate strings that are in the grammar.
Strings created this way are called derivations because each is derived from the rules of the grammar.
In each step of the game, you pick a rule and follow what it tells you to do.
Most of the lingo around formal grammars comes from playing them in this direction.
Rules are called productions because they produce strings in the grammar.

Each production in a context-free grammar has a head—its name—and a body, which describes what it generates.
In its pure form, the body is simply a list of symbols.

Symbols come in two delectable flavors:

- terminal is a letter from the grammar’s alphabet.
  You can think of it like a literal value.
  In the syntactic grammar we’re defining, the terminals are individual lexemes—tokens coming from the scanner like if or 1234.
  These are called “terminals”, in the sense of an “end point” because they don’t lead to any further “moves” in the game.
  You simply produce that one symbol.

- nonTerminal is a named reference to another rule in the grammar.
  It means “play that rule and insert whatever it produces here”.
  In this way, the grammar composes.

# example grammar for breakfast

```
breakfast → protein "with" breakfast "on the side" ;
breakfast → protein ;
breakfast → bread ;

protein → crispiness "crispy" "bacon" ;
protein → "sausage" ;
protein → cooked "eggs" ;

crispiness → "really" ;
crispiness → "really" crispiness ;

cooked → "scrambled" ;
cooked → "poached" ;
cooked → "fried" ;

bread → "toast" ;
bread → "biscuits" ;
bread → "English muffin" ;
```

after condensation

```
breakfast → protein ( "with" breakfast "on the side" )?
          | bread ;

protein   → "really"+ "crispy" "bacon"
          | "sausage"
          | ( "scrambled" | "poached" | "fried" ) "eggs" ;

bread     → "toast" | "biscuits" | "English muffin" ;
```

# example grammar for lox

```
expression → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
```
