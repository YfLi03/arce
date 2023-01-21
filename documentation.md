## design

## database tables

## modules

### model

functions in `model` serve as an interface to operate the sqlite database.

A single `init` is provided by `model.rs`, and relative operating / querying funcs are provided by corresponding files. 

### notifier

monitor the folders 

it starts a thread for every single folder

picture folders are monitored recursively, while article folders are not.

## templates

