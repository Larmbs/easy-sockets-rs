# Project TODOs


## Features
- Error Code Enum:
    An enum that can be serialized into an integer and can be deserialized from one. Also there will be a way to annotate each error code number with a special error code message that describes the error code in detail.
- Allowing Server Instance To Send Server Command:
    This idea is from the Easy ESP project. The handle_message function in the server conn trait should return an Option<ServerMsg> instead. This allows the server to be externally controlled and execute things like updating shared data, shutting down the server, and sending all clients a message.



