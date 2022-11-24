# Envuse Parser

Parser to Envuse files.

## The standard library

The Envuse definitions describe the rules to write a file and than describe a set of configurations for your project. Into a file typically named `.envuse`.

To understand the format is necessary to understand some tokens kind:

- Keyword: This Is an expression used to describe the name of variables or references. Is a set of characters. Than start with a letter of the alphabet (`a-z` or `A-Z`) or an underscore (`_`) then can use numbers (`0-9`).
- Comment: This is an expression to describe anything. The content inside is ignored by the parsers.
- Others symbols: To be described later because will be used to describe the behavior.


### How to write my first Envuse file

An envuse files expect two different expressions; The Block Comment is used to describe the next variable expression and the variable expression is used to describe the variable and type expected for your project.

### Variable Expression

This expression is used to describe a variable name and type expected. This start with a **keyword** to describe the name of your variable. The below code samples describe a sample with a variable.

```envuse
ABC
```

Also, the expression can have a definition of type to transform later. This type describes how it might transform the values in the run time, for example when the values are loaded from the environment of the app.

> I say "might" because depend of the engine used to run you app, since this you can add more transformers.

The type is described just after your variable name and using a colon symbol, the type is a **keyword** and is predefined for your app. Below you can see a variable with your type.

```envuse
ABC: String
```

Types supported:

- `String`: A literal value read.
- `Number`: Number of 32-bit (See more on [wiki](https://en.wikipedia.org/wiki/32-bit_computing)) of `−2,147,483,648` through `2,147,483,647`.
- `Boolean`: Are values that can be `true` or `false`.

### Nullable values

To define a variable as nullable is indicated through a question mark (`?`) at the end of the type and not can be using this symbol if the variable use a default value. 

Nullable values are values that might not transform since can be undefined in the environment and not have a default value in the envuse file. Below you can see a variable nullable.

```envuse
ABC: String?
```

### Default values

Sometimes we want to define a default value for the apps if is not defined for the environment. We can define a default value for any variable just follow of the variable type or if the type isn't defined after the variable name. Below you can see two samples with a default value.

```envuse
ABC = "FOO"
DEF: Number = 3_000
```


## LICENSE MIT

This is a project open source with an MIT license for you to enjoy ❤️.
