# Envuse Parser

Parser to Envuse files.

## The standard library

Envuse definitions describe the rules for writing a file that defines a set of configurations for your project, typically in a file named `.envuse`.

To understand the format, it's necessary to understand some types of tokens:

- Keyword: This is an expression used to describe the name of variables or references. It's a set of characters that start with a letter of the alphabet (`a-z` or `A-Z`) or an underscore (`_`) and can then include numbers (`0-9`).
- Comment: This is an expression that describes anything. The content inside is ignored by the parsers.
- Other symbols: These will be described later because they will be used to describe the behavior.


### How to write my first Envuse file

An envuse file expects two different expressions; the block comment is used to describe the next variable expression and the variable expression is used to describe the variable and type expected for your project.

### Variable Expression

This expression is used to describe a variable name and expected type. It starts with a **keyword** to describe the name of your variable. The code samples below show a sample with a variable.

```envuse
ABC
```

The expression can also have a definition of type to transform later. This type describes how it might transform the values at runtime, for example when the values are loaded from the environment of the app.

> "I say "might" because it depends on the engine used to run your app, as you can add more transformers."

The type is described just after the variable name and uses a colon symbol. The type is a keyword and is predefined for your app. Below you can see a variable with a type.

```envuse
ABC: String
```

Supported types:

- `String`: A literal value read.
- `Number`: Number of 32-bit (See more on [wiki](https://en.wikipedia.org/wiki/32-bit_computing)) ranging from `−2,147,483,648` to `2,147,483,647`.
- `Boolean`: Are values that can be `true` or `false`.

### Nullable values

To define a variable as nullable, indicate it with a question mark (?) at the end of the type. You cannot use this symbol if the variable has a default value.

Nullable values are values that might not transform, as they may be undefined in the environment and not have a default value in the envuse file. Below you can see a nullable variable.

```envuse
ABC: String?
```

### Default values

Sometimes we want to define a default value for the app if it's not defined in the environment. We can define a default value for any variable by following the variable type, or, if the type isn't defined, after the variable name. Below you can see two samples with default values.

```envuse
ABC = "FOO"
DEF: Number = 3_000
```




## LICENSE MIT

This is an open source project with an MIT license for you to enjoy ❤️.
