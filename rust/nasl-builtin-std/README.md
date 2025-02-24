# nasl-builtin-std

Contains functions that are within the std library of nasl.

To use the std functions it is recommended to use the defined [ContextBuilder] as it sets the function register to the one created in [nasl_std_functions] automatically.

All you have todo as a user is to create the builder

```
let cb = nasl_builtin_std::ContextBuilder::default();
```

and set all but the functions. This will use the DefaultDispatcher as well as an empty String as a key.

For production use cases it is recommended to use new method and include a key and a storage:

```
let key = "test:localhost".to_string();
let cb = nasl_builtin_std::ContextBuilder::new(key, Box::new(storage::DefaultDispatcher::default()));
```

## Add functions to std

To add a function to std you have to add function crate to the Cargo.toml

```toml
[dependencies]
nasl-builtin-string = {path = "../nasl-builtin-string"}
```

and then extend the builder within [nasl_std_functions] with the implementation of [nasl_builtin_utils::NaslFunctionExecuter] of those functions:

```text
builder = builder.push_register(nasl_builtin_string::NaslString)
```

### Mark as experimental

When you have to mark your functions as experimental than you have to declare that crate as optional and add it to the experimental feature.


```toml

nasl-builtin-ssh = {path = "../nasl-builtin-ssh", optional = true}

[features]
experimental = ["nasl-builtin-ssh"]
```

Afterwards you need to create two methods. One for when the library is not included and one for when it is.

```
#[cfg(not(feature = "nasl-builtin-ssh"))]
fn add_ssh<K: AsRef<str>>(
    builder: nasl_builtin_utils::NaslfunctionRegisterBuilder<K>,
) -> nasl_builtin_utils::NaslfunctionRegisterBuilder<K> {
    builder
}

#[cfg(feature = "nasl-builtin-ssh")]
fn add_ssh<K: AsRef<str>>(
    builder: nasl_builtin_utils::NaslfunctionRegisterBuilder<K>,
) -> nasl_builtin_utils::NaslfunctionRegisterBuilder<K> {
    builder.push_register(nasl_builtin_ssh::Ssh::default())
}

```

It is recommended to toggle on the crate name and not on experimental to also enable toggling those without using experimental.


Afterwards you call the created function to add it within the builder of [nasl_std_functions]


```text
    builder = add_ssh(builder);
```
